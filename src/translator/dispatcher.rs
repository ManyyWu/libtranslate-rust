use crate::Translation;
use crate::error::Error;
use crate::{
    Language,
    Result
};
use crate::api::{
    DetectorAPIContainer,
    TranslatorAPIContainer,
    DetectorAPI,
    TranslatorAPI,
    Request
};
use crate::api::google::google_translate::{
    API_MobileGoogleTranslate,
    API_GoogleTranslateExtensions,
    API_GoogleDictionaryChromeExtension
};

use std::convert::Into;
use std::collections::HashMap;
use std::sync::OnceLock;
use std::cmp::max;
use std::time::{
    Duration,
    Instant
};
use rand::{
    thread_rng,
    Rng
};

static DEFAULT_DETECTOR_SERVICE_LIST: OnceLock<HashMap<&'static str, (DetectorAPIContainer, u64)>> = OnceLock::new();
static DEFAULT_TRANSLATOR_SERVICE_LIST: OnceLock<HashMap<&'static str, (TranslatorAPIContainer, u64)>> = OnceLock::new();

pub(crate) trait DefaultAPI<T> {
    fn default_api() -> &'static HashMap<&'static str, (T, u64)>;
}

impl DefaultAPI<DetectorAPIContainer> for DetectorAPIContainer {
    fn default_api() -> &'static HashMap<&'static str, (DetectorAPIContainer, u64)> {
        DEFAULT_DETECTOR_SERVICE_LIST.get_or_init(|| {
            let mut map = HashMap::new();
            map.insert("google.API_GoogleDictionaryChromeExtension", ( API_GoogleTranslateExtensions {}.into(), 100_000u64 ));
            map.insert("google.API_GoogleTranslateExtensions", ( API_GoogleDictionaryChromeExtension {}.into(), 100_000u64 ));
            map
        })
    }
}


impl DefaultAPI<TranslatorAPIContainer> for TranslatorAPIContainer {
    fn default_api() -> &'static HashMap<&'static str, (TranslatorAPIContainer, u64)> {
        DEFAULT_TRANSLATOR_SERVICE_LIST.get_or_init(|| {
            let mut map = HashMap::new();
            map.insert("google.API_MobileGoogleTranslate", ( API_MobileGoogleTranslate {}.into(), 20_000u64 ));
            map.insert("google.API_GoogleDictionaryChromeExtension", ( API_GoogleTranslateExtensions {}.into(), 100_000u64 ));
            map.insert("google.API_GoogleTranslateExtensions", ( API_GoogleDictionaryChromeExtension {}.into(), 100_000u64 ));
            map
        })
    }
}

#[derive(Debug)]
pub(crate) enum ServiceStatus {
    Ready,
    Retry((u32, Instant)),
    Blocking(u32, Instant),
}

pub(crate) struct Service<T: 'static> {
    api: &'static T,
    status: ServiceStatus,
    last_error: Option<Error>,
    last_error_time: Option<Instant>,
    init_weight: u64,
    succ_req_times: u64,
    total_req_times: u64,
    consecutive_succ_req_times: u64,
}

impl<T> Service<T> {
    fn new(api: &'static T, weight: u64) -> Service<T> {
        Self {
            api: api,
            status: ServiceStatus::Ready,
            last_error: None,
            last_error_time: None,
            init_weight: weight,
            succ_req_times: 0,
            total_req_times: 0,
            consecutive_succ_req_times: 0,
        }
    }
}

pub(crate) struct Dispatcher<T: 'static> {
    registry: HashMap<String, Service<T>>,
}

impl<T: DefaultAPI<T>> Dispatcher<T> {
    pub(crate) fn new(apis: Vec<String>) -> Result<Self> {
        if 0 == apis.len() {
            return Err(Error::NoTranslatorRegistrationService);
        }

        let all_apis: &HashMap<&str, (T, u64)> = T::default_api();
        for name in &apis {
            if !all_apis.contains_key(&name.as_ref()) {
                return Err(Error::InvalidServiceName);
            }
        }

        let mut dispatcher = Dispatcher {
            registry: HashMap::new(),
        };

        for name in &apis {
            let api = all_apis.get(name.as_str()).unwrap();
            dispatcher.registry.insert(name.clone(), Service::new(&api.0, api.1));
        }

        Ok(dispatcher)
    }

    pub(crate) fn default() -> Result<Self> {
        let mut dispatcher = Dispatcher {
            registry: HashMap::new(),
        };

        for (name, api) in T::default_api() {
            dispatcher.registry.insert(name.to_string(), Service::new(&api.0, api.1));
        }

        Ok(dispatcher)
    }

    fn calc_weight(&self, service: &Service<T>) -> u64 {
        match service.status {
            ServiceStatus::Retry((_, next)) if Instant::now() < next => return 0,
            ServiceStatus::Blocking(_, end) if Instant::now() < end => return 0,
            _ => {}
        }
        if service.consecutive_succ_req_times > 3 {
            return service.init_weight;
        }
        if service.total_req_times <  100 {
            return service.init_weight;
        }
        let succ_rate = service.succ_req_times as f64 / service.total_req_times as f64;
        let weight = (succ_rate * service.init_weight as f64) as u64;
        max(service.init_weight, weight)
    }

    fn calc_max_delay(&self) -> Duration {
        let now = Instant::now();
        let mut max_delay = Duration::from_secs(0);
        self.registry.iter().for_each(|(_, v)| {
            let delay = match v.status {
                ServiceStatus::Retry((_, next)) if now < next => next - now,
                ServiceStatus::Blocking(_, end) if now < end => end - now,
                _ => Duration::from_secs(0),
            };
            max_delay = max(delay, max_delay)
        });
        max_delay
    }

    pub(crate) fn dispatch(&mut self, services: &HashMap<String, u64>) -> Result<(String, &mut Service<T>)> {
        let rand_service = |services: &HashMap<String, u64>, total_weight: i64| -> Option<String> {
            let mut rand_num = thread_rng().gen_range(1..=total_weight);
            let mut iter = services.iter();
            while let Some((name, weight)) = iter.next() {
                rand_num -= *weight as i64;
                if rand_num <= 0 {
                    return Some(name.to_string());
                }
            }
            None
        };

        let total_weight = services.iter().fold(0u64, |total, s| { total + s.1 }) as i64;
        if 0 == total_weight {
            return Err(Error::NoAvailableService(self.calc_max_delay()))
        }

        let name = rand_service(services, total_weight).unwrap();
        Ok((name.to_string(), self.registry.get_mut(&name).unwrap()))
    }

    pub(crate) fn handle_result<S, R>(service: &mut Service<S>, result: &Result<R>, begin_time: Instant)
        where R: std::fmt::Debug {

        match result {
            Ok(result) => {
                service.last_error = None;
                service.last_error_time = None;
                service.succ_req_times += 1;
                service.total_req_times += 1;
                service.consecutive_succ_req_times += 1;
                service.status = ServiceStatus::Ready;
            },
            Err(e) => {
                service.last_error = Some(e.clone());
                service.total_req_times += 1;
                service.consecutive_succ_req_times = 0;

                if service.last_error_time.is_none() {
                    service.last_error_time = Some(Instant::now());
                }
                if Instant::now() - service.last_error_time.unwrap() > Duration::from_secs(1) {
                    service.status = match service.status {
                        ServiceStatus::Ready => ServiceStatus::Retry((3, Instant::now() + Duration::from_millis(1_000))),
                        ServiceStatus::Retry((n, _)) if n > 1 => ServiceStatus::Retry((n - 1, Instant::now() + Duration::from_millis(1_000))),
                        ServiceStatus::Retry((n, _)) if n == 1 => ServiceStatus::Blocking(1, Instant::now() + Duration::from_millis(3_000)),
                        ServiceStatus::Blocking(1, _) => ServiceStatus::Blocking(2, Instant::now() + Duration::from_millis(60_000)),
                        ServiceStatus::Blocking(2, _) => ServiceStatus::Blocking(3, Instant::now() + Duration::from_millis(300_000)),
                        _ => ServiceStatus::Blocking(3, Instant::now() + Duration::from_millis(3600_000)),
                    };
                }
            },
        }
    }

    pub(crate) fn last_error(&self, api: &str) -> &Option<Error> {
        let Some(api) = self.registry.get(api) else {
            return &None;
        };
        &api.last_error
    }
}

impl<T: DefaultAPI<T> + DetectorAPI> Dispatcher<T> {
    pub(crate) async fn dispatch_detector(&mut self, request: &Request, text: &str) -> Result<Language> {
        let mut services: HashMap<String, u64> = HashMap::new();
        self.registry.iter().for_each(|(k, v)| { services.insert(k.to_string(), self.calc_weight(v)); });

        loop {
            let begin_time = Instant::now();
            let (name, service) = self.dispatch(&mut services)?;
            let result = service.api.language(request, text.as_ref()).await;
            Dispatcher::<T>::handle_result(service, &result, begin_time);
            if result.is_ok() {
                return result;
            };
            services.remove(&name);
        }
    }
}

impl<T: DefaultAPI<T> + TranslatorAPI> Dispatcher<T> {
    pub(crate) async fn dispatch_translator(&mut self, request: &Request, text: &str, source: Language, target: Language) -> Result<Translation> {
        let mut services: HashMap<String, u64> = HashMap::new();
        self.registry.iter().for_each(|(k, v)| { services.insert(k.to_string(), self.calc_weight(v)); });

        loop {
            let (name, service) = self.dispatch(&mut services)?;
            let begin_time = Instant::now();
            let result = service.api.translate(request, text.as_ref(), source, target).await;
            Dispatcher::<T>::handle_result(service, &result, begin_time);
            if result.is_ok() {
                return result;
            };
            services.remove(&name);
        }

    }
}

// TODO:
// Quotas and limits: https://cloud.google.com/translate/quotas?hl=zh-cn
// 1. The total number of google translate characters per minute does not exceed 6_000_000
// 2. The total times of google translate per minute does not exceed 60_000
// 3。Language quota checking is not currently supported
// 4. 429 => Too Many Requests, // Rate limit
//    401 => InvalidAPIKey,
//    402 => BlockedAPIKey,
//    404 => DailyLimitExceeded,
//    413 => MaxTextSizeExceeded,
//    422 => CouldNotTranslate,
//    501 => TranslationDirectionNotSupported,


#[cfg(test)]
mod tests {

    #[test]
    fn test_status_machine() {

    }
}
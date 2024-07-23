#![allow(dead_code)]

use crate::Result;
use crate::Error;
use crate::Language;
use crate::Translation;
use crate::translator::dispatcher::Dispatcher;
use crate::api::{DetectorAPIContainer, TranslatorAPIContainer, Request};

use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;

const DEFAULT_TIMEOUT_MILLIS: u64 = 30_000;

#[derive(Debug)]
pub enum Strategy {
    Default,
    Single(String),
    Mix(Vec<String>),
}

#[derive(Debug)]
pub enum TranslatorType {
    Text,
    NormalFile,
    SubTitleFile,
}

#[derive(Debug)]
pub(crate) struct Config {
    source: Language,
    target: Option<Language>,
    strategy: Strategy,
    timeout: Duration,
}

pub struct Detector {
    config: Config,
    request: Request,
    dispatcher: Dispatcher<DetectorAPIContainer>,
}

pub struct DetectorBuilder {
    config: Config,
}

impl DetectorBuilder {
    pub fn new() -> DetectorBuilder {
        DetectorBuilder::default()
    }

    pub fn default() -> DetectorBuilder {
        Self {
            config: Config {
                strategy: Strategy::Default,
                source: Language::Auto,
                target: None,
                timeout: Duration::from_millis(DEFAULT_TIMEOUT_MILLIS),
            }
        }
    }

    pub fn build(self) -> Result<Detector> {
        let dispatcher: Dispatcher<DetectorAPIContainer> = match &self.config.strategy {
            Strategy::Default => Dispatcher::default()?,
            Strategy::Single(name) => Dispatcher::new(vec![name.clone()])?,
            Strategy::Mix(names) => Dispatcher::new(names.clone())?,
        };

        let request = Request::new(self.config.timeout)?;

        Ok(Detector {
            config: self.config,
            request: request,
            dispatcher: dispatcher
        })
    }

    pub fn strategy(mut self, strategy: Strategy) -> DetectorBuilder {
        self.config.strategy = strategy;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> DetectorBuilder {
        self.config.timeout = timeout;
        self
    }
}

impl Detector {
    pub fn builder() -> DetectorBuilder {
        DetectorBuilder::new()
    }

    pub async fn language(&mut self, text: &str) -> Result<Language> {
        self.dispatcher.dispatch_detector(&self.request, text).await
    }
}

pub struct TranslatorBuilder {
    config: Config,
}

impl TranslatorBuilder {
    pub fn new() -> TranslatorBuilder {
        TranslatorBuilder::default()
    }

    fn default() -> TranslatorBuilder {
        Self {
            config: Config {
                strategy: Strategy::Default,
                source: Language::Auto,
                target: None,
                timeout: Duration::from_millis(DEFAULT_TIMEOUT_MILLIS),
            }
        }
    }

    pub fn build(self) -> Result<Translator> {
        let Some(target) = &self.config.target else {
            return Err(Error::NoTargetLanguage);
        };
        if *target == Language::Auto {
            return Err(Error::TargetLanguageIsAuto);
        }
        if *target == self.config.source {
            return Err(Error::TargetEqualToSource);
        }

        let dispatcher: Dispatcher<TranslatorAPIContainer> = match &self.config.strategy {
            Strategy::Default => Dispatcher::default()?,
            Strategy::Single(name) => Dispatcher::new(vec![name.clone()])?,
            Strategy::Mix(names) => Dispatcher::new(names.clone())?,
        };

        let request = Request::new(self.config.timeout)?;

        Ok(Translator {
            config: self.config,
            request: request,
            dispatcher: dispatcher
        })
    }

    pub fn from(mut self, source: Language) -> TranslatorBuilder {
        self.config.source = source;
        self
    }

    pub fn to(mut self, target: Language) -> TranslatorBuilder {
        self.config.target = Some(target);
        self
    }

    pub fn strategy(mut self, strategy: Strategy) -> TranslatorBuilder {
        self.config.strategy = strategy;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> TranslatorBuilder {
        self.config.timeout = timeout;
        self
    }
}

pub struct Translator {
    config: Config,
    request: Request,
    dispatcher: Dispatcher<TranslatorAPIContainer>,
}

impl Translator {
    pub fn builder() -> TranslatorBuilder {
        TranslatorBuilder::new()
    }

    pub async fn translate(&mut self, text: &str) -> Result<Translation> {
        self.dispatcher.dispatch_translator(&self.request, text, self.config.source, self.config.target.unwrap()).await
    }

    pub fn last_error(&self, api: &str) -> Option<Error> {
        self.dispatcher.last_error(api).clone()
    }
}

thread_local! {
    static DEFAULT_DETECTOR: Rc<RefCell<Detector>> = Rc::new(RefCell::new(Detector::builder().build().unwrap()));
    static DEFAULT_TRANSLATOR: Rc<RefCell<Translator>> = Rc::new(RefCell::new(Translator::builder().to(Language::English).build().unwrap()));
}

pub async fn translate(text: &str, source: Language, target: Language) -> Result<Translation> {
    let translator = DEFAULT_TRANSLATOR.with(|r| { r.clone() });
    let translator = &mut *(*translator).borrow_mut();
    translator.dispatcher.dispatch_translator(&translator.request, text, source, target).await
}

pub async fn language(text: &str) -> Result<Language> {
    let detector = DEFAULT_DETECTOR.with(|r| { r.clone() });
    let detector = &mut *(*detector).borrow_mut();
    detector.dispatcher.dispatch_detector(&detector.request, text).await
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_translator_builder() {
        use crate::Error;
        use crate::Language;
        use crate::Strategy;
        use crate::Translator;

        let _ = Translator::builder().build();
        
        assert!(matches!(
            Translator::builder()
            .strategy(Strategy::Default)
            .build(),
            Err(Error::NoTargetLanguage))
        );

        assert!(matches!(Translator::builder()
            .from(Language::English)
            .to(Language::English)
            .strategy(Strategy::Default)
            .build(),
            Err(Error::TargetEqualToSource))
        );

        assert!(matches!(Translator::builder()
            .from(Language::English)
            .to(Language::Auto)
            .strategy(Strategy::Default)
            .build(),
            Err(Error::TargetLanguageIsAuto))
        );

        assert!(matches!(
            Translator::builder()
            .from(Language::Auto)
            .to(Language::SimpleChinese)
            .strategy(Strategy::Mix(vec![]))
            .build(),
            Err(Error::NoTranslatorRegistrationService))
        );

        assert!(matches!(Translator::builder()
            .from(Language::Auto)
            .to(Language::SimpleChinese)
            .strategy(Strategy::Single(String::new()))
            .build(),
            Err(Error::InvalidServiceName))
        );
    }

    #[test]
    fn test_detector_builder() {
        use crate::Error;
        use crate::Strategy;
        use crate::Detector;

        let _ = Detector::builder().build();

        assert!(matches!(
            Detector::builder()
            .strategy(Strategy::Mix(vec![]))
            .build(),
            Err(Error::NoTranslatorRegistrationService))
        );

        assert!(matches!(Detector::builder()
            .strategy(Strategy::Single(String::new()))
            .build(),
            Err(Error::InvalidServiceName))
        );
    }
}
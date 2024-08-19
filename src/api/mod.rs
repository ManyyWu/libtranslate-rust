#![allow(dead_code)]

pub mod google;
pub mod azure;
pub mod custom;

use crate::Language;
use crate::Result;
use crate::Error;

use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;

#[derive(Debug)]
pub struct Translation {
    source: Option<Language>,
    target: Language,
    result: String,
}

#[async_trait]
pub trait DetectorAPI: Sync + Send {
    async fn language(&self, request: &Request, text: &str) -> Result<Language>;
}

#[derive(Clone)]
pub(crate) struct DetectorAPIContainer {
    inner: Arc<dyn DetectorAPI>
}

impl DetectorAPIContainer {
    pub(crate) fn new(inner: impl DetectorAPI + 'static) -> Self {
        Self { inner: Arc::new(inner) }
    }
}

#[async_trait]
impl DetectorAPI for DetectorAPIContainer {
    async fn language(&self, request: &Request, text: &str) -> Result<Language> {
        self.inner.language(request, text).await
    }
}

#[async_trait]
pub trait TranslatorAPI: Sync + Send {
    async fn translate(&self, request: &Request, text: &str, source: Language, target: Language) -> Result<Translation>;
}

pub(crate) struct TranslatorAPIContainer {
    inner: Arc<dyn TranslatorAPI>
}

impl TranslatorAPIContainer {
    pub(crate) fn new(inner: impl TranslatorAPI + 'static) -> Self {
        Self { inner: Arc::new(inner) }
    }
}

#[async_trait]
impl TranslatorAPI for TranslatorAPIContainer {
    async fn translate(&self, request: &Request, text: &str, source: Language, target: Language) -> Result<Translation> {
        self.inner.translate(request, text, source, target).await
    }
}
pub(crate) struct Request {
    client: reqwest::Client,
}

impl Request {
    pub(crate) fn new(timeout: Duration) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build();

        match client {
            Ok(client) => { Ok(Self { client }) },
            Err(e) => { return Err(Error::ReqwestError(e.to_string())); }
        }
    }

    pub(crate) async fn get(&self, url: &str) -> Result<String> {
        let rsp = self.client
            .get(url)
            .send()
            .await;
        match rsp {
            Ok(rsp) => {
                let 0..=399 = rsp.status().as_u16() else {
                    return Err(Error::Status(rsp.status().to_string()));
                };

                let body = rsp.text().await;
                let Ok(body) = body else {
                    return Err(Error::ReqwestError(body.unwrap_err().to_string()));
                };

                Ok(body)
            },
            Err(e) => Err(Error::ReqwestError(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    #[tokio::test]
    async fn test_request() {
        use crate::Error;
        use super::Request;
        let request = Request::new(Duration::from_millis(30_000)).unwrap();

        assert!(matches!(request.get("https://translate.google").await.unwrap_err(), Error::ReqwestError(_)));

        assert!(matches!(request.get("https://translate.google.com/xxxxx").await.unwrap_err(), Error::Status(_)));
    }
}
use crate::Result;
use crate::Error;
use crate::Language;
use crate::error::constant;
use crate::api::{DetectorAPI, TranslatorAPI, DetectorAPIContainer, TranslatorAPIContainer , Request, Translation};

use serde_json;
use scraper;
use regex;
use async_trait::async_trait;

// const TEXT_LIMIT: usize = 5_000;

// fn text_limit_check(text: &str) -> Result<usize> {
//     println!("{}", text);
//     let count = text.chars().count();
//     if count > TEXT_LIMIT {
//         return Err(Error::LengthLimit(count));
//     }
//     Ok(count)
// }

// https://translate.google.com/m?hl=en&sl={source}&tl={target}&q={text}
// The response is a HTML page: </style></head><body><div class="header"><div class="logo-image"></div><div class="logo-text">Translate</div></div><div class="languages-container"><div class="sl-and-tl"><a href="./m?sl=auto&amp;tl=zh-CN&amp;q=Hello%20world%21&amp;mui=sl&amp;hl=en">Detect language</a> → <a href="./m?sl=auto&amp;tl=zh-CN&amp;q=Hello%20world%21&amp;mui=tl&amp;hl=en">Chinese (Simplified)</a></div></div><div class="input-container"><form action="/m"><input type="hidden" name="sl" value="auto"><input type="hidden" name="tl" value="zh-CN"><input type="hidden" name="hl" value="en"><input type="text" aria-label="Source text" name="q" class="input-field" maxlength="2048" value="Hello world!"><div class="translate-button-container"><input type="submit" value="Translate" class="translate-button"></div></form></div><div class="result-container">你好世界！</div><div class="links-container"><ul><li><a href="https://www.google.com/m?hl=en">Google home</a></li><li><a href="https://www.google.com/tools/feedback/survey/xhtml?productId=95112&hl=en">Send feedback</a></li><li><a href="https://www.google.com/intl/en/policies">Privacy and terms</a></li><li><a href="./full">Switch to full site</a></li></ul></div></body></html>
// It returns only the translated content
// It is not suitable for translating longer texts, such as those with more than 5KB, and often returns a 400 error.
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub(crate) struct API_MobileGoogleTranslate {}

impl Into<TranslatorAPIContainer> for API_MobileGoogleTranslate {
    fn into(self) -> TranslatorAPIContainer {
        TranslatorAPIContainer::new(self)
    }
}

#[async_trait]
impl TranslatorAPI for API_MobileGoogleTranslate {
    async fn translate(&self, request: &Request, text: &str, source: Language, target: Language) -> Result<Translation> {
        // todo: 5k limit
        static API: &str = "https://translate.google.com/m";

        let sl = Language::abbreviation(&source).unwrap();
        let tl = Language::abbreviation(&target).unwrap();
        let params = format!("?hl=en&sl={}&tl={}&q={}", sl, tl, text);
        let url = format!("{}{}", API, params);
        let body = request.get(&url).await?;

        let selector = scraper::Selector::parse("div.result-container");
        let Ok(selector) = selector else {
            return Err(Error::HTMLParsingError(selector.unwrap_err().to_string()));
        };

        let document = scraper::Html::parse_document(&body);
        let Some(element) = document.select(&selector).next() else {
            return Err(Error::HTMLParsingError("no <div class=\"result-container\"> element".to_string()));
        };

        let Some(result) = element.children().next() else {
            return Err(Error::ReturnedEmptyResult);
        };

        let Some(value) = result.value().as_text() else {
            let selector = scraper::Selector::parse("div");
            let Ok(selector) = selector else {
                return Err(Error::HTMLParsingError(selector.unwrap_err().to_string()));
            };
            for main in document.select(&selector) {
                let Some(id) = main.attr("id") else {
                    continue
                };
                if "af-error-page2" != id {
                    continue;
                }
                // HTML snippet: <main id=\"af-error-container\" role=\"main\"><a href=//www.google.com><span id=logo aria-label=Google role=img></span></a><p><b>400.</b> <ins>That’s an error.</ins><p>The server cannot process the request because it is malformed. It should not be retried. <ins>That’s all we know.</ins></main>
                let reg = regex::Regex::new(r#"<title>Error (\d+) \((.*)\)!!.*</title>"#).unwrap();
                if let Some(captures) = reg.captures(&main.html()) {
                    return Err(Error::Status(captures[1].to_string() + " " + &captures[2].to_string()));
                }
            }
            return Err(Error::UnexpectedResult(format!("[{}:{}]{}, HTML: {body}", file!(), line!(), constant::UNEXPECTED_ERROR_STRING)));
        };

        Ok(Translation {
            source: None,
            target: target,
            result: value.to_string(),
        })
    }
}


// https://clients5.google.com/translate_a/t?client=dict-chrome-ex&sl={source}&tl={target}&q={text}
// The response is a json text: [["你好世界！","en"]]
// It returns the translated content and language
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub(crate) struct API_GoogleDictionaryChromeExtension {}

impl Into<DetectorAPIContainer> for API_GoogleDictionaryChromeExtension {
    fn into(self) -> DetectorAPIContainer {
        DetectorAPIContainer::new(self)
    }
}

impl Into<TranslatorAPIContainer> for API_GoogleDictionaryChromeExtension {
    fn into(self) -> TranslatorAPIContainer {
        TranslatorAPIContainer::new(self)
    }
}

#[async_trait]
impl DetectorAPI for API_GoogleDictionaryChromeExtension {
    async fn language(&self, request: &Request, text: &str) -> Result<Language> {
        let result = self.translate(request, text, Language::Auto, Language::English).await?;
        Ok(result.source.unwrap())
    }
}

#[async_trait]
impl TranslatorAPI for API_GoogleDictionaryChromeExtension {
    async fn translate(&self, request: &Request, text: &str, source: Language, target: Language) -> Result<Translation> {
        static API: &str = "https://clients5.google.com/translate_a/t";

        let sl = Language::abbreviation(&source).unwrap();
        let tl = Language::abbreviation(&target).unwrap();
        let params = format!("?client=dict-chrome-ex&sl={}&tl={}&q={}", sl, tl, text);
        let url = format!("{}{}", API, params);
        let body = request.get(&url).await?;

        let json = serde_json::from_str::<serde_json::Value>(&body);
        match json {
            Ok(json) => {
                if !json[0].is_array() {
                    return Err(Error::UnexpectedResult(format!("[{}:{}]{}, JSON: {body}", file!(), line!(), constant::UNEXPECTED_ERROR_STRING)));
                }
                if !json[0][0].is_string() {
                    return Err(Error::UnexpectedResult(format!("[{}:{}]{}, JSON: {body}", file!(), line!(), constant::UNEXPECTED_ERROR_STRING)));
                }
                if !json[0][1].is_string() {
                    return Err(Error::UnexpectedResult(format!("[{}:{}]{}, JSON: {body}", file!(), line!(), constant::UNEXPECTED_ERROR_STRING)));
                }

                let result = json[0][0].as_str().unwrap();
                let sl = json[0][1].as_str().unwrap();
                if 0 == result.len() { // [["", "en"]]
                    return Err(Error::ReturnedEmptyResult);
                }

                return Ok(Translation {
                    source: Language::from(sl),
                    target: target,
                    result: result.to_string(),
                })
            },
            Err(e) => {
                return Err(Error::JSONParsingError(e.to_string()));
            }
        }
    }
}


// https://translate.googleapis.com/translate_a/single?client=gtx&dt=t&sl={source}&tl={target}&q={text}
// The response is a json text
// It returns the translated content and language
// For the definition of `dt`, see https://stackoverflow.com/questions/26714426/what-is-the-meaning-of-google-translate-query-params/29537590#29537590
//
// Alternative APIs:
// https://translate.googleapis.com/translate_a/single?client=gtx&dt=t&dt=bd&dj=1&source=input&sl=auto&tl=en&&q=
// https://translate.googleapis.com/translate_a/single?client=gtx&dt=t&dt=bd&dt=ex&dt=ld&dt=md&dt=qca&dt=rw&dt=rm&dt=ss&dt=t&dt=at&sl=auto&tl=en&dj=1&source=bubble&hl=en&q=
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub(crate) struct API_GoogleTranslateExtensions {}

impl Into<DetectorAPIContainer> for API_GoogleTranslateExtensions {
    fn into(self) -> DetectorAPIContainer {
        DetectorAPIContainer::new(self)
    }
}

impl Into<TranslatorAPIContainer> for API_GoogleTranslateExtensions {
    fn into(self) -> TranslatorAPIContainer {
        TranslatorAPIContainer::new(self)
    }
}

#[async_trait]
impl DetectorAPI for API_GoogleTranslateExtensions {
    async fn language(&self, request: &Request, text: &str) -> Result<Language> {
        let result = self.translate(request, text, Language::Auto, Language::English).await?;
        Ok(result.source.unwrap())
    }
}

#[async_trait]
impl TranslatorAPI for API_GoogleTranslateExtensions {
    async fn translate(&self, request: &Request, text: &str, source: Language, target: Language) -> Result<Translation> {
        static API: &str = "https://translate.googleapis.com/translate_a/single";

        let sl = Language::abbreviation(&source).unwrap();
        let tl = Language::abbreviation(&target).unwrap();
        let params = format!("?client=gtx&dt=t&sl={}&tl={}&q={}", sl, tl, text);
        let url = format!("{}{}", API, params);
        let body = request.get(&url).await?;

        let json = serde_json::from_str::<serde_json::Value>(&body);
        match json {
            Ok(json) => {
                if !json[0].is_array() { // [null,null,"en",null,null,null,0,[],[["en"],null,[0],["en"]]]
                    return Err(Error::ReturnedEmptyResult);
                }
                if !json[0][0].is_array() {
                    return Err(Error::UnexpectedResult(format!("[{}:{}]{}, JSON: {body}", file!(), line!(), constant::UNEXPECTED_ERROR_STRING)));
                }
                if !json[8].is_array() {
                    return Err(Error::UnexpectedResult(format!("[{}:{}]{}, JSON: {body}", file!(), line!(), constant::UNEXPECTED_ERROR_STRING)));
                }
                if !json[8][0].is_array() {
                    return Err(Error::UnexpectedResult(format!("[{}:{}]{}, JSON: {body}", file!(), line!(), constant::UNEXPECTED_ERROR_STRING)));
                }
                if !json[8][0][0].is_string() {
                    return Err(Error::UnexpectedResult(format!("[{}:{}]{}, JSON: {body}", file!(), line!(), constant::UNEXPECTED_ERROR_STRING)));
                }

                let mut vec = vec![];
                for sentence in json[0].as_array().unwrap() {
                    if !sentence.is_array() {
                        return Err(Error::UnexpectedResult(format!("[{}:{}]{}, JSON: {body}", file!(), line!(), constant::UNEXPECTED_ERROR_STRING)));
                    }
                    if !sentence[0].is_string() {
                        return Err(Error::UnexpectedResult(format!("[{}:{}]{}, JSON: {body}", file!(), line!(), constant::UNEXPECTED_ERROR_STRING)));
                    }
                    vec.push(sentence[0].as_str().unwrap());
                }
                let result = vec.iter().fold(String::new(), |a, b| { a + b });
                if 0 == result.len() {
                    return Err(Error::ReturnedEmptyResult);
                }

                let sl = json[8][0][0].as_str().unwrap();

                return Ok(Translation {
                    source: Language::from(sl),
                    target: target,
                    result: result.to_string(),
                })
            },
            Err(e) => {
                return Err(Error::JSONParsingError(e.to_string()));
            }
        }
    }
}


// https://translate.google.com/_/TranslateWebserverUi/data/batchexecut
// 参考:
// https://github.com/Eveheeero/Eveheeero/blob/main/Jobs/Translator/src/lib.rs
// https://github.com/lushan88a/google_trans_new/blob/main/google_trans_new.py


#[cfg(test)]
mod tests {
    use std::time::Duration;

    #[tokio::test]
    #[allow(non_snake_case)]
    async fn test_API_MobileGoogleTranslate() {
        use crate::Error;
        use super::API_MobileGoogleTranslate;
        use super::TranslatorAPI;
        use crate::Language;
        use crate::api::Request;

        let api = API_MobileGoogleTranslate{};
        let request = Request::new(Duration::from_millis(30_000)).unwrap();

        assert!(matches!(api.translate(&request, &"Hello world!", Language::Auto, Language::SimpleChinese).await, Ok(_)));

        assert!(matches!(api.translate(&request, &"", Language::Auto, Language::SimpleChinese).await, Err(Error::ReturnedEmptyResult)));
    }

    #[tokio::test]
    #[allow(non_snake_case)]
    async fn test_API_GoogleDictionaryChromeExtension() {
        use crate::Error;
        use super::API_GoogleDictionaryChromeExtension;
        use super::TranslatorAPI;
        use crate::Language;
        use crate::api::Request;

        let api = API_GoogleDictionaryChromeExtension{};
        let request = Request::new(Duration::from_millis(30_000)).unwrap();

        assert!(matches!(api.translate(&request, &"Hello world!", Language::Auto, Language::SimpleChinese).await, Ok(_)));

        assert!(matches!(api.translate(&request, &"", Language::Auto, Language::SimpleChinese).await, Err(Error::ReturnedEmptyResult)));
    }

    #[tokio::test]
    #[allow(non_snake_case)]
    async fn test_API_GoogleTranslateExtensions() {
        use crate::Error;
        use crate::api::Request;
        use super::API_GoogleTranslateExtensions;
        use super::TranslatorAPI;
        use crate::Language;

        let api = API_GoogleTranslateExtensions{};
        let request = Request::new(Duration::from_millis(30_000)).unwrap();

        assert!(matches!(api.translate(&request, &"Hello world!", Language::Auto, Language::SimpleChinese).await, Ok(_)));

        assert!(matches!(api.translate(&request, &"", Language::Auto, Language::SimpleChinese).await, Err(Error::ReturnedEmptyResult)));
    }
}
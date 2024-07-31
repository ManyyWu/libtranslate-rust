# libtranslate

`libtranslate` is an asynchronous library for Google Translator, completely free without an API key.

## 1. Example
### 1.1 Dependency
Cargo.toml:
```toml
[dependencies]
libtranslate = { git = "https://github.com/ManyyWu/libtranslate-rust.git", tag = "v0.1.0" }
```
### 1.2 Translate text
```Rust
use std::time::Duration;
use libtranslate::*;

#[tokio::test]
async fn test() {
    let mut t = Translator::builder()
        .to(Language::SimpleChinese)
        .timeout(Duration::from_millis(3000))
        .build()
        .unwrap();

    let mut d = Detector::builder()
        .timeout(Duration::from_millis(3000))
        .build()
        .unwrap();

    println!("{:#?}", t.translate("Hello world").await);
    println!("{:#?}", d.language("Hello world").await);

    println!("{:#?}", translate("Hello world", Language::Auto, Language::SimpleChinese).await);
    println!("{:#?}", language("Hello world").await);
}
```
### 1.3 Builder
#### 1.3.1 Specify source and target languages
```Rust
Translator::builder()
    .from(Language::SimpleChinese)
    .to(Language::English)
    .strategy(Strategy::Default)
    .build();
```
#### 1.3.2 Specify target language and automatically detect language
```Rust
Translator::builder()
    .from(Language::Auto)
    .to(Language::English)
    .strategy(Strategy::Default)
    .build();
```
#### 1.3.3 Stratety
You can specify single or multiple APIs. The definition of Strategy is as follows:
```Rust
pub enum Strategy {
    Default,
    Single(String),
    Mix(Vec<String>),
}
```
Example:
```Rust
Default

Single("google.API_MobileGoogleTranslate".to_string())

Mix(vec![ "google.API_MobileGoogleTranslate".to_string(), "google.API_GoogleDictionaryChromeExtension".to_string(), "google.API_GoogleTranslateExtensions".to_string() ]))
```
#### 1.3.4 APIs supported by `libtranslate`:
  * `google.API_MobileGoogleTranslate`
  * `google.API_GoogleDictionaryChromeExtension`
  * `google.API_GoogleTranslateExtensions`

`libtranslate` uses all supported APIs by default.
> Note: Detector does not support `google.API_MobileGoogleTranslate`

## 2. Reference
[libretranslate-rs](https://github.com/grantshandy/libretranslate-rs)
[issues](https://github.com/ssut/py-googletrans/issues/268)

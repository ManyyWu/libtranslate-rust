use std::time::Duration;
use libtranslate::*;
use libtranslate::Strategy::*;

#[tokio::test]
async fn test() {
    let mut t = Translator::builder()
        .timeout(Duration::from_millis(3000))
        .strategy(Mix(vec!["google.API_MobileGoogleTranslate".to_string(), "google.API_GoogleDictionaryChromeExtension".to_string(), "google.API_GoogleTranslateExtensions".to_string()]))
        .build()
        .unwrap();

    let mut d = Detector::builder()
        .timeout(Duration::from_millis(3000))
        .build()
        .unwrap();

    println!("{:#?}", t.translate("Hello world", Language::Auto, Language::SimpleChinese).await);
    println!("{:#?}", d.language("Hello world").await);

    println!("{:#?}", translate("Hello world", Language::Auto, Language::SimpleChinese).await);
    println!("{:#?}", language("Hello world").await);
}
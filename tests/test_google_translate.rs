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
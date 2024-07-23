#[derive(PartialEq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Language {
    Auto                 , // auto
    Afrikaans            , // af
    Amharic              , // am
    Arabic               , // ar
    Azerbaijani          , // az
    Belarusian           , // be
    Bulgarian            , // bg
    Bengali              , // bn
    Bosnian              , // bs
    Catalan              , // ca
    Corsican             , // co
    Czech                , // cs
    Welsh                , // cy
    Danish               , // da
    German               , // de
    Greek                , // el
    English              , // en
    Esperanto            , // eo
    Spanish              , // es
    Estonian             , // et
    Basque               , // eu
    Persian              , // fa
    Finnish              , // fi
    French               , // fr
    WesternFrisian       , // fy
    Irish                , // ga
    ScottishGaelic       , // gd
    Galician             , // gl
    Gujarati             , // gu
    Hausa                , // ha
    Hebrew               , // he
    Hindi                , // hi
    Croatian             , // hr
    HaitianCreole        , // ht
    Hungarian            , // hu
    Armenian             , // hy
    Indonesian           , // id
    Igbo                 , // ig
    Icelandic            , // is
    Italian              , // it
    Japanese             , // ja
    Georgian             , // ka
    Kazakh               , // kk
    CentralKhmer         , // km
    Kannada              , // kn
    Korean               , // ko
    Kurdish              , // ku
    Kirghiz              , // ky
    Latin                , // la
    Luxembourgish        , // lb
    Lao                  , // lo
    Lithuanian           , // lt
    Latvian              , // lv
    Malagasy             , // mg
    Maori                , // mi
    Macedonian           , // mk
    Malayalam            , // ml
    Mongolian            , // mn
    Marathi              , // mr
    Malay                , // ms
    Maltese              , // mt
    Burmese              , // my
    Nepali               , // ne
    Dutch                , // nl
    Norwegian            , // no
    Chichewa             , // ny
    Oriya                , // or
    Panjabi              , // pa
    Polish               , // pl
    Pushto               , // ps
    Portuguese           , // pt
    Romanian             , // ro
    Russian              , // ru
    Sindhi               , // sd
    Sinhalese            , // si
    Slovak               , // sk
    Slovenian            , // sl
    Samoan               , // sm
    Shona                , // sn
    Somali               , // so
    Albanian             , // sq
    Serbian              , // sr
    SothoSouthern        , // st
    Sundanese            , // su
    Swedish              , // sv
    Swahili              , // sw
    Tamil                , // ta
    Telugu               , // te
    Tajik                , // tg
    Thai                 , // th
    Tagalog              , // tl
    Turkish              , // tr
    Uighur               , // ug
    Ukrainian            , // uk
    Urdu                 , // ur
    Uzbek                , // uz
    Vietnamese           , // vi
    Xhosa                , // xh
    Yiddish              , // yi
    Yoruba               , // yo
    SimpleChinese        , // zh-CN
    TraditionalChinese   , // zh-TW
    Zulu                 , // zu
}

impl Language {
    pub(crate) fn abbreviation(&self) -> Option<&'static str> {
        use self::Language::*;

        match self {
            Auto                 => Some("auto"),
            Afrikaans            => Some("af"),
            Amharic              => Some("am"),
            Arabic               => Some("ar"),
            Azerbaijani          => Some("az"),
            Belarusian           => Some("be"),
            Bulgarian            => Some("bg"),
            Bengali              => Some("bn"),
            Bosnian              => Some("bs"),
            Catalan              => Some("ca"),
            Corsican             => Some("co"),
            Czech                => Some("cs"),
            Welsh                => Some("cy"),
            Danish               => Some("da"),
            German               => Some("de"),
            Greek                => Some("el"),
            English              => Some("en"),
            Esperanto            => Some("eo"),
            Spanish              => Some("es"),
            Estonian             => Some("et"),
            Basque               => Some("eu"),
            Persian              => Some("fa"),
            Finnish              => Some("fi"),
            French               => Some("fr"),
            WesternFrisian       => Some("fy"),
            Irish                => Some("ga"),
            ScottishGaelic       => Some("gd"),
            Galician             => Some("gl"),
            Gujarati             => Some("gu"),
            Hausa                => Some("ha"),
            Hebrew               => Some("he"),
            Hindi                => Some("hi"),
            Croatian             => Some("hr"),
            HaitianCreole        => Some("ht"),
            Hungarian            => Some("hu"),
            Armenian             => Some("hy"),
            Indonesian           => Some("id"),
            Igbo                 => Some("ig"),
            Icelandic            => Some("is"),
            Italian              => Some("it"),
            Japanese             => Some("ja"),
            Georgian             => Some("ka"),
            Kazakh               => Some("kk"),
            CentralKhmer         => Some("km"),
            Kannada              => Some("kn"),
            Korean               => Some("ko"),
            Kurdish              => Some("ku"),
            Kirghiz              => Some("ky"),
            Latin                => Some("la"),
            Luxembourgish        => Some("lb"),
            Lao                  => Some("lo"),
            Lithuanian           => Some("lt"),
            Latvian              => Some("lv"),
            Malagasy             => Some("mg"),
            Maori                => Some("mi"),
            Macedonian           => Some("mk"),
            Malayalam            => Some("ml"),
            Mongolian            => Some("mn"),
            Marathi              => Some("mr"),
            Malay                => Some("ms"),
            Maltese              => Some("mt"),
            Burmese              => Some("my"),
            Nepali               => Some("ne"),
            Dutch                => Some("nl"),
            Norwegian            => Some("no"),
            Chichewa             => Some("ny"),
            Oriya                => Some("or"),
            Panjabi              => Some("pa"),
            Polish               => Some("pl"),
            Pushto               => Some("ps"),
            Portuguese           => Some("pt"),
            Romanian             => Some("ro"),
            Russian              => Some("ru"),
            Sindhi               => Some("sd"),
            Sinhalese            => Some("si"),
            Slovak               => Some("sk"),
            Slovenian            => Some("sl"),
            Samoan               => Some("sm"),
            Shona                => Some("sn"),
            Somali               => Some("so"),
            Albanian             => Some("sq"),
            Serbian              => Some("sr"),
            SothoSouthern        => Some("st"),
            Sundanese            => Some("su"),
            Swedish              => Some("sv"),
            Swahili              => Some("sw"),
            Tamil                => Some("ta"),
            Telugu               => Some("te"),
            Tajik                => Some("tg"),
            Thai                 => Some("th"),
            Tagalog              => Some("tl"),
            Turkish              => Some("tr"),
            Uighur               => Some("ug"),
            Ukrainian            => Some("uk"),
            Urdu                 => Some("ur"),
            Uzbek                => Some("uz"),
            Vietnamese           => Some("vi"),
            Xhosa                => Some("xh"),
            Yiddish              => Some("yi"),
            Yoruba               => Some("yo"),
            SimpleChinese        => Some("zh-CN"),
            TraditionalChinese   => Some("zh-TW"),
            Zulu                 => Some("zu"),
        }
    }

   pub(crate) fn from(abbreviation: &str) -> Option<Language> {
        use self::Language::*;

        match abbreviation {
            "auto"   => Some(Auto),
            "af"     => Some(Afrikaans),
            "am"     => Some(Amharic),
            "ar"     => Some(Arabic),
            "az"     => Some(Azerbaijani),
            "be"     => Some(Belarusian),
            "bg"     => Some(Bulgarian),
            "bn"     => Some(Bengali),
            "bs"     => Some(Bosnian),
            "ca"     => Some(Catalan),
            "co"     => Some(Corsican),
            "cs"     => Some(Czech),
            "cy"     => Some(Welsh),
            "da"     => Some(Danish),
            "de"     => Some(German),
            "el"     => Some(Greek),
            "en"     => Some(English),
            "eo"     => Some(Esperanto),
            "es"     => Some(Spanish),
            "et"     => Some(Estonian),
            "eu"     => Some(Basque),
            "fa"     => Some(Persian),
            "fi"     => Some(Finnish),
            "fr"     => Some(French),
            "fy"     => Some(WesternFrisian),
            "ga"     => Some(Irish),
            "gd"     => Some(ScottishGaelic),
            "gl"     => Some(Galician),
            "gu"     => Some(Gujarati),
            "ha"     => Some(Hausa),
            "he"     => Some(Hebrew),
            "hi"     => Some(Hindi),
            "hr"     => Some(Croatian),
            "ht"     => Some(HaitianCreole),
            "hu"     => Some(Hungarian),
            "hy"     => Some(Armenian),
            "id"     => Some(Indonesian),
            "ig"     => Some(Igbo),
            "is"     => Some(Icelandic),
            "it"     => Some(Italian),
            "ja"     => Some(Japanese),
            "ka"     => Some(Georgian),
            "kk"     => Some(Kazakh),
            "km"     => Some(CentralKhmer),
            "kn"     => Some(Kannada),
            "ko"     => Some(Korean),
            "ku"     => Some(Kurdish),
            "ky"     => Some(Kirghiz),
            "la"     => Some(Latin),
            "lb"     => Some(Luxembourgish),
            "lo"     => Some(Lao),
            "lt"     => Some(Lithuanian),
            "lv"     => Some(Latvian),
            "mg"     => Some(Malagasy),
            "mi"     => Some(Maori),
            "mk"     => Some(Macedonian),
            "ml"     => Some(Malayalam),
            "mn"     => Some(Mongolian),
            "mr"     => Some(Marathi),
            "ms"     => Some(Malay),
            "mt"     => Some(Maltese),
            "my"     => Some(Burmese),
            "ne"     => Some(Nepali),
            "nl"     => Some(Dutch),
            "no"     => Some(Norwegian),
            "ny"     => Some(Chichewa),
            "or"     => Some(Oriya),
            "pa"     => Some(Panjabi),
            "pl"     => Some(Polish),
            "ps"     => Some(Pushto),
            "pt"     => Some(Portuguese),
            "ro"     => Some(Romanian),
            "ru"     => Some(Russian),
            "sd"     => Some(Sindhi),
            "si"     => Some(Sinhalese),
            "sk"     => Some(Slovak),
            "sl"     => Some(Slovenian),
            "sm"     => Some(Samoan),
            "sn"     => Some(Shona),
            "so"     => Some(Somali),
            "sq"     => Some(Albanian),
            "sr"     => Some(Serbian),
            "st"     => Some(SothoSouthern),
            "su"     => Some(Sundanese),
            "sv"     => Some(Swedish),
            "sw"     => Some(Swahili),
            "ta"     => Some(Tamil),
            "te"     => Some(Telugu),
            "tg"     => Some(Tajik),
            "th"     => Some(Thai),
            "tl"     => Some(Tagalog),
            "tr"     => Some(Turkish),
            "ug"     => Some(Uighur),
            "uk"     => Some(Ukrainian),
            "ur"     => Some(Urdu),
            "uz"     => Some(Uzbek),
            "vi"     => Some(Vietnamese),
            "xh"     => Some(Xhosa),
            "yi"     => Some(Yiddish),
            "yo"     => Some(Yoruba),
            "zh-CN"  => Some(SimpleChinese),
            "zh-TW"  => Some(TraditionalChinese),
            "zu"     => Some(Zulu),
            _        => None,
        }
    }
}
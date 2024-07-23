mod api;
mod error;
mod language;
mod translator;

pub use self::language::Language;
pub use self::error::{Error, Result};
pub use self::translator::{Strategy, TranslatorType, TranslatorBuilder, Translator, DetectorBuilder, Detector, translate, language};
pub use self::api::{Translation};
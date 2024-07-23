use thiserror;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("system error")]
    System,

    #[error("the detector's strategy is not specified")]
    NoDetectorRegistrationService,

    #[error("the translator's strategy is not specified")]
    NoTranslatorRegistrationService,

    #[error("invalid service name")]
    InvalidServiceName,

    #[error("no target language set")]
    NoTargetLanguage,

    #[error("the target language cannot be `Auto`")]
    TargetLanguageIsAuto,

    #[error("the target language cannot be `Auto`")]
    TargetEqualToSource,

    #[error("No service available, please try again later")]
    NoAvailableService(std::time::Duration),

    #[error("Input text is too long {0}")]
    LengthLimit(usize),

    #[error("{0}")]
    UnexpectedResult(String),

    #[error("returns empty result")]
    ReturnedEmptyResult,

    #[error("{0}")]
    ReqwestError(String),

    #[error("HTML parsing error {0}")]
    HTMLParsingError(String),

    #[error("JSON parsing error {0}")]
    JSONParsingError(String),

    #[error("{0}")]
    Status(String),
}

pub(crate) mod constant {
    pub(crate) const UNEXPECTED_ERROR_STRING: &str = "nexpected error occurred, please report this to the developer";
}

pub type Result<T> = std::result::Result<T, Error>;


#[cfg(test)]
mod tests {
    #[test]
    fn test_error_type() {
    }
}
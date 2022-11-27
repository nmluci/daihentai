use core::fmt;

#[derive(Clone, Debug
)]
pub enum DaiHentaiError {
   RequestError,
   ApiError(String),
   ConfigError(String),
}

impl std::error::Error for DaiHentaiError {
   fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
      None
   }


}

impl fmt::Display for DaiHentaiError {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         DaiHentaiError::RequestError => write!(f, "failed to fetch request"),
         DaiHentaiError::ConfigError(key) => write!(f, "Config error: mandatory config: {} is not set", key),
         DaiHentaiError::ApiError(e) => write!(f, "API error: {}", e)
      }
   }
}

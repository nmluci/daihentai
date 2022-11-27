use crate::errors::DaiHentaiError::ConfigError;

pub fn init_config() {
   dotenv::dotenv().ok();
}

pub fn get_config(key: String) -> Result<String, Box<dyn std::error::Error>> {
   let _key = key.clone();

   match std::env::var(key) {
      Ok(val) => return Ok(val),
      Err(_) => return Err(Box::new(ConfigError(_key))),
   };
}
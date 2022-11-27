use std::{collections::HashMap};

use crate::{errors::DaiHentaiError, consts, config};

pub fn send_request(url: &String, method: &reqwest::Method, params: &HashMap<String, String>) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
   let client = match reqwest::blocking::Client::builder()
      .redirect(reqwest::redirect::Policy::none())
      .build() {
         Ok(client) => client,
         Err(e) => return Err(Box::new(DaiHentaiError::ApiError(format!("failed to build HTTP Client error: {}", e))))
      };
   
   let mut req = client.request(method.clone(), url.clone());
   // req = req.header("User-Agent", format!("daihentai v{}", consts::LIB_VER));
   req = match config::get_config("USER_AGENT".to_string()) {
      Ok(key) => req.header("user-agent", key),
      Err(e) => return Err(e),
   };

   req = match config::get_config("CF_COOKIE".to_string()) {
      Ok(key) => req.header("cookie", key),
      Err(e) => return Err(e),
   };

   for (k, v) in params.clone() {
      req = req.query(&[(k, v)]);
   }

   match client.execute(req.build()?) {
      Ok(data) => return Ok(data),
      Err(e) =>  {
         println!("an error occurred while performing request {} to {} for {:?}: {}", method.as_str(), url, params, e);
         return Err(Box::new(DaiHentaiError::RequestError));
      }
   };
}
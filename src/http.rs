use std::{collections::HashMap};
use reqwest;

use crate::{errors::DaiHentaiError, api::DaiHentaiAPI, config};

impl DaiHentaiAPI {
   pub(crate) fn init_client() -> Result<reqwest::blocking::Client, Box<dyn std::error::Error>> {
      match reqwest::blocking::Client::builder()
         .redirect(reqwest::redirect::Policy::none())
         .build() {
            Ok(_client) => return Ok(_client),
            Err(e) => return Err(Box::new(DaiHentaiError::ApiError(format!("failed to build HTTP Client error: {}", e))))
         };
   }

   pub(crate) fn send_request(&self, url: &String, method: &reqwest::Method, params: Option<&HashMap<String, String>>) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
      let mut req = self.client.request(method.clone(), url.clone());
      req = match config::get_config("USER_AGENT".to_string()) {
         Ok(key) => req.header("user-agent", key),
         Err(e) => return Err(e),
      };
   
      req = match config::get_config("CF_COOKIE".to_string()) {
         Ok(key) => req.header("cookie", key),
         Err(e) => return Err(e),
      };
   
      if let Some(params) = params {
         for (k, v) in params.clone() {
            req = req.query(&[(k, v)]);
         }
      }
   
      match self.client.execute(req.build()?) {
         Ok(data) => return Ok(data),
         Err(e) =>  {
            println!("an error occurred while performing request {} to {} for {:?}: {}", method.as_str(), url, params, e);
            return Err(Box::new(DaiHentaiError::RequestError));
         }
      };
   }
}


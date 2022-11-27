use std::{collections::HashMap};

use crate::{errors::DaiHentaiError, consts};

pub fn send_request(url: &String, method: &reqwest::Method, params: &HashMap<String, String>) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
   let client: reqwest::blocking::Client = reqwest::blocking::Client::new();
   
   let mut req = client.request(method.clone(), url.clone());
   req = req.header("User-Agent", format!("daihentai v{}", consts::LIB_VER));

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
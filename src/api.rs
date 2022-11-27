use std::{collections::HashMap};
use crate::errors::DaiHentaiError::{ApiError};

use crate::book;
use crate::gallery;
use crate::http;
use crate::consts;
use crate::config;


pub fn get_by_id(id: i64) -> Result<book::Book, Box<dyn std::error::Error>> {
   config::init_config();

   let data = http::send_request(
      &format!("{}/api/gallery/{}", consts::BASE_URL, id),
      &reqwest::Method::GET,
      &HashMap::new(),
   );

   let resp: reqwest::blocking::Response = match data {
      Ok(resp) => resp,
      Err(e) => return Err(Box::new(ApiError(format!("failed to fetch book with ID: {} error: {}", id, e)))),
   };

   let gallery: gallery::Gallery = match serde_json::from_str(&resp.text().unwrap()) {
      Ok(data) => data,
      Err(e) => return Err(Box::new(ApiError(format!("failed to parse JSON result for ID: {} error: {}", id, e))))
   };

   match gallery.to_book() {
      Ok(book) => return Ok(book),
      Err(e) => return Err(Box::new(ApiError(format!("failed to parse result into Book error: {}", e))))
   }   
}

pub fn get_random() -> Result<book::Book, Box<dyn std::error::Error>> {
   config::init_config();
   
   let _data = http::send_request(
      &format!("{}/random/", consts::BASE_URL),
      &reqwest::Method::GET,
      &HashMap::new(),   
   );

   let _resp: reqwest::blocking::Response = match _data {
      Ok(resp) => resp,
      Err(e) => return Err(Box::new(ApiError(format!("failed to fetch random book error: {}", e)))),
   };
   
   let Some(nuke_id) = _resp.headers().get("location")
      .and_then(|nuke| Some(&nuke.to_str().unwrap()[3..nuke.len()-1])) else { 
         return Err(Box::new(ApiError(format!("failed to fetch parse randomized nuke code error"))));
      };

   let data = http::send_request(
      &format!("{}/api/gallery/{}", consts::BASE_URL, nuke_id),
      &reqwest::Method::GET,
      &HashMap::new(),   
   );

   let resp = match data {
      Ok(resp) => resp,
      Err(e) => return Err(Box::new(ApiError(format!("failed to fetch random book error: {}", e)))),
   };

   let json_str = &resp.text().unwrap();
   let gallery: gallery::Gallery = match serde_json::from_str(&json_str) {
      Ok(data) => data,
      Err(e) => {
         println!("JSON Rawdata: {}", json_str);
         return Err(Box::new(ApiError(format!("failed to parse JSON result error: {}", e))))
      } 
   };

   match gallery.to_book() {
      Ok(book) => return Ok(book),
      Err(e) => return Err(Box::new(ApiError(format!("failed to parse result into Book error: {}", e))))
   }   
}
use std::{collections::HashMap};
use crate::errors::DaiHentaiError::{ApiError};

use crate::book;
use crate::gallery;
use crate::http;
use crate::consts;


pub fn get_by_id(id: i64) -> Result<book::Book, Box<dyn std::error::Error>> {
   let data = http::send_request(
      &format!("{}/{}", consts::BASE_URL, id),
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
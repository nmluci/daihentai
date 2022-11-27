use std::collections::HashMap;

use crate::errors::DaiHentaiError::{ApiError};

use crate::book;
use crate::gallery;
use crate::consts;
use crate::config;

type BookResult = Result<book::Book, Box<dyn std::error::Error>>;
type VecBookResult = Result<Vec<book::Book>, Box<dyn std::error::Error>>;

pub struct DaiHentaiAPI {
   pub(crate) user_agent: String,
   pub(crate) cookie: String,
   pub(crate) proxy_url: String, 
   pub(crate) override_baseurl: bool,
   pub(crate) client: reqwest::blocking::Client,
}

impl DaiHentaiAPI {
   pub fn new() -> Result<DaiHentaiAPI, Box<dyn std::error::Error>> {
      config::init_config();

      let mut api = DaiHentaiAPI{
         user_agent: "".to_string(),
         cookie: "".to_string(),
         proxy_url: config::get_optional_config("NH_PROXY_URL".to_string()),
         override_baseurl: config::get_optional_config("USE_PROXY".to_string()).parse().unwrap_or(false), 
         client: match DaiHentaiAPI::init_client() {
            Ok(_client) => _client, 
            Err(e) => return Err(e),
         },
      };

      api.user_agent = match config::get_config("USER_AGENT".to_string()) {
         Ok(_key) => _key, 
         Err(e) => return Err(e),
      };

      api.cookie = match config::get_config("CF_COOKIE".to_string()) {
         Ok(_key) => _key,
         Err(e) => return Err(e),
      };

      Ok(api)
   }

   pub fn update_key(&mut self, key: &str, val: String) -> Option<Box<dyn std::error::Error>> {
      match key {
         "user_agent" => self.user_agent = val,
         "cookie" => self.cookie = val,
         _ => return Some(Box::new(ApiError(format!("invalid key for {}", key)))),
      };

      return None;
   }

   pub fn refresh_config(&mut self) -> Option<Box<dyn std::error::Error>> {
      self.user_agent = match config::get_config("USER_AGENT".to_string()) {
         Ok(_key) => _key, 
         Err(e) => return Some(e),
      };

      self.cookie = match config::get_config("CF_COOKIE".to_string()) {
         Ok(_key) => _key,
         Err(e) => return Some(e),
      };

      return None
   }

   pub fn get_by_id(&self, id: i64) -> BookResult {
      let mut base_url = consts::BASE_URL;
      if self.override_baseurl && self.proxy_url.len() != 0 {
         base_url = &self.proxy_url;
      }

      let data = self.send_request(
         &format!("{}/api/gallery/{}", base_url, id),
         &reqwest::Method::GET,
         None
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
   
   pub fn get_random(&self) -> BookResult {
      let _data = self.send_request(
         &format!("{}/random/", consts::BASE_URL),
         &reqwest::Method::GET,
         None
      );
   
      let _resp: reqwest::blocking::Response = match _data {
         Ok(resp) => resp,
         Err(e) => return Err(Box::new(ApiError(format!("failed to fetch random book error: {}", e)))),
      };
      
      let Some(nuke_id) = _resp.headers().get("location")
         .and_then(|nuke| Some(&nuke.to_str().unwrap()[3..nuke.len()-1])) else { 
            return Err(Box::new(ApiError(format!("failed to fetch parse randomized nuke code error"))));
         };
   
      let data = self.send_request(
         &format!("{}/api/gallery/{}", consts::BASE_URL, nuke_id),
         &reqwest::Method::GET,
         None
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

   pub fn get_related(&self, id: i64) -> VecBookResult {
      let mut base_url = consts::BASE_URL;
      if self.override_baseurl && self.proxy_url.len() != 0 {
         base_url = &self.proxy_url;
      }
      
      let data = self.send_request(
         &format!("{}/api/gallery/{}/related", base_url, id),
         &reqwest::Method::GET,
         None
      );

      let resp = match data {
         Ok(_resp) => _resp,
         Err(e) => return Err(Box::new(ApiError(format!("failed to fetch related book for: {} error: {}", id, e)))),
      };

      let json_str = &resp.text().unwrap();
      let galleries: gallery::Galleries = match serde_json::from_str(&json_str) {
         Ok(_data) => _data, 
         Err(e) => return Err(Box::new(ApiError(format!("failed to parse JSON result error: {}", e)))),
      };

      let mut books: Vec<book::Book> = Vec::new();
      for gallery in galleries.result.iter() {
         match gallery.to_book() {
            Ok(_book) => books.push(_book),
            Err(e) => return Err(Box::new(ApiError(format!("failed to parse result into Book error: {}", e)))),
         }
      }

      return Ok(books)
   }


   pub fn search(&self, query: &String, page: i64, sort: book::SortOption) -> VecBookResult {
      let mut base_url = consts::BASE_URL;
      if self.override_baseurl && self.proxy_url.len() != 0 {
         base_url = &self.proxy_url;
      }

      let mut params: HashMap<String, String> = HashMap::new();
      params.insert("q".to_string(), query.clone().to_string());
      params.insert("query".to_string(), query.clone().to_string());
      
      if page >= 1 {
         params.insert("page".to_string(), stringify!(page).to_string());
      }

      if let Some(sort) = book::format_sort_option(&sort) {
         params.insert("sort".to_string(), sort);
      }

      let data = self.send_request(
         &format!("{}/api/galleries/search", base_url),
         &reqwest::Method::GET,
         Some(&params)
      );

      let resp = match data {
         Ok(_resp) => _resp, 
         Err(e) => return Err(Box::new(ApiError(format!("failed to fetch book with {:?} error: {}", query, e)))),
      };

      let json_str = &resp.text().unwrap();
      let galleries: gallery::Galleries = match serde_json::from_str(&json_str) {
         Ok(_data) => _data,
         Err(e) => {
            return Err(Box::new(ApiError(format!("failed to parse JSON result error: {}", e))));
         }
      };

      let mut books: Vec<book::Book> = Vec::new();
      for gallery in galleries.result.iter() {
         match gallery.to_book() {
            Ok(_book) => books.push(_book),
            Err(e) => return Err(Box::new(ApiError(format!("failed to parse result into Book error: {}", e)))),
         }
      }

      return Ok(books)

   }
}

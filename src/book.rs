use serde::{Deserialize, Serialize};
use crate::parser::deserialize_nullable;

pub enum TagOption {
   Lang, 
   Char, 
   Tags, 
   Artist,
   Parody,
   Group
}

pub fn format_tag_option(opt: &TagOption) -> Result<String, Box<dyn std::error::Error>> {
   match opt {
      TagOption::Lang => return Ok("language".to_string()),
      TagOption::Char => return Ok("character".to_string()),
      TagOption::Tags => return Ok("tag".to_string()),
      TagOption::Artist => return Ok("artist".to_string()),
      TagOption::Parody => return Ok("parody".to_string()),
      TagOption::Group => return Ok("group".to_string()),
 }
}

pub enum SortOption {
   Popular,
   PopularYear,
   PopularMonth,
   PopularWeek,
   PopularToday,
   Date
}

pub fn format_sort_option(opt: &SortOption) -> Option<String> {
   match opt {
      SortOption::Popular => return Some("popular".to_string()),
      SortOption::PopularYear => return Some("popular-year".to_string()),
      SortOption::PopularMonth => return Some("popular-month".to_string()),
      SortOption::PopularWeek => return Some("popular-week".to_string()),
      SortOption::PopularToday => return Some("popular-today".to_string()),
      SortOption::Date => return Some("date".to_string()),
   }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
   #[serde(deserialize_with = "deserialize_nullable")]
   pub english: String,
   #[serde(deserialize_with = "deserialize_nullable")]
   pub japanese: String, 
   #[serde(deserialize_with = "deserialize_nullable")]
   pub pretty: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
   pub id: i64, 
   #[serde(rename = "type")]
   pub tag_type: String,
   pub name: String,
   pub url: String, 
   pub count: i64
}

pub type BookTags = Vec<Tag>;

pub fn format_tags(tags: &BookTags, opt: &TagOption) -> Result<String, Box<dyn std::error::Error>> {
   let mut data : Vec<String> = Vec::new();

   for tag in tags {
      if tag.tag_type == format_tag_option(&opt).unwrap() && tags.len() >= 2 {
         data.push(tag.name.clone())
      }
   }

   Ok(data.join(", "))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
   pub url: String, 
   pub width: i64,
   pub height: i64
}

pub type BookPages = Vec<Page>;

pub fn get_urls(pages: &BookPages) -> Result<Vec<String>, Box<dyn std::error::Error>> {
   let mut data: Vec<String> = Vec::new();

   for page in pages {
      data.push(page.url.clone())
   }

   Ok(data)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMetadata {
   pub t: String,
   pub w: i64,
   pub h: i64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawImage {
   pub pages: Vec<ImageMetadata>,
   pub cover: ImageMetadata,
   pub thumbnail: ImageMetadata
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
   pub id: i64, 
   pub media_id: i64, 
   pub title: Title, 
   pub favorites: i64, 
   pub thumbnail: String, 
   pub cover: String, 
   pub scanlator: String, 
   pub uploaded: String, 
   pub epoch: i64, 
   pub characters: BookTags,
   pub pages: BookPages,
   pub tags: Vec<String>,
   pub num_pages: i64, 
   pub raw_tags: BookTags, 
   pub raw_images: RawImage
}

impl Book {
   pub fn get_tags(self, opt: &TagOption) -> Result<Vec<String>, Box<dyn std::error::Error>> {
      let mut data: Vec<String> = Vec::new();

      for tag in self.raw_tags {
         if tag.tag_type != format_tag_option(opt).unwrap() {
            continue;
         }

         data.push(tag.name.clone())
      }

      Ok(data)
   }
}

pub fn get_ext(ext: &str) -> String {
   match ext {
      "j" => return "jpg".to_string(),
      "p" => return "png".to_string(),
      "g" => return "gif".to_string(),
      _ => return "".to_string()
   }
}

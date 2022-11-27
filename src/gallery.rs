use crate::book::{get_ext, Book, BookTags, Page, RawImage, Title};
use crate::parser::parse_int;
use std::borrow::Borrow;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Galleries {
    pub result: Vec<Gallery>,
    pub num_page: i64,
    pub per_page: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gallery {
    pub id: serde_json::Value,
    pub media_id: String,
    pub title: Title,
    #[serde(rename = "num_favorites")]
    pub favorites: i64,
    pub scanlator: String,
    pub upload_date: i64,
    pub num_pages: i64,
    pub tags: BookTags,
    pub images: RawImage,
}

impl Gallery {
    pub fn to_book(&self) -> Result<Book, Box<dyn std::error::Error>> {
        let mut data: Book = Book {
            id: 0, 
            media_id: parse_int(&self.media_id),
            title: self.title.clone(),
            favorites: self.favorites,
            thumbnail: format!(
                "https://t.nhentai.net/galleries/{}/thumb.{}",
                self.media_id,
                get_ext(self.images.thumbnail.t.as_str())
            ),
            cover: format!(
                "https://t.nhentai.net/galleries/{}/cover.{}",
                self.media_id,
                get_ext(self.images.cover.t.as_str())
            ),
            scanlator: (*self.scanlator).to_string(),
            uploaded: "".to_string(),
            epoch: self.upload_date,
            characters: Vec::new(),
            pages: Vec::new(),
            tags: Vec::new(),
            num_pages: self.num_pages,
            raw_tags: self.tags.clone(),
            raw_images: self.images.clone(),
        };

        data.id = match &self.id {
            _i64 if self.id.is_i64() => self.id.as_i64().unwrap(),
            _str if self.id.is_string() => parse_int(self.id.as_str().unwrap().to_string().borrow()),
            _ => 0
        };

        for (idx, page) in self.images.pages.iter().enumerate() {
            let temp = Page {
                url: format!(
                    "https://i.nhentai.net/galleries/{}/{}.{}",
                    self.media_id,
                    idx,
                    get_ext(page.t.as_str())
                ),
                width: page.w,
                height: page.h,
            };

            data.pages.push(temp);
        }

        for tag in &data.raw_tags {
            if tag.tag_type != "tag" {
                continue;
            };

            data.tags.push((*tag.name).to_string());
        }

        Ok(data)
    }
}

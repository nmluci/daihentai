
use std::{collections::HashMap};
use daihentai::gallery::{Gallery};
use daihentai::book::{Book, TagOption, format_tags};
use daihentai::http;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   let nuke_code = std::env::args().nth(1).expect("Please specify your nuke code");

   let data = http::send_request(
      format!("http://138.2.77.198:3002/api/gallery/{}", nuke_code).to_string(),
      reqwest::Method::GET,
      HashMap::new(),
   ).await?;

   let text_resp = data.text().await?;
   let gallery: Gallery = serde_json::from_str(&text_resp)?;
   let book: Book = gallery.to_book().unwrap();
   println!("id: {}", book.id);
   println!("title: {}", book.title.pretty);
   println!("tags: {}", format_tags(&book.raw_tags, &TagOption::Tags).unwrap());
   println!("pages: {}", book.num_pages);
   Ok(())
}


use daihentai::{api, book};


fn main() -> Result<(), Box<dyn std::error::Error>> {
   // let nuke_code = std::env::args().nth(1).expect("Please specify your nuke code");
   // let book: book::Book = match api::get_by_id(nuke_code.parse::<i64>().unwrap()) {
   //    Ok(data) => data,
   //    Err(e) => return Err(e),
   // };

   let book: book::Book = match api::get_random() {
      Ok(data) => data,
      Err(e) => return Err(e),
   };

   println!("id: {}", book.id);
   println!("title: {}", book.title.pretty);
   println!("tags: {}", book::format_tags(&book.raw_tags, &book::TagOption::Tags).unwrap());
   println!("pages: {}", book.num_pages);
   Ok(())
}


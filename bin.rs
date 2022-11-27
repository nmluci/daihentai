use daihentai::api::DaiHentaiAPI;
use daihentai::book;

fn main() -> Result<(), Box<dyn std::error::Error>> {
   // let nuke_code = std::env::args().nth(1).expect("Please specify your nuke code");
   let nuke_string = std::env::args().nth(1).expect("Please specify taste");

   let api = match DaiHentaiAPI::new() {
      Ok(_api) => _api, 
      Err(e) => return Err(e),
   };

   // let book: book::Book = match api.get_random() {
   //    Ok(data) => data,
   //    Err(e) => return Err(e),
   // };

   // println!("id: {}", book.id);
   // println!("title: {}", book.title.pretty);
   // println!("cover_img: {}", book.cover);
   // println!("tags: {}", book::format_tags(&book.raw_tags, &book::TagOption::Tags).unwrap());
   // println!("pages: {}", book.num_pages);
   
   let books: Vec<book::Book> = match api.search(&nuke_string, 1, book::SortOption::Date) {
      Ok(_books) => _books, 
      Err(e) => return Err(e),
   };

   for book in books.iter() {
      println!("[{}] {}\n\tCover: {}\n\tTags: {}\n", book.id, book.title.pretty, book.cover, book::format_tags(&book.raw_tags, &book::TagOption::Tags).unwrap())
   }

   Ok(())
}


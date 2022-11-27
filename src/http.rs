use std::collections::HashMap;

pub async fn send_request(url: String, method: reqwest::Method, params: HashMap<String, String>) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
   let client: reqwest::Client = reqwest::Client::new();
   
   let mut req = client.request(method, url);
   req = req.header("User-Agent", "daihentai v0.1");

   for (k, v) in params {
      req = req.query(&[(k, v)]);
   }

   let res = client.execute(req.build()?).await?;
   Ok(res)
}
use std::error::Error;
fn get_articles(url:&str)-> Result<Articles,Box<dyn Error>>{
    let reponse = ureq::get(url).call()?.into_string();
    dbg!(reponse);
    todo!()
}
struct Articles {
    articles : Vec<Article>
}
struct Article {

}



fn main() {
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=e8c250e803ed41a89156fe9f351b8acc";
    let aricles= get_articles(url);
}

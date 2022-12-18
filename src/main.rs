use std::error::Error;
use serde::Deserialize;
use ureq::Response;
use colour::{dark_green,yellow};

fn get_articles(url:&str)-> Result<Articles,Box<dyn Error>>{
    let response = ureq::get(url).call()?.into_string()?;
    let articles:Articles = serde_json::from_str(&response)?;

    Ok(articles)
    
}
#[derive(Deserialize,Debug)]
struct Articles {
    articles : Vec<Article>
}
#[derive(Deserialize,Debug)]
struct Article {
    title : String,
    url:String,
}
fn render_articles(articles : &Articles){
        for a in &articles.articles {
            dark_green!("> {}\n",a.title);
            yellow!("> {}\n",a.url);
        }
}


fn main() -> Result<(),Box<dyn Error>> {
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=e8c250e803ed41a89156fe9f351b8acc";
    let articles= get_articles(url)?;
    render_articles(&articles);
    Ok(())
}

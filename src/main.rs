use std::error::Error;
use dotenv::dotenv;
use colour::{dark_green,yellow};
use newsapi::{Articles, get_articles};



fn render_articles(articles : &Articles){
        for a in &articles.articles {
            dark_green!("> {}\n",a.title);
            yellow!("> {}\n\n",a.url);
        }
}


fn main() -> Result<(),Box<dyn Error>> {
    dotenv()?;
    let api_key = std::env::var("API_KEY")?;
    let url = format!("{}{}","https://newsapi.org/v2/top-headlines?country=us&apiKey=",api_key);
    let articles= get_articles(&url)?;
    render_articles(&articles);
    Ok(())
}

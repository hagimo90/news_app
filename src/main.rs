use std::error::Error;

use colour::{dark_green,yellow};
use newsapi::{Articles, get_articles};



fn render_articles(articles : &Articles){
        for a in &articles.articles {
            dark_green!("> {}\n",a.title);
            yellow!("> {}\n\n",a.url);
        }
}


fn main() -> Result<(),Box<dyn Error>> {
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=e8c250e803ed41a89156fe9f351b8acc";
    let articles= get_articles(url)?;
    render_articles(&articles);
    Ok(())
}

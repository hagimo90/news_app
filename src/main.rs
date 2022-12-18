mod theme;
use std::error::Error;
use dotenv::dotenv;

use newsapi::{NewsApi,Endpoint,Country, Article};


fn render_articles(articles : &Vec<Article>){
    let theme = theme::default();
    theme.print_text("~#Top Headlines \n\n");
        for a in articles {
            theme.print_text(&format!("> `{}`\n",a.title));
            theme.print_text(&format!("> `{}`\n",a.url));
            theme.print_text("------");
        }
}


fn main() -> Result<(),Box<dyn Error>> {
    dotenv()?;
    let api_key = std::env::var("API_KEY")?;

    let mut newsapi = NewsApi::new(&api_key);
    newsapi.endpoint(Endpoint::TopHeadlines).country(Country::Us);
    let news_api_response = newsapi.fetch()?;
    render_articles(&news_api_response.articles());
 

//
   // let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=";
    //let url = format!("{}{}",url,api_key);
    //let articles= get_articles(&url)?;
    //render_articles(&articles);
    Ok(())
}

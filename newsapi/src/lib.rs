
use serde::Deserialize;
#[derive(thiserror::Error,Debug)]
pub enum NewsApiError {
    #[error("Failed Fetching articles")]
    RequestFailedError(ureq::Error),
    #[error("Failed converting articles to string")]
    FailedResponseToString(std::io::Error),
    #[error("Parsing article failed")]
    FailedParsingArticle(serde_json::Error)

}
#[derive(Deserialize,Debug)]
pub struct Articles {
    pub articles : Vec<Article>
}
#[derive(Deserialize,Debug)]
pub struct Article {
    pub title : String,
    pub url:String,
}
pub fn get_articles(url:&str)-> Result<Articles,NewsApiError>{
    let response = ureq::get(url).call().map_err(|err| NewsApiError::RequestFailedError(err))?.into_string().map_err(|err| NewsApiError::FailedResponseToString(err))?;
    let articles:Articles = serde_json::from_str(&response).map_err(|err| NewsApiError::FailedParsingArticle(err))?;

    Ok(articles)
    
}

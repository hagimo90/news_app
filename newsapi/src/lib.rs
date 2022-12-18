use url::Url;
use serde::Deserialize;

const BASE_URL: &str = "https://newsapi.org/v2";


#[derive(thiserror::Error,Debug)]
pub enum NewsApiError {
    #[error("Failed Fetching articles")]
    RequestFailedError(#[from]ureq::Error),
    #[error("Failed converting articles to string")]
    FailedResponseToString(#[from]std::io::Error),
    #[error("Parsing article failed")]
    FailedParsingArticle(#[from]serde_json::Error),
    #[error("Parsing base url failed")]
    UrlParsing(#[from]url::ParseError),
    #[error("Request failed:{0}")]
    BadRequest(&'static str),

}

#[derive(Deserialize,Debug)]
pub struct Article {
    pub title : String,
    pub url:String,
}

#[derive(Deserialize)]
pub struct NewsApiResponse {
    status:String,
    pub articles :Vec<Article>,
    code : Option<String>


}
impl NewsApiResponse {
    pub fn articles(&self) -> &Vec<Article> {
        &self.articles
    }
    
}
pub enum Endpoint {
    TopHeadlines,
}
impl ToString for Endpoint {
    fn to_string(&self)-> String {
        match self {
            Self::TopHeadlines =>"top-headlines".to_string()
        }
    }
}
pub enum Country {
    Us,

}
impl ToString for Country {
    fn to_string(&self)-> String {
        match self {
            Self::Us =>"Us".to_string()
        }
    }
}
pub struct NewsApi {
    api_key:String,
    endpoint:Endpoint,
    country:Country,
}

impl NewsApi {
    pub fn new(api_key:&str) -> Self {
        NewsApi {
            api_key:api_key.to_string(),
            endpoint:Endpoint::TopHeadlines,
            country:Country::Us,
        }

    }
    pub fn endpoint(&mut self,endpoint:Endpoint)-> &mut NewsApi {
        self.endpoint= endpoint;
        self
    }
    pub fn country(&mut self,country:Country)-> &mut NewsApi {
        self.country= country;
        self
    }
    fn prepare_url(&self)-> Result<String, NewsApiError> {
        let mut url = Url::parse(BASE_URL)?;
        url.path_segments_mut().unwrap().push(&self.endpoint.to_string());
        let country = format!("country={}",self.country.to_string());
        url.set_query(Some(&country));

        Ok(url.to_string())
    }

    pub fn fetch(&self) -> Result<NewsApiResponse,NewsApiError> {
        let url = self.prepare_url()?;
        let req = ureq::get(&url).set("Authorization",&self.api_key);
        let response:NewsApiResponse = req.call()?.into_json()?;
        match response.status.as_str() {
            "ok" => return Ok(response),
            _ => return Err(map_response_err(response.code))
        }

    }


}

fn map_response_err(code:Option<String>) -> NewsApiError {
    if let Some(code) = code {
        match code.as_str() {
            "apiKeyDisable" => NewsApiError::BadRequest("Your API key has been disabled."),
            "apiKeyExhausted" => NewsApiError::BadRequest("Your API key has no more requests available."),
            "apiKeyInvalid" => NewsApiError::BadRequest("Your API key hasn't been entered correctly. Double check it and try again."),
            "apiKeyMissing" => NewsApiError::BadRequest("Your API key is missing from the request."),
            "parameterInvalid" => NewsApiError::BadRequest("You've included a parameter in your request which is currently not supported. "),
            "parametersMissing" => NewsApiError::BadRequest("Required parameters are missing from the request and it cannot be completed. "),
            "rateLimited" => NewsApiError::BadRequest("You have been rate limited. Back off for a while before trying the request again."),
            "sourcesTooMany" => NewsApiError::BadRequest("You have requested too many sources in a single request. Try splitting the request into 2 smaller requests."),
            "sourceDoesNotExist" => NewsApiError::BadRequest("You have requested a source which does not exist."),
            _ => NewsApiError::BadRequest("This shouldn't happen, and if it does then it's our fault, not yours. Try the request again shortly."),


        }
    } else {
        NewsApiError::BadRequest("This shouldn't happen, and if it does then it's our fault, not yours. Try the request again shortly.")
    }
}

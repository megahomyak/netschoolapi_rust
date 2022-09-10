#[derive(Debug)]
pub struct UrlEndingWithASlash(reqwest::Url);

#[derive(Debug)]
pub enum CreationError {
    UrlDoesNotEndWithASlash,
}

pub enum ConversionError {
    UrlDoesNotEndWithASlash,
    ParseError(url::ParseError),
}

impl From<url::ParseError> for ConversionError {
    fn from(parse_error: url::ParseError) -> Self {
        Self::ParseError(parse_error)
    }
}

impl UrlEndingWithASlash {
    pub fn new(url: reqwest::Url) -> Result<Self, (CreationError, reqwest::Url)> {
        if url.as_str().ends_with('/') {
            Ok(Self(url))
        } else {
            Err((CreationError::UrlDoesNotEndWithASlash, url))
        }
    }

    pub const fn inner(&self) -> &reqwest::Url {
        &self.0
    }
}

impl From<reqwest::Url> for UrlEndingWithASlash {
    fn from(url: reqwest::Url) -> Self {
        Self::new(url).unwrap_or_else(|(_error, url)| {
            Self::new(reqwest::Url::parse(&(url.as_str().to_owned() + "/")).unwrap()).unwrap()
        })
    }
}

impl TryFrom<&str> for UrlEndingWithASlash {
    type Error = ConversionError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        if string.ends_with('/') {
            Ok(Self::new(reqwest::Url::parse(string)?).unwrap())
        } else {
            Err(ConversionError::UrlDoesNotEndWithASlash)
        }
    }
}

use super::url_ending_with_a_slash::UrlEndingWithASlash;

pub struct UrlWithApiBaseRoute(UrlEndingWithASlash);

impl UrlWithApiBaseRoute {
    pub const fn inner(&self) -> &reqwest::Url {
        self.0.inner()
    }
}

// Allowing `fallible_impl_from` because `StrNum` contains a number by definition.
#[allow(clippy::fallible_impl_from)]
impl From<UrlEndingWithASlash> for UrlWithApiBaseRoute {
    fn from(url: UrlEndingWithASlash) -> Self {
        Self(UrlEndingWithASlash::new(url.inner().join("webapi/").unwrap()).unwrap())
    }
}

use reqwest::RequestBuilder;

pub(super) trait Auth {
    fn auth<S>(self, token: S) -> Self
    where
        S: AsRef<str>;
}

impl Auth for RequestBuilder {
    fn auth<S>(self, token: S) -> Self
    where
        S: AsRef<str>,
    {
        self.bearer_auth(token.as_ref())
    }
}

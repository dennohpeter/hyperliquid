use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Response,
};
use serde::{de::DeserializeOwned, ser::Serialize};

use crate::{error::Result, types::API};

pub struct Client {
    inner_client: reqwest::Client,
    host: String,
}

impl Client {
    pub fn new(host: String) -> Self {
        Self {
            inner_client: reqwest::Client::new(),
            host,
        }
    }
}

impl Client {
    pub async fn post<T: DeserializeOwned>(
        &self,
        endpoint: &API,
        req: &impl Serialize,
    ) -> Result<T> {
        let url = &format!("{}{}", self.host, String::from(endpoint));

        let response = self
            .inner_client
            .post(url)
            .headers(self.build_headers())
            .json(req)
            .send()
            .await?;

        self.handler(response).await
    }
}

impl Client {
    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    }

    async fn handler<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        response.json::<T>().await.map_err(Into::into)
    }
}

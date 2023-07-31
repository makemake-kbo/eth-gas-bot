use reqwest::Client;
use url::Url;

use crate::etherscan::types::{GasResult, Root};

#[derive(Default, Debug, Clone)]
pub struct Etherscan {
    client: Client,
    url: String,
    token: String,
}

#[allow(dead_code)]
impl Etherscan {
    pub fn new(url: String, token: String) -> Self {
        Self {
            client: Client::new(),
            url: Url::parse(&url).expect("Your url is invalid!").into(),
            token: token,
        }
    }

    pub async fn get_gas(self) -> Result<GasResult, Box<dyn std::error::Error>> {
		let url = format!("{}/api?module=gastracker&action=gasoracle&apikey={}", self.url, self.token);
		let res = self.client.get(&url).send().await?;
		let res = res.json::<Root>().await?;
		Ok(res.result)
	}
}

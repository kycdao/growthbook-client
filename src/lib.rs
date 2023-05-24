mod error;
mod types;

pub use error::*;
use json_api_client::*;
pub use types::*;

pub use json_api_client::{AccessToken, AuthorizationCode, RefreshToken, StandardToken, Token};

pub struct Client {
    api: ApiClient,
    sdk_token: String,
}

impl Client {
    pub fn new(api_url: &str, sdk_token: &str) -> Result<Client> {
        let client = ApiClient::new(api_url, AuthConfig::NoAuth, None)?;

        Ok(Client {
            api: client,
            sdk_token: sdk_token.to_owned(),
        })
    }

    async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: JsonResponse,
    {
        self.api.get(path, None, None).await.map_err(Error::from)
    }

    pub async fn get_features(&self) -> Result<FeaturesResponse> {
        let path = format!("features/{}", self.sdk_token);
        self.get(&path).await
    }

    pub async fn get_feature(&self, name: &str) -> Result<Option<Feature>> {
        let response = self.get_features().await?;
        let res: Option<Feature> = response.features.get(name).map(|f| f.to_owned());
        Ok(res)
    }

    pub async fn is_enabled(&self, name: &str) -> Result<bool> {
        let feature_opt = self.get_feature(name).await?;

        let enabled = match feature_opt {
            Some(feature) => {
                if let Some(rules) = feature.rules {
                    let forced = rules.iter().any(|f| f.force);
                    match forced {
                        true => true,
                        false => feature.default_value,
                    }
                } else {
                    true
                }
            },
            None => false,
        };

        Ok(enabled)
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    // put unittests here
}

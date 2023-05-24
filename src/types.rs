use serde::{Deserialize, Serialize};

use json_api_client::types::*;
use std::collections::HashMap;

pub type Features = HashMap<String, Feature>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeaturesResponse {
    pub features: Features,
    #[serde(with = "time::serde::rfc3339")]
    pub date_updated: DateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    pub default_value: bool,
    pub rules: Option<Vec<Rule>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub force: bool,
}

// Update Cloud Cost Management AWS CUR config returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_cloud_cost_management::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = AwsCURConfigPatchRequest::new(AwsCURConfigPatchData::new(
        AwsCURConfigPatchRequestAttributes::new(true),
        AwsCURConfigPatchRequestType::AWS_CUR_CONFIG_PATCH_REQUEST,
    ));
    let configuration = Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api.update_cost_awscur_config("100".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

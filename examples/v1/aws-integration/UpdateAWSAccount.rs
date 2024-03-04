// Update an AWS integration returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_aws_integration::AWSIntegrationAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        AWSAccount::new()
            .account_id("163662907100".to_string())
            .account_specific_namespace_rules(
                std::collections::BTreeMap::from([("auto_scaling".to_string(), false)]),
            )
            .cspm_resource_collection_enabled(false)
            .excluded_regions(vec!["us-east-1".to_string(), "us-west-2".to_string()])
            .filter_tags(vec!["$KEY:$VALUE".to_string()])
            .host_tags(vec!["$KEY:$VALUE".to_string()])
            .metrics_collection_enabled(true)
            .resource_collection_enabled(true)
            .role_name("DatadogAWSIntegrationRole".to_string());
    let configuration = Configuration::new();
    let api = AWSIntegrationAPI::with_config(configuration);
    let resp = api.update_aws_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

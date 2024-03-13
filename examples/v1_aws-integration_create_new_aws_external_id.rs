// Generate a new external ID returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_aws_integration::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = AWSAccount::new()
        .account_id("123456789012".to_string())
        .account_specific_namespace_rules(BTreeMap::from([
            ("auto_scaling".to_string(), false),
            ("opswork".to_string(), false),
        ]))
        .cspm_resource_collection_enabled(true)
        .excluded_regions(vec!["us-east-1".to_string(), "us-west-2".to_string()])
        .filter_tags(vec!["$KEY:$VALUE".to_string()])
        .host_tags(vec!["$KEY:$VALUE".to_string()])
        .metrics_collection_enabled(false)
        .resource_collection_enabled(true)
        .role_name("DatadogAWSIntegrationRole".to_string());
    let configuration = Configuration::new();
    let api = AWSIntegrationAPI::with_config(configuration);
    let resp = api.create_new_aws_external_id(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
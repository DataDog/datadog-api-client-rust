// Delete an Azure integration returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_azure_integration::AzureIntegrationAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body = AzureAccount::new().client_id("".to_string()).tenant_name("".to_string());
    let configuration = Configuration::new();
    let api = AzureIntegrationAPI::with_config(configuration);
    let resp = api.delete_azure_integration(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

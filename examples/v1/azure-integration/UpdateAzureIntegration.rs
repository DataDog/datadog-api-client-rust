// Update an Azure integration returns "OK" response
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
    let body =
        AzureAccount::new()
            .app_service_plan_filters("key:value,filter:example".to_string())
            .automute(true)
            .client_id("".to_string())
            .client_secret("testingx./Sw*g/Y33t..R1cH+hScMDt".to_string())
            .container_app_filters("key:value,filter:example".to_string())
            .cspm_enabled(true)
            .custom_metrics_enabled(true)
            .errors(vec!["*".to_string()])
            .host_filters("key:value,filter:example".to_string())
            .new_client_id("".to_string())
            .new_tenant_name("".to_string())
            .resource_collection_enabled(true)
            .tenant_name("".to_string());
    let configuration = Configuration::new();
    let api = AzureIntegrationAPI::with_config(configuration);
    let resp = api.update_azure_integration(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Update Cloud Cost Management Azure config returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        AzureUCConfigPatchRequest::new(
            AzureUCConfigPatchData::new(
                AzureUCConfigPatchRequestAttributes::new(true),
                AzureUCConfigPatchRequestType::AZURE_UC_CONFIG_PATCH_REQUEST,
            ),
        );
    let configuration = Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api.update_cost_azure_uc_configs(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

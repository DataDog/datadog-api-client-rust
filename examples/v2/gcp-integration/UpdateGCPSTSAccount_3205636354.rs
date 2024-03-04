// Update STS Service Account returns "OK" response with enable resource collection turned on
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_gcp_integration::GCPIntegrationAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "gcp_sts_account" in the system
    let gcp_sts_account_data_id = std::env::var("GCP_STS_ACCOUNT_DATA_ID").unwrap();
    let body =
        GCPSTSServiceAccountUpdateRequest
        ::new().data(
            GCPSTSServiceAccountUpdateRequestData::new()
                .attributes(
                    GCPSTSServiceAccountAttributes::new()
                        .client_email("252bf553ef04b351@example.com".to_string())
                        .resource_collection_enabled(true),
                )
                .id(gcp_sts_account_data_id)
                .type_(GCPServiceAccountType::GCP_SERVICE_ACCOUNT),
        );
    let configuration = Configuration::new();
    let api = GCPIntegrationAPI::with_config(configuration);
    let resp = api.update_gcpsts_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

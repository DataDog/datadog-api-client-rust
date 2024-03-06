// Create a new entry for your service account with resource collection enabled returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_gcp_integration::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body =
        GCPSTSServiceAccountCreateRequest
        ::new().data(
            GCPSTSServiceAccountData::new()
                .attributes(
                    GCPSTSServiceAccountAttributes::new()
                        .client_email("252bf553ef04b351@test-project.iam.gserviceaccount.com".to_string())
                        .host_filters(vec![])
                        .resource_collection_enabled(true),
                )
                .type_(GCPServiceAccountType::GCP_SERVICE_ACCOUNT),
        );
    let configuration = Configuration::new();
    let api = GCPIntegrationAPI::with_config(configuration);
    let resp = api.create_gcpsts_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

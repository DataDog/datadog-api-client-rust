// List ownership inference history for a resource returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_csm_ownership::CSMOwnershipAPI;
use datadog_api_client::datadogV2::api_csm_ownership::ListOwnershipHistoryOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListOwnershipHistory", true);
    let api = CSMOwnershipAPI::with_config(configuration);
    let resp = api
        .list_ownership_history(
            "res-1".to_string(),
            ListOwnershipHistoryOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

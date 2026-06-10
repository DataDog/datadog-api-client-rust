// List ownership history by owner type returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_csm_ownership::CSMOwnershipAPI;
use datadog_api_client::datadogV2::api_csm_ownership::ListOwnershipHistoryByOwnerTypeOptionalParams;
use datadog_api_client::datadogV2::model::OwnershipOwnerType;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListOwnershipHistoryByOwnerType", true);
    let api = CSMOwnershipAPI::with_config(configuration);
    let resp = api
        .list_ownership_history_by_owner_type(
            "res-1".to_string(),
            OwnershipOwnerType::TEAM,
            ListOwnershipHistoryByOwnerTypeOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

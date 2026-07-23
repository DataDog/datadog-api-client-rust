// Update ownership settings for the org returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_csm_ownership::CSMOwnershipAPI;
use datadog_api_client::datadogV2::model::OwnershipConfidenceLevel;
use datadog_api_client::datadogV2::model::OwnershipSettingsRequest;
use datadog_api_client::datadogV2::model::OwnershipSettingsRequestAttributes;
use datadog_api_client::datadogV2::model::OwnershipSettingsRequestData;
use datadog_api_client::datadogV2::model::OwnershipSettingsType;

#[tokio::main]
async fn main() {
    let body = OwnershipSettingsRequest::new(OwnershipSettingsRequestData::new(
        OwnershipSettingsRequestAttributes::new(true, OwnershipConfidenceLevel::HIGH),
        OwnershipSettingsType::OWNERSHIP_SETTINGS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.PostOwnershipSettings", true);
    let api = CSMOwnershipAPI::with_config(configuration);
    let resp = api.post_ownership_settings(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

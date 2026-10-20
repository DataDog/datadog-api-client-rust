// Patch a saved filter returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;
use datadog_api_client::datadogV2::model::PatchSavedFilterAttributes;
use datadog_api_client::datadogV2::model::PatchSavedFilterData;
use datadog_api_client::datadogV2::model::PatchSavedFilterRequest;
use datadog_api_client::datadogV2::model::SavedFilterDataType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = PatchSavedFilterRequest::new(PatchSavedFilterData::new(
        PatchSavedFilterAttributes::new()
            .name(Some("patchSavedFilterTest-renamed-157e7ff3".to_string())),
        SavedFilterDataType::SAVED_FILTERS,
    ));
    let configuration = datadog::Configuration::new();
    let api = FeatureFlagsAPI::with_config(configuration);
    let resp = api
        .update_saved_filter_attributes(
            Uuid::parse_str("2cfc7949-7eb4-4d76-b1b8-d1aa9b565535").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

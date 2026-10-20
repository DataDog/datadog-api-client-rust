// Patch a saved filter updates only the description of a saved list
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
            .description(Some("Updated synthetic test description.".to_string())),
        SavedFilterDataType::SAVED_FILTERS,
    ));
    let configuration = datadog::Configuration::new();
    let api = FeatureFlagsAPI::with_config(configuration);
    let resp = api
        .update_saved_filter_attributes(
            Uuid::parse_str("d8e3a81b-1505-4600-99c6-c563d01d46b1").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

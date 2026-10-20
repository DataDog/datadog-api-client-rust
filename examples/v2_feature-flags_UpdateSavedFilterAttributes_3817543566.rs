// Patch a saved filter accepts the current timestamp for metadata
use chrono::{DateTime, Utc};
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
            .description(Some("Metadata with current timestamp.".to_string()))
            .last_updated_at(Some(
                DateTime::parse_from_rfc3339("2026-10-16T20:20:29+00:00")
                    .expect("Failed to parse datetime")
                    .with_timezone(&Utc),
            )),
        SavedFilterDataType::SAVED_FILTERS,
    ));
    let configuration = datadog::Configuration::new();
    let api = FeatureFlagsAPI::with_config(configuration);
    let resp = api
        .update_saved_filter_attributes(
            Uuid::parse_str("a8c393b4-3a9c-48fd-aa48-402c97e8d9c9").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

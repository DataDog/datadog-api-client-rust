// Set the approval requirement for a feature flag returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;
use datadog_api_client::datadogV2::model::FeatureFlagApprovalsDataType;
use datadog_api_client::datadogV2::model::UpdateFeatureFlagApprovalsAttributes;
use datadog_api_client::datadogV2::model::UpdateFeatureFlagApprovalsData;
use datadog_api_client::datadogV2::model::UpdateFeatureFlagApprovalsRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = UpdateFeatureFlagApprovalsRequest::new(UpdateFeatureFlagApprovalsData::new(
        UpdateFeatureFlagApprovalsAttributes::new(true),
        FeatureFlagApprovalsDataType::FEATURE_FLAG_APPROVALS,
    ));
    let configuration = datadog::Configuration::new();
    let api = FeatureFlagsAPI::with_config(configuration);
    let resp = api
        .update_feature_flag_approvals(
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

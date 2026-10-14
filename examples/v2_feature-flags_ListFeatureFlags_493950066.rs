// List feature flags filters disabled production status
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;
use datadog_api_client::datadogV2::api_feature_flags::ListFeatureFlagsOptionalParams;
use datadog_api_client::datadogV2::model::FeatureFlagProductionStatus;

#[tokio::main]
async fn main() {
    // there is a valid "feature_flag_list_a" in the system
    let feature_flag_list_a_data_id =
        uuid::Uuid::parse_str(&std::env::var("FEATURE_FLAG_LIST_A_DATA_ID").unwrap())
            .expect("Invalid UUID");

    // there is a valid "feature_flag_list_b" in the system
    let feature_flag_list_b_data_id =
        uuid::Uuid::parse_str(&std::env::var("FEATURE_FLAG_LIST_B_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let configuration = datadog::Configuration::new();
    let api = FeatureFlagsAPI::with_config(configuration);
    let resp = api
        .list_feature_flags(
            ListFeatureFlagsOptionalParams::default()
                .flag_ids(vec![
                    feature_flag_list_a_data_id.clone(),
                    feature_flag_list_b_data_id.clone(),
                ])
                .production_status(vec![FeatureFlagProductionStatus::DISABLED]),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

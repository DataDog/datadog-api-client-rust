// List feature flags combines exact keys tags search types creators and IDs
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;
use datadog_api_client::datadogV2::api_feature_flags::ListFeatureFlagsOptionalParams;
use datadog_api_client::datadogV2::model::ValueType;

#[tokio::main]
async fn main() {
    // there is a valid "feature_flag_list_a" in the system
    let feature_flag_list_a_data_attributes_created_by = uuid::Uuid::parse_str(
        &std::env::var("FEATURE_FLAG_LIST_A_DATA_ATTRIBUTES_CREATED_BY").unwrap(),
    )
    .expect("Invalid UUID");
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
                .key("list-a-example-feature-flag".to_string())
                .tags(vec![
                    "list-test:example-feature-flag".to_string(),
                    "color:blue".to_string(),
                ])
                .search("LIST-A-example-feature-flag".to_string())
                .value_type(vec![ValueType::BOOLEAN, ValueType::STRING])
                .created_by(vec![feature_flag_list_a_data_attributes_created_by.clone()])
                .flag_ids(vec![
                    feature_flag_list_a_data_id.clone(),
                    feature_flag_list_b_data_id.clone(),
                ])
                .is_archived(false),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

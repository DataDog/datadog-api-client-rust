// Update targeting rules for a flag in an environment returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;
use datadog_api_client::datadogV2::model::AllocationDataRequest;
use datadog_api_client::datadogV2::model::AllocationDataType;
use datadog_api_client::datadogV2::model::AllocationType;
use datadog_api_client::datadogV2::model::ExposureRolloutStepRequest;
use datadog_api_client::datadogV2::model::ExposureScheduleRequest;
use datadog_api_client::datadogV2::model::OverwriteAllocationsRequest;
use datadog_api_client::datadogV2::model::RolloutOptionsRequest;
use datadog_api_client::datadogV2::model::RolloutStrategy;
use datadog_api_client::datadogV2::model::UpsertAllocationRequest;
use datadog_api_client::datadogV2::model::VariantWeightRequest;

#[tokio::main]
async fn main() {
    // there is a valid "feature_flag" in the system
    let feature_flag_data_attributes_variants_0_id = uuid::Uuid::parse_str(
        &std::env::var("FEATURE_FLAG_DATA_ATTRIBUTES_VARIANTS_0_ID").unwrap(),
    )
    .expect("Invalid UUID");
    let feature_flag_data_id =
        uuid::Uuid::parse_str(&std::env::var("FEATURE_FLAG_DATA_ID").unwrap())
            .expect("Invalid UUID");

    // there is a valid "environment" in the system
    let environment_data_id = uuid::Uuid::parse_str(&std::env::var("ENVIRONMENT_DATA_ID").unwrap())
        .expect("Invalid UUID");
    let body = OverwriteAllocationsRequest::new(vec![AllocationDataRequest::new(
        UpsertAllocationRequest::new(
            "overwrite-allocation-example-feature-flag".to_string(),
            "New targeting rule Example-Feature-Flag".to_string(),
            AllocationType::CANARY,
        )
        .exposure_schedule(ExposureScheduleRequest::new(
            RolloutOptionsRequest::new(RolloutStrategy::UNIFORM_INTERVALS)
                .autostart(Some(false))
                .selection_interval_ms(86400000),
            vec![
                ExposureRolloutStepRequest::new(0.05, 0, false).interval_ms(None),
                ExposureRolloutStepRequest::new(0.25, 1, false).interval_ms(None),
                ExposureRolloutStepRequest::new(1.0, 2, false).interval_ms(None),
            ],
        ))
        .guardrail_metrics(vec![])
        .targeting_rules(vec![])
        .variant_weights(vec![VariantWeightRequest::new(100.0)
            .variant_id(feature_flag_data_attributes_variants_0_id.clone())]),
        AllocationDataType::ALLOCATIONS,
    )]);
    let configuration = datadog::Configuration::new();
    let api = FeatureFlagsAPI::with_config(configuration);
    let resp = api
        .update_allocations_for_feature_flag_in_environment(
            feature_flag_data_id.clone(),
            environment_data_id.clone(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

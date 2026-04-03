// Update targeting rules for a flag returns "OK" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;
use datadog_api_client::datadogV2::model::AllocationDataRequest;
use datadog_api_client::datadogV2::model::AllocationDataType;
use datadog_api_client::datadogV2::model::AllocationType;
use datadog_api_client::datadogV2::model::ConditionOperator;
use datadog_api_client::datadogV2::model::ConditionRequest;
use datadog_api_client::datadogV2::model::ExposureRolloutStepRequest;
use datadog_api_client::datadogV2::model::ExposureScheduleRequest;
use datadog_api_client::datadogV2::model::GuardrailMetricRequest;
use datadog_api_client::datadogV2::model::GuardrailTriggerAction;
use datadog_api_client::datadogV2::model::OverwriteAllocationsRequest;
use datadog_api_client::datadogV2::model::RolloutOptionsRequest;
use datadog_api_client::datadogV2::model::RolloutStrategy;
use datadog_api_client::datadogV2::model::TargetingRuleRequest;
use datadog_api_client::datadogV2::model::UpsertAllocationRequest;
use datadog_api_client::datadogV2::model::VariantWeightRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = OverwriteAllocationsRequest::new(vec![AllocationDataRequest::new(
        UpsertAllocationRequest::new(
            "prod-rollout".to_string(),
            "Production Rollout".to_string(),
            AllocationType::FEATURE_GATE,
        )
        .experiment_id(Some("550e8400-e29b-41d4-a716-446655440030".to_string()))
        .exposure_schedule(
            ExposureScheduleRequest::new(
                RolloutOptionsRequest::new(RolloutStrategy::UNIFORM_INTERVALS)
                    .autostart(Some(false))
                    .selection_interval_ms(3600000),
                vec![ExposureRolloutStepRequest::new(0.5, 1, false)
                    .id(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440040")
                        .expect("invalid UUID"))
                    .interval_ms(Some(3600000))],
            )
            .absolute_start_time(Some(
                DateTime::parse_from_rfc3339("2025-06-13T12:00:00+00:00")
                    .expect("Failed to parse datetime")
                    .with_timezone(&Utc),
            ))
            .control_variant_id(Some("550e8400-e29b-41d4-a716-446655440012".to_string()))
            .control_variant_key(Some("control".to_string()))
            .id(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440010").expect("invalid UUID")),
        )
        .guardrail_metrics(vec![GuardrailMetricRequest::new(
            "metric-error-rate".to_string(),
            GuardrailTriggerAction::PAUSE,
        )])
        .id(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440020").expect("invalid UUID"))
        .targeting_rules(vec![TargetingRuleRequest::new(vec![
            ConditionRequest::new(
                "user_tier".to_string(),
                ConditionOperator::ONE_OF,
                vec!["premium".to_string(), "enterprise".to_string()],
            ),
        ])])
        .variant_weights(vec![VariantWeightRequest::new(50.0)
            .variant_id(
                Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").expect("invalid UUID"),
            )
            .variant_key("control".to_string())]),
        AllocationDataType::ALLOCATIONS,
    )]);
    let configuration = datadog::Configuration::new();
    let api = FeatureFlagsAPI::with_config(configuration);
    let resp = api
        .update_allocations_for_feature_flag_in_environment(
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").expect("invalid UUID"),
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

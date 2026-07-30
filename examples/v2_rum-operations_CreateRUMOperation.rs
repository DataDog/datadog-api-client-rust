// Create a RUM operation returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_operations::RUMOperationsAPI;
use datadog_api_client::datadogV2::model::RUMOperationCreateRequest;
use datadog_api_client::datadogV2::model::RUMOperationCreateRequestData;
use datadog_api_client::datadogV2::model::RUMOperationJourneyCompositeRule;
use datadog_api_client::datadogV2::model::RUMOperationJourneyCompositeRuleKind;
use datadog_api_client::datadogV2::model::RUMOperationJourneyNode;
use datadog_api_client::datadogV2::model::RUMOperationJourneyPredicate;
use datadog_api_client::datadogV2::model::RUMOperationJourneyRum;
use datadog_api_client::datadogV2::model::RUMOperationJourneyStep;
use datadog_api_client::datadogV2::model::RUMOperationJourneyStepType;
use datadog_api_client::datadogV2::model::RUMOperationRequestAttributes;
use datadog_api_client::datadogV2::model::RUMOperationType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = RUMOperationCreateRequest::new(RUMOperationCreateRequestData::new(
        RUMOperationRequestAttributes::new(
            RUMOperationJourneyRum::new(vec![RUMOperationJourneyStep::new(
                RUMOperationJourneyStepType::START,
            )
            .composite(
                RUMOperationJourneyCompositeRule::new(
                    RUMOperationJourneyCompositeRuleKind::ALL_OF,
                    vec![RUMOperationJourneyPredicate::new(
                        "@type:action @action.type:click".to_string(),
                    )],
                )
                .max_window_ms(30000),
            )
            .nodes(vec![RUMOperationJourneyNode::new(
                "@type:action @action.type:click".to_string(),
            )])]),
            "checkout_completed".to_string(),
            vec!["team:checkout".to_string()],
        )
        .application_id(
            Uuid::parse_str("abc12345-1234-5678-abcd-ef1234567890").expect("invalid UUID"),
        )
        .category(None)
        .description(None)
        .display_name("Checkout completed".to_string())
        .feature_ids(vec![]),
        RUMOperationType::OPERATIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateRUMOperation", true);
    let api = RUMOperationsAPI::with_config(configuration);
    let resp = api.create_rum_operation(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

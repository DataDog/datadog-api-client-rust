// Create a RUM operation returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_operations::RUMOperationsAPI;
use datadog_api_client::datadogV2::model::RUMOperationCreateRequest;
use datadog_api_client::datadogV2::model::RUMOperationCreateRequestData;
use datadog_api_client::datadogV2::model::RUMOperationJourneyNode;
use datadog_api_client::datadogV2::model::RUMOperationJourneyRum;
use datadog_api_client::datadogV2::model::RUMOperationJourneyStep;
use datadog_api_client::datadogV2::model::RUMOperationJourneyStepType;
use datadog_api_client::datadogV2::model::RUMOperationRequestAttributes;
use datadog_api_client::datadogV2::model::RUMOperationType;

#[tokio::main]
async fn main() {
    let body = RUMOperationCreateRequest::new(RUMOperationCreateRequestData::new(
        RUMOperationRequestAttributes::new(
            RUMOperationJourneyRum::new(Some(vec![
                RUMOperationJourneyStep::new(RUMOperationJourneyStepType::START).nodes(vec![
                    RUMOperationJourneyNode::new("@type:action @action.type:click".to_string()),
                ]),
                RUMOperationJourneyStep::new(RUMOperationJourneyStepType::STOP)
                    .nodes(vec![RUMOperationJourneyNode::new("@type:view".to_string())]),
            ])),
            "bdd_examplerumoperation".to_string(),
            vec!["team:rum-backend".to_string()],
        )
        .display_name("BDD operation".to_string()),
        RUMOperationType::OPERATIONS,
    ));
    let configuration = datadog::Configuration::new();
    let api = RUMOperationsAPI::with_config(configuration);
    let resp = api.create_rum_operation(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

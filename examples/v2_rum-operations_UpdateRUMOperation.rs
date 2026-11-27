// Update a RUM operation returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_operations::RUMOperationsAPI;
use datadog_api_client::datadogV2::model::RUMOperationJourneyNode;
use datadog_api_client::datadogV2::model::RUMOperationJourneyRum;
use datadog_api_client::datadogV2::model::RUMOperationJourneyStep;
use datadog_api_client::datadogV2::model::RUMOperationJourneyStepType;
use datadog_api_client::datadogV2::model::RUMOperationRequestAttributes;
use datadog_api_client::datadogV2::model::RUMOperationType;
use datadog_api_client::datadogV2::model::RUMOperationUpdateRequest;
use datadog_api_client::datadogV2::model::RUMOperationUpdateRequestData;

#[tokio::main]
async fn main() {
    // there is a valid "rum_operation" in the system
    let rum_operation_data_attributes_name =
        std::env::var("RUM_OPERATION_DATA_ATTRIBUTES_NAME").unwrap();
    let rum_operation_data_id = std::env::var("RUM_OPERATION_DATA_ID").unwrap();
    let body = RUMOperationUpdateRequest::new(RUMOperationUpdateRequestData::new(
        RUMOperationRequestAttributes::new(
            RUMOperationJourneyRum::new(Some(vec![
                RUMOperationJourneyStep::new(RUMOperationJourneyStepType::START).nodes(vec![
                    RUMOperationJourneyNode::new("@type:action @action.type:click".to_string()),
                ]),
                RUMOperationJourneyStep::new(RUMOperationJourneyStepType::STOP)
                    .nodes(vec![RUMOperationJourneyNode::new("@type:view".to_string())]),
            ])),
            rum_operation_data_attributes_name.clone(),
            vec!["team:rum-backend".to_string()],
        )
        .display_name("BDD operation updated".to_string()),
        rum_operation_data_id.clone(),
        RUMOperationType::OPERATIONS,
    ));
    let configuration = datadog::Configuration::new();
    let api = RUMOperationsAPI::with_config(configuration);
    let resp = api
        .update_rum_operation(rum_operation_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

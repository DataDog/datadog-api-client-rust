// Generate data transformation code returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_workflow_automation::WorkflowAutomationAPI;
use datadog_api_client::datadogV2::model::ChatHistoryItem;
use datadog_api_client::datadogV2::model::ChatHistoryItemRole;
use datadog_api_client::datadogV2::model::DataTransformationContext;
use datadog_api_client::datadogV2::model::DataTransformationLanguage;
use datadog_api_client::datadogV2::model::DataTransformationRequest;

#[tokio::main]
async fn main() {
    let body = DataTransformationRequest::new("Convert timestamp to ISO format".to_string())
        .chat_history(vec![ChatHistoryItem::new(
            "Please add error handling".to_string(),
            ChatHistoryItemRole::USER,
        )])
        .context(DataTransformationContext::new(
            r#"{ "timestamp": 1234567890 }"#.to_string(),
            "return data.timestamp;".to_string(),
        ))
        .language(DataTransformationLanguage::JAVASCRIPT)
        .stream(true);
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateDataTransformation", true);
    let api = WorkflowAutomationAPI::with_config(configuration);
    let resp = api.create_data_transformation(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

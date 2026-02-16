// Generate data transformation description returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_workflow_automation::WorkflowAutomationAPI;
use datadog_api_client::datadogV2::model::DataTransformationDescriptionRequest;

#[tokio::main]
async fn main() {
    let body = DataTransformationDescriptionRequest::new(
        "com.datadoghq.transform.timestamp".to_string(),
        "return new Date(data.timestamp).toISOString();".to_string(),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateDataTransformationDescription", true);
    let api = WorkflowAutomationAPI::with_config(configuration);
    let resp = api.create_data_transformation_description(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

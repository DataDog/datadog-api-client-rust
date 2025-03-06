// Post AWS Scan Options returns "Agentless scan options enabled successfully."
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agentless_scanning::AgentlessScanningAPI;
use datadog_api_client::datadogV2::model::AwsScanOptionsCreateAttributes;
use datadog_api_client::datadogV2::model::AwsScanOptionsCreateData;
use datadog_api_client::datadogV2::model::AwsScanOptionsCreateRequest;
use datadog_api_client::datadogV2::model::AwsScanOptionsType;

#[tokio::main]
async fn main() {
    let body = AwsScanOptionsCreateRequest::new(AwsScanOptionsCreateData::new(
        AwsScanOptionsCreateAttributes::new(true, false, true, true),
        "000000000003".to_string(),
        AwsScanOptionsType::AWS_SCAN_OPTIONS,
    ));
    let configuration = datadog::Configuration::new();
    let api = AgentlessScanningAPI::with_config(configuration);
    let resp = api.create_aws_scan_options(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

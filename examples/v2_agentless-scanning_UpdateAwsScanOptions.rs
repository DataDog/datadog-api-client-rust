// Patch AWS Scan Options returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agentless_scanning::AgentlessScanningAPI;
use datadog_api_client::datadogV2::model::AwsScanOptionsType;
use datadog_api_client::datadogV2::model::AwsScanOptionsUpdateAttributes;
use datadog_api_client::datadogV2::model::AwsScanOptionsUpdateData;
use datadog_api_client::datadogV2::model::AwsScanOptionsUpdateRequest;

#[tokio::main]
async fn main() {
    let body = AwsScanOptionsUpdateRequest::new(AwsScanOptionsUpdateData::new(
        AwsScanOptionsUpdateAttributes::new()
            .lambda(false)
            .vuln_containers_os(true)
            .vuln_host_os(true),
        "000000000002".to_string(),
        AwsScanOptionsType::AWS_SCAN_OPTIONS,
    ));
    let configuration = datadog::Configuration::new();
    let api = AgentlessScanningAPI::with_config(configuration);
    let resp = api
        .update_aws_scan_options("000000000002".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

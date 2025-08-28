// Get AWS scan options returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agentless_scanning::AgentlessScanningAPI;

#[tokio::main]
async fn main() {
    // there is a valid "aws_scan_options" in the system
    let aws_scan_options_id = std::env::var("AWS_SCAN_OPTIONS_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = AgentlessScanningAPI::with_config(configuration);
    let resp = api.get_aws_scan_options(aws_scan_options_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

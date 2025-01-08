// Get AWS Scan Options returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agentless_scanning::AgentlessScanningAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = AgentlessScanningAPI::with_config(configuration);
    let resp = api.list_aws_scan_options().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

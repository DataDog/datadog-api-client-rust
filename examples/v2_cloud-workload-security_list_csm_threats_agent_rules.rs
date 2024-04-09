// Get all CSM Threats Agent rules returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_workload_security::CloudWorkloadSecurityAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = CloudWorkloadSecurityAPI::with_config(configuration);
    let resp = api.list_csm_threats_agent_rules().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

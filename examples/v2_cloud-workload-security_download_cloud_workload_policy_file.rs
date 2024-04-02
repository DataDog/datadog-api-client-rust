// Get the latest Cloud Workload Security policy returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api::api_cloud_workload_security::CloudWorkloadSecurityAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = CloudWorkloadSecurityAPI::with_config(configuration);
    let resp = api.download_cloud_workload_policy_file().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

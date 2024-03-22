// Get the details of a project returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_case_management::CaseManagementAPI;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api.get_project("project_id".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
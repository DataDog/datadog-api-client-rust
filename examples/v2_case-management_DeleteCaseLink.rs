// Delete a case link returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteCaseLink", true);
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api
        .delete_case_link("804cd682-55f6-4541-ab00-b608b282ea7d".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

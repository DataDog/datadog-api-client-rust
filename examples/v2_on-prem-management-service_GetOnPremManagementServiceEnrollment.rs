// Get enrollment status returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_prem_management_service::OnPremManagementServiceAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetOnPremManagementServiceEnrollment", true);
    let api = OnPremManagementServiceAPI::with_config(configuration);
    let resp = api
        .get_on_prem_management_service_enrollment(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

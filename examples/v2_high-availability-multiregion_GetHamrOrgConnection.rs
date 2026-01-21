// Get HAMR organization connection returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_high_availability_multi_region::HighAvailabilityMultiRegionAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetHamrOrgConnection", true);
    let api = HighAvailabilityMultiRegionAPI::with_config(configuration);
    let resp = api.get_hamr_org_connection().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

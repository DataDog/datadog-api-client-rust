// Get all devices returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_end_user_device_monitoring::EndUserDeviceMonitoringAPI;
use datadog_api_client::datadogV2::api_end_user_device_monitoring::GetEUDMDevicesOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.get_eudm_devices", true);
    let api = EndUserDeviceMonitoringAPI::with_config(configuration);
    let resp = api
        .get_eudm_devices(GetEUDMDevicesOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

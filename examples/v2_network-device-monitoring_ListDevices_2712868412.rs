// Get the list of devices returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_network_device_monitoring::ListDevicesOptionalParams;
use datadog_api_client::datadogV2::api_network_device_monitoring::NetworkDeviceMonitoringAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = NetworkDeviceMonitoringAPI::with_config(configuration);
    let response = api.list_devices_with_pagination(ListDevicesOptionalParams::default());
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}

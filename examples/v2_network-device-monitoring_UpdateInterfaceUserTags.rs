// Update the tags for an interface returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_network_device_monitoring::NetworkDeviceMonitoringAPI;
use datadog_api_client::datadogV2::model::ListInterfaceTagsResponse;
use datadog_api_client::datadogV2::model::ListInterfaceTagsResponseData;
use datadog_api_client::datadogV2::model::ListTagsResponseDataAttributes;

#[tokio::main]
async fn main() {
    let body = ListInterfaceTagsResponse::new().data(
        ListInterfaceTagsResponseData::new()
            .attributes(
                ListTagsResponseDataAttributes::new()
                    .tags(vec!["tag:test".to_string(), "tag:testbis".to_string()]),
            )
            .id("example:1.2.3.4:1".to_string())
            .type_("tags".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = NetworkDeviceMonitoringAPI::with_config(configuration);
    let resp = api
        .update_interface_user_tags("example:1.2.3.4:1".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

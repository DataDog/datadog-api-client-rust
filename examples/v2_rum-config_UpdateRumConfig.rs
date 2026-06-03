// Update the RUM configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_config::RumConfigAPI;
use datadog_api_client::datadogV2::model::RumConfigType;
use datadog_api_client::datadogV2::model::RumConfigUpdateAttributes;
use datadog_api_client::datadogV2::model::RumConfigUpdateData;
use datadog_api_client::datadogV2::model::RumConfigUpdateRequest;

#[tokio::main]
async fn main() {
    let body = RumConfigUpdateRequest::new(RumConfigUpdateData::new(
        RumConfigUpdateAttributes::new(true),
        RumConfigType::RUM_CONFIG,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateRumConfig", true);
    let api = RumConfigAPI::with_config(configuration);
    let resp = api.update_rum_config(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

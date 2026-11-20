// Update the RUM configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_config::RUMConfigAPI;
use datadog_api_client::datadogV2::model::RUMConfigType;
use datadog_api_client::datadogV2::model::RUMConfigUpdateAttributes;
use datadog_api_client::datadogV2::model::RUMConfigUpdateData;
use datadog_api_client::datadogV2::model::RUMConfigUpdateRequest;

#[tokio::main]
async fn main() {
    let body = RUMConfigUpdateRequest::new(RUMConfigUpdateData::new(
        RUMConfigUpdateAttributes::new(true),
        RUMConfigType::RUM_CONFIG,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_rum_config", true);
    let api = RUMConfigAPI::with_config(configuration);
    let resp = api.update_rum_config(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Create the RUM configuration returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_config::RUMConfigAPI;
use datadog_api_client::datadogV2::model::RUMConfigCreateAttributes;
use datadog_api_client::datadogV2::model::RUMConfigCreateData;
use datadog_api_client::datadogV2::model::RUMConfigCreateRequest;
use datadog_api_client::datadogV2::model::RUMConfigType;

#[tokio::main]
async fn main() {
    let body = RUMConfigCreateRequest::new(RUMConfigCreateData::new(
        RUMConfigCreateAttributes::new(true),
        RUMConfigType::RUM_CONFIG,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_rum_config", true);
    let api = RUMConfigAPI::with_config(configuration);
    let resp = api.create_rum_config(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

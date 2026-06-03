// Create the RUM configuration returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_config::RumConfigAPI;
use datadog_api_client::datadogV2::model::RumConfigCreateAttributes;
use datadog_api_client::datadogV2::model::RumConfigCreateData;
use datadog_api_client::datadogV2::model::RumConfigCreateRequest;
use datadog_api_client::datadogV2::model::RumConfigType;

#[tokio::main]
async fn main() {
    let body = RumConfigCreateRequest::new(RumConfigCreateData::new(
        RumConfigCreateAttributes::new(true),
        RumConfigType::RUM_CONFIG,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateRumConfig", true);
    let api = RumConfigAPI::with_config(configuration);
    let resp = api.create_rum_config(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

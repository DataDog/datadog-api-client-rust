// Update the Statuspage account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_statuspage_integration::StatuspageIntegrationAPI;
use datadog_api_client::datadogV2::model::StatuspageAccountType;
use datadog_api_client::datadogV2::model::StatuspageAccountUpdateAttributes;
use datadog_api_client::datadogV2::model::StatuspageAccountUpdateData;
use datadog_api_client::datadogV2::model::StatuspageAccountUpdateRequest;

#[tokio::main]
async fn main() {
    let body = StatuspageAccountUpdateRequest::new(StatuspageAccountUpdateData::new(
        StatuspageAccountUpdateAttributes::new()
            .api_key("00000000-0000-0000-0000-000000000000".to_string()),
        StatuspageAccountType::STATUSPAGE_ACCOUNT,
    ));
    let configuration = datadog::Configuration::new();
    let api = StatuspageIntegrationAPI::with_config(configuration);
    let resp = api.update_statuspage_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

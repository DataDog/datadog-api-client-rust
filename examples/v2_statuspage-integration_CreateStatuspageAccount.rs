// Create the Statuspage account returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_statuspage_integration::StatuspageIntegrationAPI;
use datadog_api_client::datadogV2::model::StatuspageAccountCreateAttributes;
use datadog_api_client::datadogV2::model::StatuspageAccountCreateData;
use datadog_api_client::datadogV2::model::StatuspageAccountCreateRequest;
use datadog_api_client::datadogV2::model::StatuspageAccountType;

#[tokio::main]
async fn main() {
    let body = StatuspageAccountCreateRequest::new(StatuspageAccountCreateData::new(
        StatuspageAccountCreateAttributes::new("00000000-0000-0000-0000-000000000000".to_string()),
        StatuspageAccountType::STATUSPAGE_ACCOUNT,
    ));
    let configuration = datadog::Configuration::new();
    let api = StatuspageIntegrationAPI::with_config(configuration);
    let resp = api.create_statuspage_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

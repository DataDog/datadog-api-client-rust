// Create a Statuspage URL setting returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_statuspage_integration::StatuspageIntegrationAPI;
use datadog_api_client::datadogV2::model::StatuspageUrlSettingCreateAttributes;
use datadog_api_client::datadogV2::model::StatuspageUrlSettingCreateData;
use datadog_api_client::datadogV2::model::StatuspageUrlSettingCreateRequest;
use datadog_api_client::datadogV2::model::StatuspageUrlSettingType;

#[tokio::main]
async fn main() {
    let body = StatuspageUrlSettingCreateRequest::new(StatuspageUrlSettingCreateData::new(
        StatuspageUrlSettingCreateAttributes::new(
            "team:collaboration-integrations,env:prod".to_string(),
            "https://example.statuspage.io".to_string(),
        ),
        StatuspageUrlSettingType::STATUSPAGE_URL_SETTING,
    ));
    let configuration = datadog::Configuration::new();
    let api = StatuspageIntegrationAPI::with_config(configuration);
    let resp = api.create_statuspage_url_setting(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Update a Statuspage URL setting returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_statuspage_integration::StatuspageIntegrationAPI;
use datadog_api_client::datadogV2::model::StatuspageUrlSettingType;
use datadog_api_client::datadogV2::model::StatuspageUrlSettingUpdateAttributes;
use datadog_api_client::datadogV2::model::StatuspageUrlSettingUpdateData;
use datadog_api_client::datadogV2::model::StatuspageUrlSettingUpdateRequest;

#[tokio::main]
async fn main() {
    let body = StatuspageUrlSettingUpdateRequest::new(StatuspageUrlSettingUpdateData::new(
        StatuspageUrlSettingUpdateAttributes::new()
            .custom_tags("team:collaboration-integrations,env:prod".to_string())
            .url("https://example.statuspage.io".to_string()),
        "596da4af-0563-4097-90ff-07230c3f9db3".to_string(),
        StatuspageUrlSettingType::STATUSPAGE_URL_SETTING,
    ));
    let configuration = datadog::Configuration::new();
    let api = StatuspageIntegrationAPI::with_config(configuration);
    let resp = api
        .update_statuspage_url_setting("statuspage_url_setting_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

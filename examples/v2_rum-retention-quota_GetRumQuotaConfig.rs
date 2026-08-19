// Get a RUM retention quota configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_quota::RUMRetentionQuotaAPI;
use datadog_api_client::datadogV2::model::RumRetentionQuotaScopeType;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RUMRetentionQuotaAPI::with_config(configuration);
    let resp = api
        .get_rum_quota_config(
            RumRetentionQuotaScopeType::APPLICATION,
            "ced16651-97b6-4e67-8590-8caec3af0695".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

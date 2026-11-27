// Get a RUM retention quota configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_quotas::RUMRetentionQuotasAPI;
use datadog_api_client::datadogV2::model::RumRetentionQuotaScopeType;

#[tokio::main]
async fn main() {
    // there is a valid "rum_application" in the system
    let rum_application_data_id = std::env::var("RUM_APPLICATION_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = RUMRetentionQuotasAPI::with_config(configuration);
    let resp = api
        .get_rum_quota_config(
            RumRetentionQuotaScopeType::APPLICATION,
            rum_application_data_id.clone(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

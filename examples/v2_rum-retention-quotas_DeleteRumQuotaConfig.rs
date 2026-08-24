// Delete a RUM retention quota configuration returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_quotas::RUMRetentionQuotasAPI;
use datadog_api_client::datadogV2::model::RumRetentionQuotaScopeType;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RUMRetentionQuotasAPI::with_config(configuration);
    let resp = api
        .delete_rum_quota_config(
            RumRetentionQuotaScopeType::APPLICATION,
            "cd73a516-a481-4af5-8352-9b577465c77b".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

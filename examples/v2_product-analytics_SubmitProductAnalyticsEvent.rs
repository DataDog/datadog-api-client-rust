// Send server-side events returns "Request accepted for processing (always 202
// empty JSON)." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_product_analytics::ProductAnalyticsAPI;
use datadog_api_client::datadogV2::model::ProductAnalyticsServerSideEventItem;
use datadog_api_client::datadogV2::model::ProductAnalyticsServerSideEventItemAccount;
use datadog_api_client::datadogV2::model::ProductAnalyticsServerSideEventItemApplication;
use datadog_api_client::datadogV2::model::ProductAnalyticsServerSideEventItemEvent;
use datadog_api_client::datadogV2::model::ProductAnalyticsServerSideEventItemSession;
use datadog_api_client::datadogV2::model::ProductAnalyticsServerSideEventItemType;
use datadog_api_client::datadogV2::model::ProductAnalyticsServerSideEventItemUsr;

#[tokio::main]
async fn main() {
    let body = ProductAnalyticsServerSideEventItem::new(
        ProductAnalyticsServerSideEventItemApplication::new(
            "123abcde-123a-123b-1234-123456789abc".to_string(),
        ),
        ProductAnalyticsServerSideEventItemEvent::new("payment.processed".to_string()),
        ProductAnalyticsServerSideEventItemType::SERVER,
    )
    .account(ProductAnalyticsServerSideEventItemAccount::new(
        "account-67890".to_string(),
    ))
    .session(ProductAnalyticsServerSideEventItemSession::new(
        "session-abcdef".to_string(),
    ))
    .usr(ProductAnalyticsServerSideEventItemUsr::new(
        "user-12345".to_string(),
    ));
    let configuration = datadog::Configuration::new();
    let api = ProductAnalyticsAPI::with_config(configuration);
    let resp = api.submit_product_analytics_event(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

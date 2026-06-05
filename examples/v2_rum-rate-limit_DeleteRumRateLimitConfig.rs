// Delete a RUM rate limit configuration returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_rate_limit::RumRateLimitAPI;
use datadog_api_client::datadogV2::model::RumRateLimitScopeType;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteRumRateLimitConfig", true);
    let api = RumRateLimitAPI::with_config(configuration);
    let resp = api
        .delete_rum_rate_limit_config(
            RumRateLimitScopeType::APPLICATION,
            "cd73a516-a481-4af5-8352-9b577465c77b".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

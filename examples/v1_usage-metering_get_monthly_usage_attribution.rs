// Get monthly usage attribution returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_usage_metering::GetMonthlyUsageAttributionOptionalParams;
use datadog_api_client::datadogV1::api::api_usage_metering::UsageMeteringAPI;
use datadog_api_client::datadogV1::model::MonthlyUsageAttributionSupportedMetrics;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .get_monthly_usage_attribution(
            DateTime::parse_from_rfc3339("2021-11-08T11:11:11+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
            MonthlyUsageAttributionSupportedMetrics::INFRA_HOST_USAGE,
            GetMonthlyUsageAttributionOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

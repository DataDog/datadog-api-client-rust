// Get hourly usage for indexed spans returns "OK" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_usage_metering::GetUsageIndexedSpansOptionalParams;
use datadog_api_client::datadogV1::api_usage_metering::UsageMeteringAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .get_usage_indexed_spans(
            DateTime::parse_from_rfc3339("2021-11-06T11:11:11+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
            GetUsageIndexedSpansOptionalParams::default().end_hr(
                DateTime::parse_from_rfc3339("2021-11-08T11:11:11+00:00")
                    .expect("Failed to parse datetime")
                    .with_timezone(&Utc),
            ),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

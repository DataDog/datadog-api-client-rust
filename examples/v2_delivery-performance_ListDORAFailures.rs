// Get a list of incident events returns "OK" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_delivery_performance::DeliveryPerformanceAPI;
use datadog_api_client::datadogV2::model::DORAListFailuresRequest;
use datadog_api_client::datadogV2::model::DORAListFailuresRequestAttributes;
use datadog_api_client::datadogV2::model::DORAListFailuresRequestData;
use datadog_api_client::datadogV2::model::DORAListFailuresRequestDataType;

#[tokio::main]
async fn main() {
    let body = DORAListFailuresRequest::new(
        DORAListFailuresRequestData::new(
            DORAListFailuresRequestAttributes::new()
                .from(
                    DateTime::parse_from_rfc3339("2025-01-01T00:00:00+00:00")
                        .expect("Failed to parse datetime")
                        .with_timezone(&Utc),
                )
                .limit(100)
                .query("severity:(SEV-1 OR SEV-2) env:production team:backend".to_string())
                .sort("-started_at".to_string())
                .to(DateTime::parse_from_rfc3339("2025-01-31T23:59:59+00:00")
                    .expect("Failed to parse datetime")
                    .with_timezone(&Utc)),
        )
        .type_(DORAListFailuresRequestDataType::DORA_FAILURES_LIST_REQUEST),
    );
    let configuration = datadog::Configuration::new();
    let api = DeliveryPerformanceAPI::with_config(configuration);
    let resp = api.list_dora_failures(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

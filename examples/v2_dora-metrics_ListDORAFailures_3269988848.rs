// Get a list of failure events returns "OK" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dora_metrics::DORAMetricsAPI;
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
                    DateTime::parse_from_rfc3339("2025-03-23T00:00:00+00:00")
                        .expect("Failed to parse datetime")
                        .with_timezone(&Utc),
                )
                .limit(1)
                .to(DateTime::parse_from_rfc3339("2025-03-24T00:00:00+00:00")
                    .expect("Failed to parse datetime")
                    .with_timezone(&Utc)),
        )
        .type_(DORAListFailuresRequestDataType::DORA_FAILURES_LIST_REQUEST),
    );
    let configuration = datadog::Configuration::new();
    let api = DORAMetricsAPI::with_config(configuration);
    let resp = api.list_dora_failures(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

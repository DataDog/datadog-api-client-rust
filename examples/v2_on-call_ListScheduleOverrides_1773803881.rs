// List On-Call schedule overrides returns "OK" response with pagination
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::ListScheduleOverridesOptionalParams;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let response = api.list_schedule_overrides_with_pagination(
        "3653d3c6-0c75-11ea-ad28-fb5701eabc7d".to_string(),
        DateTime::parse_from_rfc3339("2024-01-07T02:53:01+00:00")
            .expect("Failed to parse datetime")
            .with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2024-01-14T02:53:01+00:00")
            .expect("Failed to parse datetime")
            .with_timezone(&Utc),
        ListScheduleOverridesOptionalParams::default(),
    );
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}

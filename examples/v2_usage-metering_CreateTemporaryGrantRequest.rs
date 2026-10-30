// Create a temporary grant request returns "Created" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_usage_metering::UsageMeteringAPI;
use datadog_api_client::datadogV2::model::TemporaryGrantRequestCreateAttributes;
use datadog_api_client::datadogV2::model::TemporaryGrantRequestCreateData;
use datadog_api_client::datadogV2::model::TemporaryGrantRequestCreateRequest;
use datadog_api_client::datadogV2::model::TemporaryGrantRequestType;

#[tokio::main]
async fn main() {
    let body = TemporaryGrantRequestCreateRequest::new(TemporaryGrantRequestCreateData::new(
        TemporaryGrantRequestCreateAttributes::new(
            "Credits needed to finish the incident investigation.".to_string(),
            200,
        )
        .expires_at(Some(
            DateTime::parse_from_rfc3339("2028-02-16T00:00:00+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
        )),
        TemporaryGrantRequestType::TEMPORARY_GRANT_REQUESTS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_temporary_grant_request", true);
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api.create_temporary_grant_request(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

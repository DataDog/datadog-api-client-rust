// Get a list of deployment events returns "OK" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dora_metrics::DORAMetricsAPI;
use datadog_api_client::datadogV2::model::DORAListDeploymentsRequest;
use datadog_api_client::datadogV2::model::DORAListDeploymentsRequestAttributes;
use datadog_api_client::datadogV2::model::DORAListDeploymentsRequestData;
use datadog_api_client::datadogV2::model::DORAListDeploymentsRequestDataType;

#[tokio::main]
async fn main() {
    let body = DORAListDeploymentsRequest::new(
        DORAListDeploymentsRequestData::new(
            DORAListDeploymentsRequestAttributes::new()
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
        .type_(DORAListDeploymentsRequestDataType::DORA_DEPLOYMENTS_LIST_REQUEST),
    );
    let configuration = datadog::Configuration::new();
    let api = DORAMetricsAPI::with_config(configuration);
    let resp = api.list_dora_deployments(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

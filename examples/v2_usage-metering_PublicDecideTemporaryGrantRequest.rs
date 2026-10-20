// Decide a temporary grant request returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_usage_metering::UsageMeteringAPI;
use datadog_api_client::datadogV2::model::TemporaryGrantRequestDecision;
use datadog_api_client::datadogV2::model::TemporaryGrantRequestDecisionAttributes;
use datadog_api_client::datadogV2::model::TemporaryGrantRequestDecisionData;
use datadog_api_client::datadogV2::model::TemporaryGrantRequestDecisionRequest;
use datadog_api_client::datadogV2::model::TemporaryGrantRequestType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = TemporaryGrantRequestDecisionRequest::new(
        TemporaryGrantRequestDecisionData::new(
            TemporaryGrantRequestDecisionAttributes::new(TemporaryGrantRequestDecision::APPROVED),
            TemporaryGrantRequestType::TEMPORARY_GRANT_REQUESTS,
        )
        .id(Uuid::parse_str("12345678-1234-1234-1234-123456789012").expect("invalid UUID")),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.public_decide_temporary_grant_request", true);
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .public_decide_temporary_grant_request(
            Uuid::parse_str("12345678-1234-1234-1234-123456789012").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

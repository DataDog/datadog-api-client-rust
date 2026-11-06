// Cancel a batch recovery request returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_recovery::RUMRecoveryAPI;
use datadog_api_client::datadogV2::model::RumRecoveryCancelAttributes;
use datadog_api_client::datadogV2::model::RumRecoveryCancelData;
use datadog_api_client::datadogV2::model::RumRecoveryCancelRequest;
use datadog_api_client::datadogV2::model::RumRecoveryCancelStatus;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = RumRecoveryCancelRequest::new(
        RumRecoveryCancelData::new(RumRecoveryCancelAttributes::new(
            RumRecoveryCancelStatus::CANCELLED,
        ))
        .id("11111111-1111-4111-8111-111111111111".to_string())
        .type_("rum_recovery_request".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = RUMRecoveryAPI::with_config(configuration);
    let resp = api
        .update_rum_recovery_batch_request(
            Uuid::parse_str("11111111-1111-4111-8111-111111111111").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Count recoverable sessions accepts exactly 180 days
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_recovery::RUMRecoveryAPI;
use datadog_api_client::datadogV2::model::RumRecoveryCountAttributes;
use datadog_api_client::datadogV2::model::RumRecoveryCountRequest;
use datadog_api_client::datadogV2::model::RumRecoveryCountRequestData;

#[tokio::main]
async fn main() {
    let body = RumRecoveryCountRequest::new(
        RumRecoveryCountRequestData::new(RumRecoveryCountAttributes::new(
            1621077071000,
            "service:rum18600-fixture".to_string(),
            1636629071000,
        ))
        .type_("rum_recovery_count".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = RUMRecoveryAPI::with_config(configuration);
    let resp = api.get_rum_recovery_session_count(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Create a recovery request returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_recovery::RUMRecoveryAPI;
use datadog_api_client::datadogV2::model::RumRecoveryCreateAttributes;
use datadog_api_client::datadogV2::model::RumRecoveryCreateData;
use datadog_api_client::datadogV2::model::RumRecoveryCreateRequest;
use datadog_api_client::datadogV2::model::RumRecoverySessionInput;
use datadog_api_client::datadogV2::model::RumRecoverySessionsCreateAttributes;
use datadog_api_client::datadogV2::model::RumRecoverySessionsRequestType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = RumRecoveryCreateRequest::new(
        RumRecoveryCreateData::new(
            RumRecoveryCreateAttributes::RumRecoverySessionsCreateAttributes(Box::new(
                RumRecoverySessionsCreateAttributes::new(vec![RumRecoverySessionInput::new(
                    Uuid::parse_str("a33671aa-24fd-4dcd-ba4b-5bbdbafe7690").expect("invalid UUID"),
                    Uuid::parse_str("00000000-0000-4000-8000-000000000001").expect("invalid UUID"),
                )])
                .request_type(RumRecoverySessionsRequestType::SESSIONS),
            )),
        )
        .type_("rum_recovery_request".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = RUMRecoveryAPI::with_config(configuration);
    let resp = api.create_rum_recovery_request(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

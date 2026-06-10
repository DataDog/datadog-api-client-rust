// Submit feedback on an ownership inference returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_csm_ownership::CSMOwnershipAPI;
use datadog_api_client::datadogV2::model::OwnershipFeedbackAction;
use datadog_api_client::datadogV2::model::OwnershipFeedbackRequest;
use datadog_api_client::datadogV2::model::OwnershipFeedbackRequestAttributes;
use datadog_api_client::datadogV2::model::OwnershipFeedbackRequestData;
use datadog_api_client::datadogV2::model::OwnershipFeedbackType;
use datadog_api_client::datadogV2::model::OwnershipOwnerType;

#[tokio::main]
async fn main() {
    let body = OwnershipFeedbackRequest::new(OwnershipFeedbackRequestData::new(
        OwnershipFeedbackRequestAttributes::new(
            OwnershipFeedbackAction::CONFIRM,
            "user@example.com".to_string(),
            "user".to_string(),
            "abc123".to_string(),
        )
        .corrected_owner_handle(Some("team-b".to_string()))
        .corrected_owner_type(Some("team".to_string()))
        .reason(Some("Confirmed by team lead.".to_string())),
        OwnershipFeedbackType::OWNERSHIP_FEEDBACK,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateOwnershipFeedback", true);
    let api = CSMOwnershipAPI::with_config(configuration);
    let resp = api
        .create_ownership_feedback("res-1".to_string(), OwnershipOwnerType::TEAM, body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Update an incident status page notice returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::api_incidents::UpdateIncidentStatusPageNoticeOptionalParams;
use datadog_api_client::datadogV2::model::IncidentStatusPageNoticeIntegrationType;
use datadog_api_client::datadogV2::model::IncidentStatusPageNoticeUpdateData;
use datadog_api_client::datadogV2::model::IncidentStatusPageNoticeUpdateDataAttributes;
use datadog_api_client::datadogV2::model::IncidentStatusPageNoticeUpdateRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = IncidentStatusPageNoticeUpdateRequest::new(IncidentStatusPageNoticeUpdateData::new(
        IncidentStatusPageNoticeUpdateDataAttributes::new()
            .message("The issue has been resolved.".to_string())
            .status("resolved".to_string())
            .title("Service degradation resolved.".to_string()),
        IncidentStatusPageNoticeIntegrationType::INCIDENT_INTEGRATIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateIncidentStatusPageNotice", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .update_incident_status_page_notice(
            "incident_id".to_string(),
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            body,
            UpdateIncidentStatusPageNoticeOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

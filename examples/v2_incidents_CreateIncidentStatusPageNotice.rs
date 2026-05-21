// Publish an incident status page notice returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::CreateIncidentStatusPageNoticeOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentStatusPageNoticeCreateData;
use datadog_api_client::datadogV2::model::IncidentStatusPageNoticeCreateDataAttributes;
use datadog_api_client::datadogV2::model::IncidentStatusPageNoticeCreateRequest;
use datadog_api_client::datadogV2::model::IncidentStatusPageNoticeIntegrationType;
use std::collections::BTreeMap;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = IncidentStatusPageNoticeCreateRequest::new(IncidentStatusPageNoticeCreateData::new(
        IncidentStatusPageNoticeCreateDataAttributes::new()
            .components(BTreeMap::from([(
                "component_1".to_string(),
                "degraded_performance".to_string(),
            )]))
            .message("We are investigating reports of elevated error rates.".to_string())
            .status("investigating".to_string())
            .title("Service degradation detected.".to_string()),
        IncidentStatusPageNoticeIntegrationType::INCIDENT_INTEGRATIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentStatusPageNotice", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .create_incident_status_page_notice(
            "incident_id".to_string(),
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            body,
            CreateIncidentStatusPageNoticeOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

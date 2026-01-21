// Create incident attachment returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::CreateIncidentAttachmentOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::AttachmentDataAttributesAttachmentType;
use datadog_api_client::datadogV2::model::CreateAttachmentRequest;
use datadog_api_client::datadogV2::model::CreateAttachmentRequestData;
use datadog_api_client::datadogV2::model::CreateAttachmentRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateAttachmentRequestDataAttributesAttachment;
use datadog_api_client::datadogV2::model::IncidentAttachmentType;

#[tokio::main]
async fn main() {
    // there is a valid "incident" in the system
    let incident_data_id = std::env::var("INCIDENT_DATA_ID").unwrap();
    let body = CreateAttachmentRequest::new().data(
        CreateAttachmentRequestData::new(IncidentAttachmentType::INCIDENT_ATTACHMENTS).attributes(
            CreateAttachmentRequestDataAttributes::new()
                .attachment(
                    CreateAttachmentRequestDataAttributesAttachment::new()
                        .document_url(
                            "https://app.datadoghq.com/notebook/ExampleIncident/Example-Incident"
                                .to_string(),
                        )
                        .title("Example-Incident".to_string()),
                )
                .attachment_type(AttachmentDataAttributesAttachmentType::POSTMORTEM),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentAttachment", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .create_incident_attachment(
            incident_data_id.clone(),
            body,
            CreateIncidentAttachmentOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

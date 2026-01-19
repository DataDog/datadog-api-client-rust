// Create postmortem attachment returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentAttachmentType;
use datadog_api_client::datadogV2::model::PostmortemAttachmentRequest;
use datadog_api_client::datadogV2::model::PostmortemAttachmentRequestAttributes;
use datadog_api_client::datadogV2::model::PostmortemAttachmentRequestData;
use datadog_api_client::datadogV2::model::PostmortemCell;
use datadog_api_client::datadogV2::model::PostmortemCellAttributes;
use datadog_api_client::datadogV2::model::PostmortemCellDefinition;
use datadog_api_client::datadogV2::model::PostmortemCellType;

#[tokio::main]
async fn main() {
    let body = PostmortemAttachmentRequest::new(PostmortemAttachmentRequestData::new(
        PostmortemAttachmentRequestAttributes::new()
            .cells(vec![PostmortemCell::new()
                .attributes(
                    PostmortemCellAttributes::new().definition(
                        PostmortemCellDefinition::new().content(
                            r#"## Incident Summary
This incident was caused by..."#
                                .to_string(),
                        ),
                    ),
                )
                .id("cell-1".to_string())
                .type_(PostmortemCellType::MARKDOWN)])
            .content(
                r#"# Incident Report - IR-123
[...]"#
                    .to_string(),
            )
            .postmortem_template_id("93645509-874e-45c4-adfa-623bfeaead89-123".to_string())
            .title("Postmortem-IR-123".to_string()),
        IncidentAttachmentType::INCIDENT_ATTACHMENTS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentPostmortemAttachment", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .create_incident_postmortem_attachment(
            "00000000-0000-0000-0000-000000000000".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

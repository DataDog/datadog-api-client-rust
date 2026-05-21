// Render an incident template returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentRenderTemplateDataAttributesRequest;
use datadog_api_client::datadogV2::model::IncidentRenderTemplateDataRequest;
use datadog_api_client::datadogV2::model::IncidentRenderTemplateRequest;
use datadog_api_client::datadogV2::model::IncidentRenderedTemplateType;

#[tokio::main]
async fn main() {
    let body = IncidentRenderTemplateRequest::new(IncidentRenderTemplateDataRequest::new(
        IncidentRenderTemplateDataAttributesRequest::new("Incident INC-123 is SEV-1.".to_string())
            .datetime_format("2006-01-02T15:04:05Z07:00".to_string())
            .timezone("America/New_York".to_string())
            .validate_links(Some(false))
            .validate_variables(Some(false)),
        IncidentRenderedTemplateType::RENDERED_TEMPLATES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.RenderIncidentTemplate", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .render_incident_template("incident_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

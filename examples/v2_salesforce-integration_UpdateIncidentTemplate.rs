// Update a Salesforce incident template returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_salesforce_integration::SalesforceIntegrationAPI;
use datadog_api_client::datadogV2::model::SalesforceIncidentsTemplatePriority;
use datadog_api_client::datadogV2::model::SalesforceIncidentsTemplateType;
use datadog_api_client::datadogV2::model::SalesforceIncidentsTemplateUpdateAttributes;
use datadog_api_client::datadogV2::model::SalesforceIncidentsTemplateUpdateData;
use datadog_api_client::datadogV2::model::SalesforceIncidentsTemplateUpdateRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body =
        SalesforceIncidentsTemplateUpdateRequest::new(SalesforceIncidentsTemplateUpdateData::new(
            SalesforceIncidentsTemplateUpdateAttributes::new()
                .description("An incident was detected by Datadog monitors.".to_string())
                .name("production-outage".to_string())
                .owner_id("005000000000000".to_string())
                .priority(SalesforceIncidentsTemplatePriority::HIGH)
                .salesforce_org_id(
                    Uuid::parse_str("596da4af-0563-4097-90ff-07230c3f9db3").expect("invalid UUID"),
                )
                .subject("Datadog Incident: Production Outage".to_string()),
            "596da4af-0563-4097-90ff-07230c3f9db3".to_string(),
            SalesforceIncidentsTemplateType::SALESFORCE_INCIDENTS_INCIDENT_TEMPLATE,
        ));
    let configuration = datadog::Configuration::new();
    let api = SalesforceIntegrationAPI::with_config(configuration);
    let resp = api
        .update_incident_template("incident_template_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Create a Salesforce incident template returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_salesforce_integration::SalesforceIntegrationAPI;
use datadog_api_client::datadogV2::model::SalesforceIncidentsTemplateCreateAttributes;
use datadog_api_client::datadogV2::model::SalesforceIncidentsTemplateCreateData;
use datadog_api_client::datadogV2::model::SalesforceIncidentsTemplateCreateRequest;
use datadog_api_client::datadogV2::model::SalesforceIncidentsTemplatePriority;
use datadog_api_client::datadogV2::model::SalesforceIncidentsTemplateType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body =
        SalesforceIncidentsTemplateCreateRequest::new(SalesforceIncidentsTemplateCreateData::new(
            SalesforceIncidentsTemplateCreateAttributes::new(
                "An incident was detected by Datadog monitors.".to_string(),
                "production-outage".to_string(),
                "005000000000000".to_string(),
                SalesforceIncidentsTemplatePriority::HIGH,
                Uuid::parse_str("596da4af-0563-4097-90ff-07230c3f9db3").expect("invalid UUID"),
                "Datadog Incident: Production Outage".to_string(),
            ),
            SalesforceIncidentsTemplateType::SALESFORCE_INCIDENTS_INCIDENT_TEMPLATE,
        ));
    let configuration = datadog::Configuration::new();
    let api = SalesforceIntegrationAPI::with_config(configuration);
    let resp = api.create_incident_template(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

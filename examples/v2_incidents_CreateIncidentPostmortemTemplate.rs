// Create postmortem template returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::PostmortemTemplateAttributesRequest;
use datadog_api_client::datadogV2::model::PostmortemTemplateDataRequest;
use datadog_api_client::datadogV2::model::PostmortemTemplateRequest;
use datadog_api_client::datadogV2::model::PostmortemTemplateType;

#[tokio::main]
async fn main() {
    let body = PostmortemTemplateRequest::new(PostmortemTemplateDataRequest::new(
        PostmortemTemplateAttributesRequest::new("Standard Postmortem Template".to_string()),
        PostmortemTemplateType::POSTMORTEM_TEMPLATE,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentPostmortemTemplate", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api.create_incident_postmortem_template(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Delete a Salesforce incident template returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_salesforce_integration::SalesforceIntegrationAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = SalesforceIntegrationAPI::with_config(configuration);
    let resp = api
        .delete_incident_template("incident_template_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Create a child organization returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_organizations::OrganizationsAPI;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    let body =
        OrganizationCreateBody::new("New child org".to_string())
            .billing(OrganizationBilling::new().type_("parent_billing".to_string()))
            .subscription(OrganizationSubscription::new().type_("pro".to_string()));
    let configuration = Configuration::new();
    let api = OrganizationsAPI::with_config(configuration);
    let resp = api.create_child_org(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

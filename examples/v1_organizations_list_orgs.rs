// List your managed organizations returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_organizations::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = OrganizationsAPI::with_config(configuration);
    let resp = api.list_orgs().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

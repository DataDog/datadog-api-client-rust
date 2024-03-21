// Upload IdP metadata returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_organizations::OrganizationsAPI;
use std::fs;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = OrganizationsAPI::with_config(configuration);
    let resp = api
        .upload_idp_for_org(
            "abc123".to_string(),
            fs::read("./idp_metadata.xml").unwrap(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

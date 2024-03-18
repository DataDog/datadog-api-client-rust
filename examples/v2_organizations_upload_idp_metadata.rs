// Upload IdP metadata returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_organizations::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = OrganizationsAPI::with_config(configuration);
    let resp = api
        .upload_idp_metadata(
            UploadIdPMetadataOptionalParams::default().idp_file(
                std::fs::read("fixtures/organizations/saml_configurations/valid_idp_metadata.xml")
                    .unwrap(),
            ),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

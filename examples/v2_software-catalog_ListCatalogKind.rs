// Get a list of entity kinds returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_software_catalog::ListCatalogKindOptionalParams;
use datadog_api_client::datadogV2::api_software_catalog::SoftwareCatalogAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = SoftwareCatalogAPI::with_config(configuration);
    let resp = api
        .list_catalog_kind(ListCatalogKindOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

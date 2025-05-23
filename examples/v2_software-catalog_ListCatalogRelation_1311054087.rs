// Get a list of entity relations returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_software_catalog::ListCatalogRelationOptionalParams;
use datadog_api_client::datadogV2::api_software_catalog::SoftwareCatalogAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = SoftwareCatalogAPI::with_config(configuration);
    let response = api.list_catalog_relation_with_pagination(
        ListCatalogRelationOptionalParams::default().page_limit(20),
    );
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}

// List SKUs returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_product_catalog::ListProductCatalogSKUsOptionalParams;
use datadog_api_client::datadogV2::api_product_catalog::ProductCatalogAPI;
use datadog_api_client::datadogV2::model::ProductCatalogSKUsAPIVersion;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListProductCatalogSKUs", true);
    let api = ProductCatalogAPI::with_config(configuration);
    let resp = api
        .list_product_catalog_sk_us(
            ProductCatalogSKUsAPIVersion::V1,
            ListProductCatalogSKUsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

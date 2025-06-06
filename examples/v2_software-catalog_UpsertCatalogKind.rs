// Create or update kinds returns "ACCEPTED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_software_catalog::SoftwareCatalogAPI;
use datadog_api_client::datadogV2::model::KindObj;
use datadog_api_client::datadogV2::model::UpsertCatalogKindRequest;

#[tokio::main]
async fn main() {
    let body = UpsertCatalogKindRequest::KindObj(Box::new(KindObj::new("my-job".to_string())));
    let configuration = datadog::Configuration::new();
    let api = SoftwareCatalogAPI::with_config(configuration);
    let resp = api.upsert_catalog_kind(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

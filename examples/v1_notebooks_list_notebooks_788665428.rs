// Get all notebooks returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_notebooks::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = NotebooksAPI::with_config(configuration);
    let resp = api.list_notebooks(ListNotebooksOptionalParams::default().count(2)).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

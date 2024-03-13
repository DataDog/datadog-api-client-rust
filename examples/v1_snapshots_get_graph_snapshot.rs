// Take graph snapshots returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_snapshots::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = SnapshotsAPI::with_config(configuration);
    let resp = api
        .get_graph_snapshot(
            1636542671,
            1636629071,
            GetGraphSnapshotOptionalParams::default()
                .metric_query("avg:system.load.1{*}".to_string())
                .title("System load".to_string())
                .height(400)
                .width(600),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
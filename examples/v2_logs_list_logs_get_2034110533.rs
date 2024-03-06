// Get a quick list of logs returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_logs::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = LogsAPI::with_config(configuration);
    let resp =
        api
            .list_logs_get(
                ListLogsGetOptionalParams::default()
                    .filter_query("datadog-agent".to_string())
                    .filter_indexes(vec!["main".to_string()])
                    .filter_from("2020-09-17T11:48:36+01:00".to_string())
                    .filter_to("2020-09-17T12:48:36+01:00".to_string())
                    .page_limit(5),
            )
            .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

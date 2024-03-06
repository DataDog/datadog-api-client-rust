// Update archive order returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_logs_archives::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body =
        LogsArchiveOrder
        ::new().data(
            LogsArchiveOrderDefinition::new(
                LogsArchiveOrderAttributes::new(
                    vec![
                        "a2zcMylnM4OCHpYusxIi1g".to_string(),
                        "a2zcMylnM4OCHpYusxIi2g".to_string(),
                        "a2zcMylnM4OCHpYusxIi3g".to_string()
                    ],
                ),
                LogsArchiveOrderDefinitionType::ARCHIVE_ORDER,
            ),
        );
    let configuration = Configuration::new();
    let api = LogsArchivesAPI::with_config(configuration);
    let resp = api.update_logs_archive_order(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

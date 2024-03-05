// Bulk Delete SLO Timeframes returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_service_level_objectives::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        BTreeMap::from(
            [
                ("id1".to_string(), vec![SLOTimeframe::SEVEN_DAYS, SLOTimeframe::THIRTY_DAYS]),
                ("id2".to_string(), vec![SLOTimeframe::SEVEN_DAYS, SLOTimeframe::THIRTY_DAYS]),
            ],
        );
    let configuration = Configuration::new();
    let api = ServiceLevelObjectivesAPI::with_config(configuration);
    let resp = api.delete_slo_timeframe_in_bulk(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

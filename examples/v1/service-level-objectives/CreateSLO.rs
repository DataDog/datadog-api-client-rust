// Create an SLO object returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_service_level_objectives::ServiceLevelObjectivesAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        ServiceLevelObjectiveRequest::new(
            "Example-Service-Level-Objective".to_string(),
            vec![
                SLOThreshold::new(97.0, SLOTimeframe::SEVEN_DAYS)
                    .target_display("97.0".to_string())
                    .warning(98)
                    .warning_display("98.0".to_string())
            ],
            SLOType::METRIC,
        )
            .description(Some("string".to_string()))
            .groups(vec!["env:test".to_string(), "role:mysql".to_string()])
            .monitor_ids(vec![])
            .query(
                ServiceLevelObjectiveQuery::new(
                    "sum:httpservice.hits{!code:3xx}.as_count()".to_string(),
                    "sum:httpservice.hits{code:2xx}.as_count()".to_string(),
                ),
            )
            .tags(vec!["env:prod".to_string(), "app:core".to_string()])
            .target_threshold(97.0)
            .timeframe(SLOTimeframe::SEVEN_DAYS)
            .warning_threshold(98);
    let configuration = Configuration::new();
    let api = ServiceLevelObjectivesAPI::with_config(configuration);
    let resp = api.create_slo(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

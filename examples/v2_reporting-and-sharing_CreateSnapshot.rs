// Create a graph snapshot returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_reporting_and_sharing::ReportingAndSharingAPI;
use datadog_api_client::datadogV2::model::CreateSnapshotAdditionalConfig;
use datadog_api_client::datadogV2::model::CreateSnapshotDataAttributesRequest;
use datadog_api_client::datadogV2::model::CreateSnapshotDataRequest;
use datadog_api_client::datadogV2::model::CreateSnapshotRequest;
use datadog_api_client::datadogV2::model::CreateSnapshotTTL;
use datadog_api_client::datadogV2::model::CreateSnapshotTemplateVariable;
use datadog_api_client::datadogV2::model::CreateSnapshotTimeseriesLegendType;
use datadog_api_client::datadogV2::model::CreateSnapshotType;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = CreateSnapshotRequest::new(CreateSnapshotDataRequest::new(
        CreateSnapshotDataAttributesRequest::new(
            1692464800000,
            1692464000000,
            BTreeMap::from([("type".to_string(), Value::from("timeseries"))]),
        )
        .additional_config(
            CreateSnapshotAdditionalConfig::new()
                .template_variables(vec![CreateSnapshotTemplateVariable::new(
                    "host".to_string(),
                    "host".to_string(),
                    vec!["web-server-1".to_string(), "web-server-2".to_string()],
                )])
                .timeseries_legend_type(CreateSnapshotTimeseriesLegendType::EXPANDED)
                .timezone_offset_minutes(300),
        )
        .height(185)
        .is_authenticated(false)
        .ttl(CreateSnapshotTTL::SIXTY_DAYS)
        .width(300),
        CreateSnapshotType::CREATE_SNAPSHOT,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateSnapshot", true);
    let api = ReportingAndSharingAPI::with_config(configuration);
    let resp = api.create_snapshot(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

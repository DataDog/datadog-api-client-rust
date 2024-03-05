// Schedule a downtime returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_downtimes::DowntimesAPI;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body =
        DowntimeCreateRequest::new(
            DowntimeCreateRequestData::new(
                DowntimeCreateRequestAttributes::new(
                    DowntimeMonitorIdentifier::DowntimeMonitorIdentifierTags(
                        Box::new(DowntimeMonitorIdentifierTags::new(vec!["cat:hat".to_string()])),
                    ),
                    "test:exampledowntime".to_string(),
                )
                    .message(Some("dark forest".to_string()))
                    .schedule(
                        DowntimeScheduleCreateRequest::DowntimeScheduleOneTimeCreateUpdateRequest(
                            Box::new(DowntimeScheduleOneTimeCreateUpdateRequest::new().start(Some(None))),
                        ),
                    ),
                DowntimeResourceType::DOWNTIME,
            ),
        );
    let configuration = Configuration::new();
    let api = DowntimesAPI::with_config(configuration);
    let resp = api.create_downtime(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

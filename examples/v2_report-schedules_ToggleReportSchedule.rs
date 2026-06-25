// Toggle a report schedule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_report_schedules::ReportSchedulesAPI;
use datadog_api_client::datadogV2::model::ReportScheduleStatus;
use datadog_api_client::datadogV2::model::ReportScheduleToggleRequest;
use datadog_api_client::datadogV2::model::ReportScheduleToggleRequestAttributes;
use datadog_api_client::datadogV2::model::ReportScheduleToggleRequestData;
use datadog_api_client::datadogV2::model::ReportScheduleType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = ReportScheduleToggleRequest::new(ReportScheduleToggleRequestData::new(
        ReportScheduleToggleRequestAttributes::new(ReportScheduleStatus::ACTIVE),
        ReportScheduleType::SCHEDULE,
    ));
    let configuration = datadog::Configuration::new();
    let api = ReportSchedulesAPI::with_config(configuration);
    let resp = api
        .toggle_report_schedule(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

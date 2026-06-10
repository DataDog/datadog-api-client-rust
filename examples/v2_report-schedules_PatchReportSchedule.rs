// Update a report schedule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_report_schedules::ReportSchedulesAPI;
use datadog_api_client::datadogV2::model::ReportScheduleDeliveryFormat;
use datadog_api_client::datadogV2::model::ReportSchedulePatchRequest;
use datadog_api_client::datadogV2::model::ReportSchedulePatchRequestAttributes;
use datadog_api_client::datadogV2::model::ReportSchedulePatchRequestData;
use datadog_api_client::datadogV2::model::ReportScheduleTemplateVariable;
use datadog_api_client::datadogV2::model::ReportScheduleType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body =
        ReportSchedulePatchRequest::new(
            ReportSchedulePatchRequestData::new(
                ReportSchedulePatchRequestAttributes::new(
                    "Updated weekly summary of infrastructure health.".to_string(),
                    vec![
                        "user@example.com".to_string(),
                        "slack:T01234567.C01234567.alerts".to_string(),
                        "teams:11111111-1111-1111-1111-111111111111|22222222-2222-2222-2222-222222222222|19:exampleChannelId@thread.tacv2".to_string()
                    ],
                    r#"DTSTART;TZID=America/New_York:20260601T090000
RRULE:FREQ=WEEKLY;BYDAY=MO;BYHOUR=9;BYMINUTE=0"#.to_string(),
                    vec![ReportScheduleTemplateVariable::new("env".to_string(), vec!["prod".to_string()])],
                    "calendar_month".to_string(),
                    "America/New_York".to_string(),
                    "Weekly Infrastructure Report".to_string(),
                )
                    .delivery_format(ReportScheduleDeliveryFormat::PDF)
                    .tab_id(Uuid::parse_str("66666666-7777-8888-9999-000000000000").expect("invalid UUID")),
                ReportScheduleType::SCHEDULE,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.PatchReportSchedule", true);
    let api = ReportSchedulesAPI::with_config(configuration);
    let resp = api
        .patch_report_schedule(
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

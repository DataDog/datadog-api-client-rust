// Print a report returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_report_schedules::ReportSchedulesAPI;
use datadog_api_client::datadogV2::model::PrintReportRequest;
use datadog_api_client::datadogV2::model::PrintReportRequestAttributes;
use datadog_api_client::datadogV2::model::PrintReportRequestData;
use datadog_api_client::datadogV2::model::PrintReportType;
use datadog_api_client::datadogV2::model::ReportScheduleResourceType;
use datadog_api_client::datadogV2::model::ReportScheduleTemplateVariable;

#[tokio::main]
async fn main() {
    let body = PrintReportRequest::new(PrintReportRequestData::new(
        PrintReportRequestAttributes::new(
            "abc-def-ghi".to_string(),
            ReportScheduleResourceType::DASHBOARD,
            vec![ReportScheduleTemplateVariable::new(
                "env".to_string(),
                vec!["prod".to_string()],
            )],
            "America/New_York".to_string(),
        )
        .from_ts(1780318800000)
        .timeframe("1w".to_string())
        .to_ts(1780923600000),
        PrintReportType::REPORT,
    ));
    let configuration = datadog::Configuration::new();
    let api = ReportSchedulesAPI::with_config(configuration);
    let resp = api.print_report(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

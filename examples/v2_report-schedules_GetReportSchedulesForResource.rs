// Get report schedules for a resource returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_report_schedules::ReportSchedulesAPI;
use datadog_api_client::datadogV2::model::ReportScheduleResourceType;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = ReportSchedulesAPI::with_config(configuration);
    let resp = api
        .get_report_schedules_for_resource(
            ReportScheduleResourceType::DASHBOARD,
            "resource_id".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

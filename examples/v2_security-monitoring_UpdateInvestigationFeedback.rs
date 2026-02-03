// Update investigation feedback returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalInvestigationFeedbackMetric;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalInvestigationFeedbackRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalInvestigationFeedbackRequestAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalInvestigationFeedbackRequestData;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalInvestigationFeedbackSection;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalInvestigationFeedbackType;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringSignalInvestigationFeedbackRequest::new(
        SecurityMonitoringSignalInvestigationFeedbackRequestData::new(
            SecurityMonitoringSignalInvestigationFeedbackRequestAttributes::new(
                "positive".to_string(),
                "AAAAAWgN8Xwgr1vKDQAAAABBV2dOOFh3ZzZobm1mWXJFYTR0OA".to_string(),
            )
            .feedback_content(vec![
                SecurityMonitoringSignalInvestigationFeedbackSection::new(
                    "section-1".to_string(),
                    vec![SecurityMonitoringSignalInvestigationFeedbackMetric::new(
                        "How helpful was this investigation?".to_string(),
                        "Very helpful".to_string(),
                        "sentiment".to_string(),
                    )
                    .placeholder("Enter your feedback here".to_string())],
                    "Investigation Quality".to_string(),
                ),
            ])
            .incomplete(false)
            .rating("positive".to_string())
            .type_("metrics".to_string()),
            SecurityMonitoringSignalInvestigationFeedbackType::INVESTIGATION_FEEDBACK,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateInvestigationFeedback", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.update_investigation_feedback(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Create an enrollment returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_prem_management_service::OnPremManagementServiceAPI;
use datadog_api_client::datadogV2::model::OnPremManagementServiceCreateEnrollmentRequest;
use datadog_api_client::datadogV2::model::OnPremManagementServiceEnrollmentAttributes;
use datadog_api_client::datadogV2::model::OnPremManagementServiceEnrollmentAttributesRunnerModesItems;
use datadog_api_client::datadogV2::model::OnPremManagementServiceEnrollmentDataRequest;
use datadog_api_client::datadogV2::model::OnPremManagementServiceEnrollmentType;

#[tokio::main]
async fn main() {
    let body =
        OnPremManagementServiceCreateEnrollmentRequest::new(
            OnPremManagementServiceEnrollmentDataRequest::new(
                OnPremManagementServiceEnrollmentAttributes::new(
                    vec!["com.datadoghq.jenkins.*".to_string()],
                    vec![OnPremManagementServiceEnrollmentAttributesRunnerModesItems::WORKFLOW_AUTOMATION],
                    "my-runner".to_string(),
                ).runner_host("runner.example.com".to_string()),
                OnPremManagementServiceEnrollmentType::ENROLLMENT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.CreateOnPremManagementServiceEnrollment", true);
    let api = OnPremManagementServiceAPI::with_config(configuration);
    let resp = api.create_on_prem_management_service_enrollment(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

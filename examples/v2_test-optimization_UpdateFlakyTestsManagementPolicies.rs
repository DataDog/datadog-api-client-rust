// Update Flaky Tests Management policies returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_test_optimization::TestOptimizationAPI;
use datadog_api_client::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesAttemptToFix;
use datadog_api_client::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesAutoDisableRule;
use datadog_api_client::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesAutoQuarantineRule;
use datadog_api_client::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesBranchRule;
use datadog_api_client::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesDisabled;
use datadog_api_client::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesDisabledFailureRateRule;
use datadog_api_client::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesDisabledStatus;
use datadog_api_client::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesQuarantined;
use datadog_api_client::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesQuarantinedFailureRateRule;
use datadog_api_client::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesUpdateRequest;
use datadog_api_client::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesUpdateRequestAttributes;
use datadog_api_client::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesUpdateRequestData;
use datadog_api_client::datadogV2::model::TestOptimizationUpdateFlakyTestsManagementPoliciesRequestDataType;

#[tokio::main]
async fn main() {
    let body =
        TestOptimizationFlakyTestsManagementPoliciesUpdateRequest::new(
            TestOptimizationFlakyTestsManagementPoliciesUpdateRequestData::new(
                TestOptimizationFlakyTestsManagementPoliciesUpdateRequestAttributes::new(
                    "github.com/datadog/shopist".to_string(),
                )
                    .attempt_to_fix(TestOptimizationFlakyTestsManagementPoliciesAttemptToFix::new().retries(3))
                    .disabled(
                        TestOptimizationFlakyTestsManagementPoliciesDisabled::new()
                            .auto_disable_rule(
                                TestOptimizationFlakyTestsManagementPoliciesAutoDisableRule::new()
                                    .enabled(false)
                                    .status(TestOptimizationFlakyTestsManagementPoliciesDisabledStatus::ACTIVE)
                                    .window_seconds(3600),
                            )
                            .branch_rule(
                                TestOptimizationFlakyTestsManagementPoliciesBranchRule::new()
                                    .branches(vec!["main".to_string()])
                                    .enabled(true)
                                    .excluded_branches(vec![])
                                    .excluded_test_services(vec![]),
                            )
                            .enabled(false)
                            .failure_rate_rule(
                                TestOptimizationFlakyTestsManagementPoliciesDisabledFailureRateRule::new()
                                    .branches(vec![])
                                    .enabled(false)
                                    .min_runs(10)
                                    .status(TestOptimizationFlakyTestsManagementPoliciesDisabledStatus::ACTIVE)
                                    .threshold(0.5 as f64),
                            ),
                    )
                    .quarantined(
                        TestOptimizationFlakyTestsManagementPoliciesQuarantined::new()
                            .auto_quarantine_rule(
                                TestOptimizationFlakyTestsManagementPoliciesAutoQuarantineRule::new()
                                    .enabled(true)
                                    .window_seconds(3600),
                            )
                            .branch_rule(
                                TestOptimizationFlakyTestsManagementPoliciesBranchRule::new()
                                    .branches(vec!["main".to_string()])
                                    .enabled(true)
                                    .excluded_branches(vec![])
                                    .excluded_test_services(vec![]),
                            )
                            .enabled(true)
                            .failure_rate_rule(
                                TestOptimizationFlakyTestsManagementPoliciesQuarantinedFailureRateRule::new()
                                    .branches(vec!["main".to_string()])
                                    .enabled(true)
                                    .min_runs(10)
                                    .threshold(0.5 as f64),
                            ),
                    ),
                TestOptimizationUpdateFlakyTestsManagementPoliciesRequestDataType
                ::TEST_OPTIMIZATION_UPDATE_FLAKY_TESTS_MANAGEMENT_POLICIES_REQUEST,
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = TestOptimizationAPI::with_config(configuration);
    let resp = api.update_flaky_tests_management_policies(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

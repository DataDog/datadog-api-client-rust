// Edit a mobile test returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV1::model::SyntheticsCheckType;
use datadog_api_client::datadogV1::model::SyntheticsConfigVariable;
use datadog_api_client::datadogV1::model::SyntheticsConfigVariableType;
use datadog_api_client::datadogV1::model::SyntheticsMobileStep;
use datadog_api_client::datadogV1::model::SyntheticsMobileStepParams;
use datadog_api_client::datadogV1::model::SyntheticsMobileStepParamsDirection;
use datadog_api_client::datadogV1::model::SyntheticsMobileStepParamsElement;
use datadog_api_client::datadogV1::model::SyntheticsMobileStepParamsElementContextType;
use datadog_api_client::datadogV1::model::SyntheticsMobileStepParamsElementRelativePosition;
use datadog_api_client::datadogV1::model::SyntheticsMobileStepParamsElementUserLocator;
use datadog_api_client::datadogV1::model::SyntheticsMobileStepParamsElementUserLocatorValuesItems;
use datadog_api_client::datadogV1::model::SyntheticsMobileStepParamsElementUserLocatorValuesItemsType;
use datadog_api_client::datadogV1::model::SyntheticsMobileStepParamsPositionsItems;
use datadog_api_client::datadogV1::model::SyntheticsMobileStepParamsVariable;
use datadog_api_client::datadogV1::model::SyntheticsMobileStepType;
use datadog_api_client::datadogV1::model::SyntheticsMobileTest;
use datadog_api_client::datadogV1::model::SyntheticsMobileTestConfig;
use datadog_api_client::datadogV1::model::SyntheticsMobileTestOptions;
use datadog_api_client::datadogV1::model::SyntheticsMobileTestType;
use datadog_api_client::datadogV1::model::SyntheticsMobileTestsMobileApplication;
use datadog_api_client::datadogV1::model::SyntheticsMobileTestsMobileApplicationReferenceType;
use datadog_api_client::datadogV1::model::SyntheticsTestCiOptions;
use datadog_api_client::datadogV1::model::SyntheticsTestExecutionRule;
use datadog_api_client::datadogV1::model::SyntheticsTestOptionsMonitorOptions;
use datadog_api_client::datadogV1::model::SyntheticsTestOptionsMonitorOptionsNotificationPresetName;
use datadog_api_client::datadogV1::model::SyntheticsTestOptionsRetry;
use datadog_api_client::datadogV1::model::SyntheticsTestOptionsScheduling;
use datadog_api_client::datadogV1::model::SyntheticsTestOptionsSchedulingTimeframe;
use datadog_api_client::datadogV1::model::SyntheticsTestPauseStatus;
use datadog_api_client::datadogV1::model::SyntheticsTestRestrictionPolicyBinding;
use datadog_api_client::datadogV1::model::SyntheticsTestRestrictionPolicyBindingRelation;

#[tokio::main]
async fn main() {
    let body =
        SyntheticsMobileTest::new(
            SyntheticsMobileTestConfig
            ::new().variables(
                vec![
                    SyntheticsConfigVariable::new(
                        "VARIABLE_NAME".to_string(),
                        SyntheticsConfigVariableType::TEXT,
                    ).secure(false)
                ],
            ),
            "Notification message".to_string(),
            "Example test name".to_string(),
            SyntheticsMobileTestOptions::new(
                vec!["synthetics:mobile:device:apple_ipad_10th_gen_2022_ios_16".to_string()],
                SyntheticsMobileTestsMobileApplication::new(
                    "00000000-0000-0000-0000-aaaaaaaaaaaa".to_string(),
                    "00000000-0000-0000-0000-aaaaaaaaaaab".to_string(),
                    SyntheticsMobileTestsMobileApplicationReferenceType::LATEST,
                ),
                300,
            )
                .bindings(
                    vec![
                        SyntheticsTestRestrictionPolicyBinding::new()
                            .principals(vec![])
                            .relation(SyntheticsTestRestrictionPolicyBindingRelation::EDITOR)
                    ],
                )
                .ci(SyntheticsTestCiOptions::new(SyntheticsTestExecutionRule::BLOCKING))
                .monitor_options(
                    SyntheticsTestOptionsMonitorOptions
                    ::new().notification_preset_name(
                        SyntheticsTestOptionsMonitorOptionsNotificationPresetName::SHOW_ALL,
                    ),
                )
                .restricted_roles(vec!["xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx".to_string()])
                .retry(SyntheticsTestOptionsRetry::new())
                .scheduling(
                    SyntheticsTestOptionsScheduling::new(
                        vec![
                            SyntheticsTestOptionsSchedulingTimeframe::new(1, "07:00".to_string(), "16:00".to_string()),
                            SyntheticsTestOptionsSchedulingTimeframe::new(3, "07:00".to_string(), "16:00".to_string())
                        ],
                        "America/New_York".to_string(),
                    ),
                ),
            SyntheticsMobileTestType::MOBILE,
        )
            .device_ids(vec!["chrome.laptop_large".to_string()])
            .status(SyntheticsTestPauseStatus::LIVE)
            .steps(
                vec![
                    SyntheticsMobileStep::new(
                        "".to_string(),
                        SyntheticsMobileStepParams::new()
                            .check(SyntheticsCheckType::EQUALS)
                            .direction(SyntheticsMobileStepParamsDirection::UP)
                            .element(
                                SyntheticsMobileStepParamsElement::new()
                                    .context_type(SyntheticsMobileStepParamsElementContextType::NATIVE)
                                    .relative_position(SyntheticsMobileStepParamsElementRelativePosition::new())
                                    .user_locator(
                                        SyntheticsMobileStepParamsElementUserLocator
                                        ::new().values(
                                            vec![
                                                SyntheticsMobileStepParamsElementUserLocatorValuesItems
                                                ::new().type_(
                                                    SyntheticsMobileStepParamsElementUserLocatorValuesItemsType
                                                    ::ACCESSIBILITY_ID,
                                                )
                                            ],
                                        ),
                                    ),
                            )
                            .positions(vec![SyntheticsMobileStepParamsPositionsItems::new()])
                            .variable(
                                SyntheticsMobileStepParamsVariable::new("".to_string(), "VAR_NAME".to_string()),
                            ),
                        SyntheticsMobileStepType::ASSERTELEMENTCONTENT,
                    ).public_id("pub-lic-id0".to_string())
                ],
            )
            .tags(vec!["env:production".to_string()]);
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.update_mobile_test("public_id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

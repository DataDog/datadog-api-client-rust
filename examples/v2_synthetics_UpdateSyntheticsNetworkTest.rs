// Edit a Network Path test returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV2::model::SyntheticsNetworkAssertion;
use datadog_api_client::datadogV2::model::SyntheticsNetworkAssertionLatency;
use datadog_api_client::datadogV2::model::SyntheticsNetworkAssertionLatencyType;
use datadog_api_client::datadogV2::model::SyntheticsNetworkAssertionOperator;
use datadog_api_client::datadogV2::model::SyntheticsNetworkAssertionProperty;
use datadog_api_client::datadogV2::model::SyntheticsNetworkTest;
use datadog_api_client::datadogV2::model::SyntheticsNetworkTestConfig;
use datadog_api_client::datadogV2::model::SyntheticsNetworkTestEdit;
use datadog_api_client::datadogV2::model::SyntheticsNetworkTestEditRequest;
use datadog_api_client::datadogV2::model::SyntheticsNetworkTestRequest;
use datadog_api_client::datadogV2::model::SyntheticsNetworkTestRequestTCPMethod;
use datadog_api_client::datadogV2::model::SyntheticsNetworkTestSubType;
use datadog_api_client::datadogV2::model::SyntheticsNetworkTestType;
use datadog_api_client::datadogV2::model::SyntheticsTestOptions;
use datadog_api_client::datadogV2::model::SyntheticsTestOptionsMonitorOptions;
use datadog_api_client::datadogV2::model::SyntheticsTestOptionsMonitorOptionsNotificationPresetName;
use datadog_api_client::datadogV2::model::SyntheticsTestOptionsRetry;
use datadog_api_client::datadogV2::model::SyntheticsTestOptionsScheduling;
use datadog_api_client::datadogV2::model::SyntheticsTestOptionsSchedulingTimeframe;
use datadog_api_client::datadogV2::model::SyntheticsTestPauseStatus;

#[tokio::main]
async fn main() {
    let body = SyntheticsNetworkTestEditRequest::new(SyntheticsNetworkTestEdit::new(
        SyntheticsNetworkTest::new(
            SyntheticsNetworkTestConfig::new()
                .assertions(vec![
                    SyntheticsNetworkAssertion::SyntheticsNetworkAssertionLatency(Box::new(
                        SyntheticsNetworkAssertionLatency::new(
                            SyntheticsNetworkAssertionOperator::LESS_THAN,
                            SyntheticsNetworkAssertionProperty::AVG,
                            500.0,
                            SyntheticsNetworkAssertionLatencyType::LATENCY,
                        ),
                    )),
                ])
                .request(
                    SyntheticsNetworkTestRequest::new(50, "".to_string(), 30, 3)
                        .port(443)
                        .tcp_method(SyntheticsNetworkTestRequestTCPMethod::PREFER_SACK),
                ),
            vec![
                "aws:us-east-1".to_string(),
                "agent:my-agent-name".to_string(),
            ],
            "Network Path test notification".to_string(),
            "Example Network Path test".to_string(),
            SyntheticsTestOptions::new()
                .monitor_options(
                    SyntheticsTestOptionsMonitorOptions::new().notification_preset_name(
                        SyntheticsTestOptionsMonitorOptionsNotificationPresetName::SHOW_ALL,
                    ),
                )
                .restricted_roles(vec!["xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx".to_string()])
                .retry(SyntheticsTestOptionsRetry::new())
                .scheduling(SyntheticsTestOptionsScheduling::new(
                    vec![
                        SyntheticsTestOptionsSchedulingTimeframe::new(
                            1,
                            "07:00".to_string(),
                            "16:00".to_string(),
                        ),
                        SyntheticsTestOptionsSchedulingTimeframe::new(
                            3,
                            "07:00".to_string(),
                            "16:00".to_string(),
                        ),
                    ],
                    "America/New_York".to_string(),
                )),
            SyntheticsNetworkTestType::NETWORK,
        )
        .status(SyntheticsTestPauseStatus::LIVE)
        .subtype(SyntheticsNetworkTestSubType::TCP)
        .tags(vec!["env:production".to_string()]),
        SyntheticsNetworkTestType::NETWORK,
    ));
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api
        .update_synthetics_network_test("public_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

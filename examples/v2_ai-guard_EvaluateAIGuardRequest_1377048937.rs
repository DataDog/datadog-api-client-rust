// Evaluate an AI Guard request with per-category confidence thresholds
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_guard::AIGuardAPI;
use datadog_api_client::datadogV2::model::AIGuardConfidenceThreshold;
use datadog_api_client::datadogV2::model::AIGuardConfidenceThresholdOverride;
use datadog_api_client::datadogV2::model::AIGuardEvaluateRequest;
use datadog_api_client::datadogV2::model::AIGuardEvaluateRequestData;
use datadog_api_client::datadogV2::model::AIGuardEvaluateRequestDataAttributes;
use datadog_api_client::datadogV2::model::AIGuardEvaluationType;
use datadog_api_client::datadogV2::model::AIGuardMessage;
use datadog_api_client::datadogV2::model::AIGuardMessageContent;
use datadog_api_client::datadogV2::model::AIGuardMessageRole;
use datadog_api_client::datadogV2::model::AIGuardMeta;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = AIGuardEvaluateRequest::new(AIGuardEvaluateRequestData::new(
        AIGuardEvaluateRequestDataAttributes::new(vec![AIGuardMessage::new(
            AIGuardMessageRole::USER,
        )
        .content(AIGuardMessageContent::AIGuardMessageContentText(
            "Hello world!".to_string(),
        ))])
        .meta(
            AIGuardMeta::new().confidence_threshold(
                AIGuardConfidenceThresholdOverride::AIGuardConfidenceThreshold(Box::new(
                    AIGuardConfidenceThreshold::new()
                        .flagging_global(0.6 as f64)
                        .additional_properties(BTreeMap::from([
                            ("policy-evasion".to_string(), 0.9),
                            ("indirect-prompt-injection".to_string(), 0.4),
                        ])),
                )),
            ),
        ),
        AIGuardEvaluationType::EVALUATIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.evaluate_ai_guard_request", true);
    let api = AIGuardAPI::with_config(configuration);
    let resp = api.evaluate_ai_guard_request(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

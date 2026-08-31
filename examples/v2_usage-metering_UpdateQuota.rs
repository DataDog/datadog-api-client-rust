// Update a usage quota returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_usage_metering::UsageMeteringAPI;
use datadog_api_client::datadogV2::model::UsageQuotaType;
use datadog_api_client::datadogV2::model::UsageQuotaUpdateAttributes;
use datadog_api_client::datadogV2::model::UsageQuotaUpdateData;
use datadog_api_client::datadogV2::model::UsageQuotaUpdateRequest;

#[tokio::main]
async fn main() {
    let body = UsageQuotaUpdateRequest::new(UsageQuotaUpdateData::new(
        UsageQuotaUpdateAttributes::new()
            .enforced(Some(false))
            .usage_limit(Some(120000)),
        "MjAfYWlfY3JlZGl0c1911c2VyX2hhbmRsZTpfX0FMTF9f".to_string(),
        UsageQuotaType::QUOTAS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateQuota", true);
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .update_quota(
            "ai_credits".to_string(),
            "MjAfYWlfY3JlZGl0c1911c2VyX2hhbmRsZTpfX0FMTF9f".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

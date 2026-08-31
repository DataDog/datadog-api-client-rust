// Create or update usage quotas returns "OK. The response includes each item's
// result; see each item's `error` attribute for any that failed to write."
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_usage_metering::CreateQuotasOptionalParams;
use datadog_api_client::datadogV2::api_usage_metering::UsageMeteringAPI;
use datadog_api_client::datadogV2::model::UsageQuotaCreateAttributes;
use datadog_api_client::datadogV2::model::UsageQuotaCreateData;
use datadog_api_client::datadogV2::model::UsageQuotaType;
use datadog_api_client::datadogV2::model::UsageQuotasCreateRequest;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = UsageQuotasCreateRequest::new(vec![UsageQuotaCreateData::new(
        UsageQuotaCreateAttributes::new(true, 100000).scope(BTreeMap::from([(
            "user_handle".to_string(),
            "jane@example.com".to_string(),
        )])),
        UsageQuotaType::QUOTAS,
    )]);
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateQuotas", true);
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .create_quotas(
            "ai_credits".to_string(),
            body,
            CreateQuotasOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

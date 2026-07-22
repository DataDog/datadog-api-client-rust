// Add insights to a case returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::CaseInsight;
use datadog_api_client::datadogV2::model::CaseInsightType;
use datadog_api_client::datadogV2::model::CaseInsightsAttributes;
use datadog_api_client::datadogV2::model::CaseInsightsData;
use datadog_api_client::datadogV2::model::CaseInsightsRequest;
use datadog_api_client::datadogV2::model::CaseResourceType;

#[tokio::main]
async fn main() {
    let body = CaseInsightsRequest::new(CaseInsightsData::new(
        CaseInsightsAttributes::new(vec![CaseInsight::new(
            "/monitors/12345?q=total".to_string(),
            "12345".to_string(),
            CaseInsightType::SECURITY_SIGNAL,
        )]),
        CaseResourceType::CASE,
    ));
    let configuration = datadog::Configuration::new();
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api.add_case_insights("case_id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

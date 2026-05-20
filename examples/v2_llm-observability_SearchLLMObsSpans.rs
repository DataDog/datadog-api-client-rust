// Search LLM Observability spans returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsSearchSpansRequest;
use datadog_api_client::datadogV2::model::LLMObsSearchSpansRequestAttributes;
use datadog_api_client::datadogV2::model::LLMObsSearchSpansRequestData;
use datadog_api_client::datadogV2::model::LLMObsSearchSpansRequestType;
use datadog_api_client::datadogV2::model::LLMObsSpanFilter;
use datadog_api_client::datadogV2::model::LLMObsSpanPageQuery;
use datadog_api_client::datadogV2::model::LLMObsSpanSearchOptions;

#[tokio::main]
async fn main() {
    let body = LLMObsSearchSpansRequest::new(LLMObsSearchSpansRequestData::new(
        LLMObsSearchSpansRequestAttributes::new()
            .filter(
                LLMObsSpanFilter::new()
                    .from("now-900s".to_string())
                    .ml_app("my-llm-app".to_string())
                    .query("@session_id:abc123def456".to_string())
                    .span_id("abc123def456".to_string())
                    .span_kind("llm".to_string())
                    .span_name("llm_call".to_string())
                    .to("now".to_string())
                    .trace_id("trace-9a8b7c6d5e4f".to_string()),
            )
            .options(
                LLMObsSpanSearchOptions::new()
                    .include_attachments(true)
                    .time_offset(0),
            )
            .page(
                LLMObsSpanPageQuery::new()
                    .cursor("eyJzdGFydCI6MTAwfQ==".to_string())
                    .limit(10),
            )
            .sort("-start_ns".to_string()),
        LLMObsSearchSpansRequestType::SPANS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.SearchLLMObsSpans", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api.search_llm_obs_spans(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

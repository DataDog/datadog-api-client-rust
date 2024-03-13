// Send pipeline event returns "Request accepted for processing" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_ci_visibility_pipelines::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body = CIAppCreatePipelineEventRequest::new().data(
        CIAppCreatePipelineEventRequestData::new()
            .attributes(CIAppCreatePipelineEventRequestAttributes::new(
                CIAppCreatePipelineEventRequestAttributesResource::CIAppPipelineEventPipeline(
                    Box::new(
                        CIAppPipelineEventPipeline::new(
                            "2021-11-11T11:10:41+00:00".to_string(),
                            CIAppPipelineEventPipelineLevel::PIPELINE,
                            "Deploy to AWS".to_string(),
                            false,
                            "2021-11-11T11:09:11+00:00".to_string(),
                            CIAppPipelineEventPipelineStatus::SUCCESS,
                            "3eacb6f3-ff04-4e10-8a9c-46e6d054024a".to_string(),
                            "https://my-ci-provider.example/pipelines/my-pipeline/run/1"
                                .to_string(),
                        )
                        .git(Some(CIAppGitInfo::new(
                            "john.doe@email.com".to_string(),
                            "https://github.com/DataDog/datadog-agent".to_string(),
                            "7f263865994b76066c4612fd1965215e7dcb4cd2".to_string(),
                        ))),
                    ),
                ),
            ))
            .type_(CIAppCreatePipelineEventRequestDataType::CIPIPELINE_RESOURCE_REQUEST),
    );
    let configuration = Configuration::new();
    let api = CIVisibilityPipelinesAPI::with_config(configuration);
    let resp = api.create_ci_app_pipeline_event(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
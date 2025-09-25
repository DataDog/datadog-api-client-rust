// Send several pipeline events returns "Request accepted for processing" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ci_visibility_pipelines::CIVisibilityPipelinesAPI;
use datadog_api_client::datadogV2::model::CIAppCreatePipelineEventRequest;
use datadog_api_client::datadogV2::model::CIAppCreatePipelineEventRequestAttributes;
use datadog_api_client::datadogV2::model::CIAppCreatePipelineEventRequestAttributesResource;
use datadog_api_client::datadogV2::model::CIAppCreatePipelineEventRequestData;
use datadog_api_client::datadogV2::model::CIAppCreatePipelineEventRequestDataSingleOrArray;
use datadog_api_client::datadogV2::model::CIAppCreatePipelineEventRequestDataType;
use datadog_api_client::datadogV2::model::CIAppGitInfo;
use datadog_api_client::datadogV2::model::CIAppPipelineEventFinishedPipeline;
use datadog_api_client::datadogV2::model::CIAppPipelineEventPipeline;
use datadog_api_client::datadogV2::model::CIAppPipelineEventPipelineLevel;
use datadog_api_client::datadogV2::model::CIAppPipelineEventPipelineStatus;

#[tokio::main]
async fn main() {
    let body =
        CIAppCreatePipelineEventRequest
        ::new().data(
            CIAppCreatePipelineEventRequestDataSingleOrArray::CIAppCreatePipelineEventRequestDataArray(
                vec![
                    CIAppCreatePipelineEventRequestData::new()
                        .attributes(
                            CIAppCreatePipelineEventRequestAttributes::new(
                                CIAppCreatePipelineEventRequestAttributesResource::CIAppPipelineEventPipeline(
                                    Box::new(
                                        CIAppPipelineEventPipeline::CIAppPipelineEventFinishedPipeline(
                                            Box::new(
                                                CIAppPipelineEventFinishedPipeline::new(
                                                    DateTime::parse_from_rfc3339("2021-11-11T11:10:41+00:00")
                                                        .expect("Failed to parse datetime")
                                                        .with_timezone(&Utc),
                                                    CIAppPipelineEventPipelineLevel::PIPELINE,
                                                    "Deploy to AWS".to_string(),
                                                    false,
                                                    DateTime::parse_from_rfc3339("2021-11-11T11:09:11+00:00")
                                                        .expect("Failed to parse datetime")
                                                        .with_timezone(&Utc),
                                                    CIAppPipelineEventPipelineStatus::SUCCESS,
                                                    "3eacb6f3-ff04-4e10-8a9c-46e6d054024a".to_string(),
                                                    "https://my-ci-provider.example/pipelines/my-pipeline/run/1".to_string(),
                                                ).git(
                                                    Some(
                                                        CIAppGitInfo::new(
                                                            "john.doe@email.com".to_string(),
                                                            "https://github.com/DataDog/datadog-agent".to_string(),
                                                            "7f263865994b76066c4612fd1965215e7dcb4cd2".to_string(),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ).provider_name("example-provider".to_string()),
                        )
                        .type_(CIAppCreatePipelineEventRequestDataType::CIPIPELINE_RESOURCE_REQUEST),
                    CIAppCreatePipelineEventRequestData::new()
                        .attributes(
                            CIAppCreatePipelineEventRequestAttributes::new(
                                CIAppCreatePipelineEventRequestAttributesResource::CIAppPipelineEventPipeline(
                                    Box::new(
                                        CIAppPipelineEventPipeline::CIAppPipelineEventFinishedPipeline(
                                            Box::new(
                                                CIAppPipelineEventFinishedPipeline::new(
                                                    DateTime::parse_from_rfc3339("2021-11-11T11:10:26+00:00")
                                                        .expect("Failed to parse datetime")
                                                        .with_timezone(&Utc),
                                                    CIAppPipelineEventPipelineLevel::PIPELINE,
                                                    "Deploy to Production".to_string(),
                                                    false,
                                                    DateTime::parse_from_rfc3339("2021-11-11T11:08:11+00:00")
                                                        .expect("Failed to parse datetime")
                                                        .with_timezone(&Utc),
                                                    CIAppPipelineEventPipelineStatus::SUCCESS,
                                                    "7b2c8f9e-aa15-4d22-9c7d-83f4e065138b".to_string(),
                                                    "https://my-ci-provider.example/pipelines/prod-pipeline/run/2".to_string(),
                                                ).git(
                                                    Some(
                                                        CIAppGitInfo::new(
                                                            "jane.smith@email.com".to_string(),
                                                            "https://github.com/DataDog/datadog-agent".to_string(),
                                                            "9a4f7c28b3e5d12f8e6c9b2a5d8f3e1c7b4a6d9e".to_string(),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ).provider_name("example-provider".to_string()),
                        )
                        .type_(CIAppCreatePipelineEventRequestDataType::CIPIPELINE_RESOURCE_REQUEST)
                ],
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = CIVisibilityPipelinesAPI::with_config(configuration);
    let resp = api.create_ci_app_pipeline_event(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

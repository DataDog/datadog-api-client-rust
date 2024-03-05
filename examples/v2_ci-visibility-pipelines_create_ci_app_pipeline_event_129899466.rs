// Send pipeline job event returns "Request accepted for processing" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_ci_visibility_pipelines::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        CIAppCreatePipelineEventRequest
        ::new().data(
            CIAppCreatePipelineEventRequestData::new()
                .attributes(
                    CIAppCreatePipelineEventRequestAttributes::new(
                        CIAppCreatePipelineEventRequestAttributesResource::CIAppPipelineEventJob(
                            Box::new(
                                CIAppPipelineEventJob::new(
                                    (Utc::now() + chrono::Duration::seconds(-30)).to_rfc3339(),
                                    "cf9456de-8b9e-4c27-aa79-27b1e78c1a33".to_string(),
                                    CIAppPipelineEventJobLevel::JOB,
                                    "Build image".to_string(),
                                    "Deploy to AWS".to_string(),
                                    "3eacb6f3-ff04-4e10-8a9c-46e6d054024a".to_string(),
                                    (Utc::now() + chrono::Duration::seconds(-120)).to_rfc3339(),
                                    CIAppPipelineEventJobStatus::ERROR,
                                    "https://my-ci-provider.example/jobs/my-jobs/run/1".to_string(),
                                ),
                            ),
                        ),
                    ),
                )
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

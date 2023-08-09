/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// CiAppPipelineEventStageLevel : Used to distinguish between pipelines, stages, jobs and steps.

/// Used to distinguish between pipelines, stages, jobs and steps.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CiAppPipelineEventStageLevel {
    #[serde(rename = "stage")]
    STAGE,

}

impl ToString for CiAppPipelineEventStageLevel {
    fn to_string(&self) -> String {
        match self {
            Self::STAGE => String::from("stage"),
        }
    }
}

impl Default for CiAppPipelineEventStageLevel {
    fn default() -> CiAppPipelineEventStageLevel {
        Self::STAGE
    }
}





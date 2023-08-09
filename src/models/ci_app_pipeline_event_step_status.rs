/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// CiAppPipelineEventStepStatus : The final status of the step.

/// The final status of the step.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CiAppPipelineEventStepStatus {
    #[serde(rename = "success")]
    SUCCESS,
    #[serde(rename = "error")]
    ERROR,

}

impl ToString for CiAppPipelineEventStepStatus {
    fn to_string(&self) -> String {
        match self {
            Self::SUCCESS => String::from("success"),
            Self::ERROR => String::from("error"),
        }
    }
}

impl Default for CiAppPipelineEventStepStatus {
    fn default() -> CiAppPipelineEventStepStatus {
        Self::SUCCESS
    }
}





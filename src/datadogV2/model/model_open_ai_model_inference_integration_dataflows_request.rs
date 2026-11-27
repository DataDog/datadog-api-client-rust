// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data Datadog collects from OpenAI Model Inference, keyed by dataflow ID. Each dataflow turns on a distinct type of collection. Set `enabled` to start or stop a dataflow. The default for each dataflow applies when the account is created. On update, omitted fields keep their current values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpenAIModelInferenceIntegrationDataflowsRequest {
    /// Makes the key in `authentication` available to run evaluations in [LLM Observability](<https://docs.datadoghq.com/llm_observability/>). While this dataflow is enabled, the API returns a `422` response for a key that cannot call the OpenAI chat completions endpoint.
    #[serde(rename = "openai-llm-completions")]
    pub openai_llm_completions: Option<
        crate::datadogV2::model::OpenAIModelInferenceLLMCompletionsIntegrationDataflowRequest,
    >,
    /// Costs of the OpenAI project that owns the key in `authentication`. Analyze these costs in [Cloud Cost Management](<https://docs.datadoghq.com/cloud_cost_management/>). Cost data appears in Cloud Cost Management within 24 hours after you enable this dataflow.
    #[serde(rename = "openai-project-cloud-cost-data")]
    pub openai_project_cloud_cost_data: Option<
        crate::datadogV2::model::OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowRequest,
    >,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OpenAIModelInferenceIntegrationDataflowsRequest {
    pub fn new() -> OpenAIModelInferenceIntegrationDataflowsRequest {
        OpenAIModelInferenceIntegrationDataflowsRequest {
            openai_llm_completions: None,
            openai_project_cloud_cost_data: None,
            _unparsed: false,
        }
    }

    pub fn openai_llm_completions(
        mut self,
        value: crate::datadogV2::model::OpenAIModelInferenceLLMCompletionsIntegrationDataflowRequest,
    ) -> Self {
        self.openai_llm_completions = Some(value);
        self
    }

    pub fn openai_project_cloud_cost_data(
        mut self,
        value: crate::datadogV2::model::OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowRequest,
    ) -> Self {
        self.openai_project_cloud_cost_data = Some(value);
        self
    }
}

impl Default for OpenAIModelInferenceIntegrationDataflowsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OpenAIModelInferenceIntegrationDataflowsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenAIModelInferenceIntegrationDataflowsRequestVisitor;
        impl<'a> Visitor<'a> for OpenAIModelInferenceIntegrationDataflowsRequestVisitor {
            type Value = OpenAIModelInferenceIntegrationDataflowsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut openai_llm_completions: Option<crate::datadogV2::model::OpenAIModelInferenceLLMCompletionsIntegrationDataflowRequest> = None;
                let mut openai_project_cloud_cost_data: Option<crate::datadogV2::model::OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowRequest> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "openai-llm-completions" => {
                            if v.is_null() {
                                continue;
                            }
                            openai_llm_completions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "openai-project-cloud-cost-data" => {
                            if v.is_null() {
                                continue;
                            }
                            openai_project_cloud_cost_data =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = OpenAIModelInferenceIntegrationDataflowsRequest {
                    openai_llm_completions,
                    openai_project_cloud_cost_data,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OpenAIModelInferenceIntegrationDataflowsRequestVisitor)
    }
}

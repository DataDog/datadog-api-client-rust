// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data Datadog collects from OpenAI Model Inference, keyed by dataflow ID.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpenAIModelInferenceIntegrationDataflowsResponse {
    /// Makes the key in `authentication` available to run evaluations in [LLM Observability](<https://docs.datadoghq.com/llm_observability/>).
    #[serde(rename = "openai-llm-completions")]
    pub openai_llm_completions: Option<crate::datadogV2::model::OpenAIModelInferenceLLMCompletionsIntegrationDataflowResponse>,
    /// Costs of the OpenAI project that owns the key in `authentication`. Analyze these costs in [Cloud Cost Management](<https://docs.datadoghq.com/cloud_cost_management/>).
    #[serde(rename = "openai-project-cloud-cost-data")]
    pub openai_project_cloud_cost_data: Option<crate::datadogV2::model::OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowResponse>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl OpenAIModelInferenceIntegrationDataflowsResponse {
    pub fn new() -> OpenAIModelInferenceIntegrationDataflowsResponse {
        OpenAIModelInferenceIntegrationDataflowsResponse {
            openai_llm_completions: None,
            openai_project_cloud_cost_data: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn openai_llm_completions(
        mut self,
        value: crate::datadogV2::model::OpenAIModelInferenceLLMCompletionsIntegrationDataflowResponse,
    ) -> Self {
        self.openai_llm_completions = Some(value);
        self
    }

    pub fn openai_project_cloud_cost_data(
        mut self,
        value: crate::datadogV2::model::OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowResponse,
    ) -> Self {
        self.openai_project_cloud_cost_data = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for OpenAIModelInferenceIntegrationDataflowsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OpenAIModelInferenceIntegrationDataflowsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenAIModelInferenceIntegrationDataflowsResponseVisitor;
        impl<'a> Visitor<'a> for OpenAIModelInferenceIntegrationDataflowsResponseVisitor {
            type Value = OpenAIModelInferenceIntegrationDataflowsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut openai_llm_completions: Option<crate::datadogV2::model::OpenAIModelInferenceLLMCompletionsIntegrationDataflowResponse> = None;
                let mut openai_project_cloud_cost_data: Option<crate::datadogV2::model::OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowResponse> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = OpenAIModelInferenceIntegrationDataflowsResponse {
                    openai_llm_completions,
                    openai_project_cloud_cost_data,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OpenAIModelInferenceIntegrationDataflowsResponseVisitor)
    }
}

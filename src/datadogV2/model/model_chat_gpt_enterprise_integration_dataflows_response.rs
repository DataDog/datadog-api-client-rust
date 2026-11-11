// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data Datadog collects from ChatGPT Enterprise, keyed by dataflow id.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChatGPTEnterpriseIntegrationDataflowsResponse {
    /// Activity, token consumption, credits, and estimated usage costs across ChatGPT Chat, ChatGPT Work, and Codex, shown in Agent Console with breakdowns by product, model, and user where OpenAI provides them.
    #[serde(rename = "chatgpt-enterprise-agent-console")]
    pub chatgpt_enterprise_agent_console:
        Option<crate::datadogV2::model::ChatGPTEnterpriseAgentConsoleIntegrationDataflowResponse>,
    /// ChatGPT Enterprise costs from the OpenAI Costs API, so that ChatGPT Enterprise spend can be analyzed in [Cloud Cost Management](<https://docs.datadoghq.com/cloud_cost_management/>). These costs are excluded from Cloud Cost Management spend-monitored billing. Reported costs are estimates, not final invoice amounts.
    #[serde(rename = "chatgpt-enterprise-cloud-cost-data")]
    pub chatgpt_enterprise_cloud_cost_data:
        Option<crate::datadogV2::model::ChatGPTEnterpriseCloudCostDataIntegrationDataflowResponse>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ChatGPTEnterpriseIntegrationDataflowsResponse {
    pub fn new() -> ChatGPTEnterpriseIntegrationDataflowsResponse {
        ChatGPTEnterpriseIntegrationDataflowsResponse {
            chatgpt_enterprise_agent_console: None,
            chatgpt_enterprise_cloud_cost_data: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn chatgpt_enterprise_agent_console(
        mut self,
        value: crate::datadogV2::model::ChatGPTEnterpriseAgentConsoleIntegrationDataflowResponse,
    ) -> Self {
        self.chatgpt_enterprise_agent_console = Some(value);
        self
    }

    pub fn chatgpt_enterprise_cloud_cost_data(
        mut self,
        value: crate::datadogV2::model::ChatGPTEnterpriseCloudCostDataIntegrationDataflowResponse,
    ) -> Self {
        self.chatgpt_enterprise_cloud_cost_data = Some(value);
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

impl Default for ChatGPTEnterpriseIntegrationDataflowsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ChatGPTEnterpriseIntegrationDataflowsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChatGPTEnterpriseIntegrationDataflowsResponseVisitor;
        impl<'a> Visitor<'a> for ChatGPTEnterpriseIntegrationDataflowsResponseVisitor {
            type Value = ChatGPTEnterpriseIntegrationDataflowsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut chatgpt_enterprise_agent_console: Option<crate::datadogV2::model::ChatGPTEnterpriseAgentConsoleIntegrationDataflowResponse> = None;
                let mut chatgpt_enterprise_cloud_cost_data: Option<crate::datadogV2::model::ChatGPTEnterpriseCloudCostDataIntegrationDataflowResponse> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "chatgpt-enterprise-agent-console" => {
                            if v.is_null() {
                                continue;
                            }
                            chatgpt_enterprise_agent_console =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "chatgpt-enterprise-cloud-cost-data" => {
                            if v.is_null() {
                                continue;
                            }
                            chatgpt_enterprise_cloud_cost_data =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ChatGPTEnterpriseIntegrationDataflowsResponse {
                    chatgpt_enterprise_agent_console,
                    chatgpt_enterprise_cloud_cost_data,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChatGPTEnterpriseIntegrationDataflowsResponseVisitor)
    }
}

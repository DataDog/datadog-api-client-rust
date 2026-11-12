// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data Datadog collects from ChatGPT Enterprise, keyed by dataflow id. Each dataflow turns on a distinct kind of collection: set `enabled` to start or stop it. Defaults listed on each dataflow apply when the account is created; on update, omitted fields keep their current values. Every dataflow requires a ChatGPT Enterprise workspace with access to the OpenAI Costs API; a dataflow enabled without it is stored but collects no data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChatGPTEnterpriseIntegrationDataflowsRequest {
    /// Activity, token consumption, credits, and estimated usage costs across ChatGPT Chat, ChatGPT Work, and Codex, shown in Agent Console with breakdowns by product, model, and user where OpenAI provides them. Data already available from OpenAI appears within 1 hour of enabling this dataflow.
    #[serde(rename = "chatgpt-enterprise-agent-console")]
    pub chatgpt_enterprise_agent_console:
        Option<crate::datadogV2::model::ChatGPTEnterpriseAgentConsoleIntegrationDataflowRequest>,
    /// ChatGPT Enterprise costs from the OpenAI Costs API, so that ChatGPT Enterprise spend can be analyzed in [Cloud Cost Management](<https://docs.datadoghq.com/cloud_cost_management/>). These costs are excluded from Cloud Cost Management spend-monitored billing. Reported costs are estimates, not final invoice amounts.
    #[serde(rename = "chatgpt-enterprise-cloud-cost-data")]
    pub chatgpt_enterprise_cloud_cost_data:
        Option<crate::datadogV2::model::ChatGPTEnterpriseCloudCostDataIntegrationDataflowRequest>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ChatGPTEnterpriseIntegrationDataflowsRequest {
    pub fn new() -> ChatGPTEnterpriseIntegrationDataflowsRequest {
        ChatGPTEnterpriseIntegrationDataflowsRequest {
            chatgpt_enterprise_agent_console: None,
            chatgpt_enterprise_cloud_cost_data: None,
            _unparsed: false,
        }
    }

    pub fn chatgpt_enterprise_agent_console(
        mut self,
        value: crate::datadogV2::model::ChatGPTEnterpriseAgentConsoleIntegrationDataflowRequest,
    ) -> Self {
        self.chatgpt_enterprise_agent_console = Some(value);
        self
    }

    pub fn chatgpt_enterprise_cloud_cost_data(
        mut self,
        value: crate::datadogV2::model::ChatGPTEnterpriseCloudCostDataIntegrationDataflowRequest,
    ) -> Self {
        self.chatgpt_enterprise_cloud_cost_data = Some(value);
        self
    }
}

impl Default for ChatGPTEnterpriseIntegrationDataflowsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ChatGPTEnterpriseIntegrationDataflowsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChatGPTEnterpriseIntegrationDataflowsRequestVisitor;
        impl<'a> Visitor<'a> for ChatGPTEnterpriseIntegrationDataflowsRequestVisitor {
            type Value = ChatGPTEnterpriseIntegrationDataflowsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut chatgpt_enterprise_agent_console: Option<crate::datadogV2::model::ChatGPTEnterpriseAgentConsoleIntegrationDataflowRequest> = None;
                let mut chatgpt_enterprise_cloud_cost_data: Option<crate::datadogV2::model::ChatGPTEnterpriseCloudCostDataIntegrationDataflowRequest> = None;
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
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = ChatGPTEnterpriseIntegrationDataflowsRequest {
                    chatgpt_enterprise_agent_console,
                    chatgpt_enterprise_cloud_cost_data,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChatGPTEnterpriseIntegrationDataflowsRequestVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Activity, token consumption, credits, and estimated usage costs across ChatGPT Chat, ChatGPT Work, and Codex, shown in Agent Console with breakdowns by product, model, and user where OpenAI provides them. Data already available from OpenAI appears within 1 hour of enabling this dataflow.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChatGPTEnterpriseAgentConsoleIntegrationDataflowRequest {
    /// Whether Datadog collects this data. Defaults to `true`; set to `false` to stop collection.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ChatGPTEnterpriseAgentConsoleIntegrationDataflowRequest {
    pub fn new(enabled: bool) -> ChatGPTEnterpriseAgentConsoleIntegrationDataflowRequest {
        ChatGPTEnterpriseAgentConsoleIntegrationDataflowRequest {
            enabled,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for ChatGPTEnterpriseAgentConsoleIntegrationDataflowRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChatGPTEnterpriseAgentConsoleIntegrationDataflowRequestVisitor;
        impl<'a> Visitor<'a> for ChatGPTEnterpriseAgentConsoleIntegrationDataflowRequestVisitor {
            type Value = ChatGPTEnterpriseAgentConsoleIntegrationDataflowRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;

                let content =
                    ChatGPTEnterpriseAgentConsoleIntegrationDataflowRequest { enabled, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChatGPTEnterpriseAgentConsoleIntegrationDataflowRequestVisitor)
    }
}

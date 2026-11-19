// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings for creating the OpenAI Codex integration account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpenAICodexIntegrationAccountSettingsRequest {
    /// ID of the Codex workspace to collect data from.
    #[serde(rename = "codex_workspace_id")]
    pub codex_workspace_id: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OpenAICodexIntegrationAccountSettingsRequest {
    pub fn new(codex_workspace_id: String) -> OpenAICodexIntegrationAccountSettingsRequest {
        OpenAICodexIntegrationAccountSettingsRequest {
            codex_workspace_id,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for OpenAICodexIntegrationAccountSettingsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenAICodexIntegrationAccountSettingsRequestVisitor;
        impl<'a> Visitor<'a> for OpenAICodexIntegrationAccountSettingsRequestVisitor {
            type Value = OpenAICodexIntegrationAccountSettingsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut codex_workspace_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "codex_workspace_id" => {
                            codex_workspace_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let codex_workspace_id = codex_workspace_id
                    .ok_or_else(|| M::Error::missing_field("codex_workspace_id"))?;

                let content = OpenAICodexIntegrationAccountSettingsRequest {
                    codex_workspace_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OpenAICodexIntegrationAccountSettingsRequestVisitor)
    }
}

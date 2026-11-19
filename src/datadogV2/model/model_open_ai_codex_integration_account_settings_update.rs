// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings for updating the OpenAI Codex integration account. Only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpenAICodexIntegrationAccountSettingsUpdate {
    /// ID of the Codex workspace to collect data from.
    #[serde(rename = "codex_workspace_id")]
    pub codex_workspace_id: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OpenAICodexIntegrationAccountSettingsUpdate {
    pub fn new() -> OpenAICodexIntegrationAccountSettingsUpdate {
        OpenAICodexIntegrationAccountSettingsUpdate {
            codex_workspace_id: None,
            _unparsed: false,
        }
    }

    pub fn codex_workspace_id(mut self, value: String) -> Self {
        self.codex_workspace_id = Some(value);
        self
    }
}

impl Default for OpenAICodexIntegrationAccountSettingsUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OpenAICodexIntegrationAccountSettingsUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenAICodexIntegrationAccountSettingsUpdateVisitor;
        impl<'a> Visitor<'a> for OpenAICodexIntegrationAccountSettingsUpdateVisitor {
            type Value = OpenAICodexIntegrationAccountSettingsUpdate;

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
                            if v.is_null() {
                                continue;
                            }
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

                let content = OpenAICodexIntegrationAccountSettingsUpdate {
                    codex_workspace_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OpenAICodexIntegrationAccountSettingsUpdateVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings configured on the OpenAI Codex integration account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpenAICodexIntegrationAccountSettingsResponse {
    /// ID of the Codex workspace to collect data from.
    #[serde(rename = "codex_workspace_id")]
    pub codex_workspace_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OpenAICodexIntegrationAccountSettingsResponse {
    pub fn new(codex_workspace_id: String) -> OpenAICodexIntegrationAccountSettingsResponse {
        OpenAICodexIntegrationAccountSettingsResponse {
            codex_workspace_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for OpenAICodexIntegrationAccountSettingsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenAICodexIntegrationAccountSettingsResponseVisitor;
        impl<'a> Visitor<'a> for OpenAICodexIntegrationAccountSettingsResponseVisitor {
            type Value = OpenAICodexIntegrationAccountSettingsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut codex_workspace_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "codex_workspace_id" => {
                            codex_workspace_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let codex_workspace_id = codex_workspace_id
                    .ok_or_else(|| M::Error::missing_field("codex_workspace_id"))?;

                let content = OpenAICodexIntegrationAccountSettingsResponse {
                    codex_workspace_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OpenAICodexIntegrationAccountSettingsResponseVisitor)
    }
}

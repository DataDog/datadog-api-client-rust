// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating an LLM Observability prompt version. At least one of `description`, `labels`, or `env_ids` must be provided; all three attributes are optional individually.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsUpdatePromptVersionDataAttributes {
    /// Optional new description for this version.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Optional feature-flag environment UUIDs the service attempts to enable and configure to use this version as their default.
    #[serde(rename = "env_ids")]
    pub env_ids: Option<Vec<String>>,
    /// Optional new labels for this version. Do not use this attribute for new integrations.
    #[deprecated]
    #[serde(rename = "labels")]
    pub labels: Option<Vec<crate::datadogV2::model::LLMObsPromptVersionLabel>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsUpdatePromptVersionDataAttributes {
    pub fn new() -> LLMObsUpdatePromptVersionDataAttributes {
        #[allow(deprecated)]
        LLMObsUpdatePromptVersionDataAttributes {
            description: None,
            env_ids: None,
            labels: None,
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn env_ids(mut self, value: Vec<String>) -> Self {
        self.env_ids = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn labels(mut self, value: Vec<crate::datadogV2::model::LLMObsPromptVersionLabel>) -> Self {
        self.labels = Some(value);
        self
    }
}

impl Default for LLMObsUpdatePromptVersionDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsUpdatePromptVersionDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsUpdatePromptVersionDataAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsUpdatePromptVersionDataAttributesVisitor {
            type Value = LLMObsUpdatePromptVersionDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut env_ids: Option<Vec<String>> = None;
                let mut labels: Option<Vec<crate::datadogV2::model::LLMObsPromptVersionLabel>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "env_ids" => {
                            if v.is_null() {
                                continue;
                            }
                            env_ids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "labels" => {
                            if v.is_null() {
                                continue;
                            }
                            labels = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                #[allow(deprecated)]
                let content = LLMObsUpdatePromptVersionDataAttributes {
                    description,
                    env_ids,
                    labels,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsUpdatePromptVersionDataAttributesVisitor)
    }
}

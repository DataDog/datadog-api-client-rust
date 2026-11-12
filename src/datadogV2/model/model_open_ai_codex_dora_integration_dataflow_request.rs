// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A daily per-engineer engagement signal that [AI Impact](<https://docs.datadoghq.com/delivery_performance/ai_impact/>) uses to attribute AI-assisted pull requests.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpenAICodexDoraIntegrationDataflowRequest {
    /// Whether Datadog collects this data. Defaults to `true`; set to `false` to stop collection.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OpenAICodexDoraIntegrationDataflowRequest {
    pub fn new() -> OpenAICodexDoraIntegrationDataflowRequest {
        OpenAICodexDoraIntegrationDataflowRequest {
            enabled: None,
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
}

impl Default for OpenAICodexDoraIntegrationDataflowRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OpenAICodexDoraIntegrationDataflowRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenAICodexDoraIntegrationDataflowRequestVisitor;
        impl<'a> Visitor<'a> for OpenAICodexDoraIntegrationDataflowRequestVisitor {
            type Value = OpenAICodexDoraIntegrationDataflowRequest;

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
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = OpenAICodexDoraIntegrationDataflowRequest { enabled, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OpenAICodexDoraIntegrationDataflowRequestVisitor)
    }
}

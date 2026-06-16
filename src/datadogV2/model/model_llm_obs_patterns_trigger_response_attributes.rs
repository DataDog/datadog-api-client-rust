// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an LLM Observability patterns trigger response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsPatternsTriggerResponseAttributes {
    /// The ID of the patterns configuration that was run.
    #[serde(rename = "config_id")]
    pub config_id: String,
    /// The ID of the patterns run that was started.
    #[serde(rename = "run_id")]
    pub run_id: String,
    /// Status of the patterns run.
    #[serde(rename = "status")]
    pub status: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsPatternsTriggerResponseAttributes {
    pub fn new(
        config_id: String,
        run_id: String,
        status: String,
    ) -> LLMObsPatternsTriggerResponseAttributes {
        LLMObsPatternsTriggerResponseAttributes {
            config_id,
            run_id,
            status,
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

impl<'de> Deserialize<'de> for LLMObsPatternsTriggerResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsPatternsTriggerResponseAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsPatternsTriggerResponseAttributesVisitor {
            type Value = LLMObsPatternsTriggerResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut config_id: Option<String> = None;
                let mut run_id: Option<String> = None;
                let mut status: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "config_id" => {
                            config_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "run_id" => {
                            run_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let config_id = config_id.ok_or_else(|| M::Error::missing_field("config_id"))?;
                let run_id = run_id.ok_or_else(|| M::Error::missing_field("run_id"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = LLMObsPatternsTriggerResponseAttributes {
                    config_id,
                    run_id,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsPatternsTriggerResponseAttributesVisitor)
    }
}

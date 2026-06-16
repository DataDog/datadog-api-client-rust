// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an LLM Observability patterns run status.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsPatternsRunStatusResponseAttributes {
    /// Timestamp when the run was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// List of step-by-step progress entries for a patterns run.
    #[serde(rename = "progress")]
    pub progress: Vec<crate::datadogV2::model::LLMObsPatternsActivityProgress>,
    /// Overall status of the run.
    #[serde(rename = "status")]
    pub status: String,
    /// The current step of the run.
    #[serde(rename = "step")]
    pub step: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsPatternsRunStatusResponseAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        progress: Vec<crate::datadogV2::model::LLMObsPatternsActivityProgress>,
        status: String,
        step: String,
    ) -> LLMObsPatternsRunStatusResponseAttributes {
        LLMObsPatternsRunStatusResponseAttributes {
            created_at,
            progress,
            status,
            step,
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

impl<'de> Deserialize<'de> for LLMObsPatternsRunStatusResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsPatternsRunStatusResponseAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsPatternsRunStatusResponseAttributesVisitor {
            type Value = LLMObsPatternsRunStatusResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut progress: Option<
                    Vec<crate::datadogV2::model::LLMObsPatternsActivityProgress>,
                > = None;
                let mut status: Option<String> = None;
                let mut step: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "progress" => {
                            progress = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "step" => {
                            step = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let progress = progress.ok_or_else(|| M::Error::missing_field("progress"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let step = step.ok_or_else(|| M::Error::missing_field("step"))?;

                let content = LLMObsPatternsRunStatusResponseAttributes {
                    created_at,
                    progress,
                    status,
                    step,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsPatternsRunStatusResponseAttributesVisitor)
    }
}

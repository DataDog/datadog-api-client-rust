// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Optional metadata providing context about the originating service and request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AIGuardMeta {
    /// Identifier of the coding agent sending the request, if applicable.
    #[serde(rename = "coding_agent")]
    pub coding_agent: Option<String>,
    /// Override for the default threat detection confidence threshold, between 0.0 and 1.0.
    #[serde(rename = "confidence_threshold")]
    pub confidence_threshold: Option<f64>,
    /// The deployment environment of the originating service.
    #[serde(rename = "env")]
    pub env: Option<String>,
    /// Override whether sensitive data scanning is applied to this request.
    #[serde(rename = "is_sds_enabled_override")]
    pub is_sds_enabled_override: Option<bool>,
    /// The name of the service sending the evaluation request.
    #[serde(rename = "service")]
    pub service: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AIGuardMeta {
    pub fn new() -> AIGuardMeta {
        AIGuardMeta {
            coding_agent: None,
            confidence_threshold: None,
            env: None,
            is_sds_enabled_override: None,
            service: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn coding_agent(mut self, value: String) -> Self {
        self.coding_agent = Some(value);
        self
    }

    pub fn confidence_threshold(mut self, value: f64) -> Self {
        self.confidence_threshold = Some(value);
        self
    }

    pub fn env(mut self, value: String) -> Self {
        self.env = Some(value);
        self
    }

    pub fn is_sds_enabled_override(mut self, value: bool) -> Self {
        self.is_sds_enabled_override = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
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

impl Default for AIGuardMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AIGuardMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AIGuardMetaVisitor;
        impl<'a> Visitor<'a> for AIGuardMetaVisitor {
            type Value = AIGuardMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut coding_agent: Option<String> = None;
                let mut confidence_threshold: Option<f64> = None;
                let mut env: Option<String> = None;
                let mut is_sds_enabled_override: Option<bool> = None;
                let mut service: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "coding_agent" => {
                            if v.is_null() {
                                continue;
                            }
                            coding_agent =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "confidence_threshold" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            confidence_threshold =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "env" => {
                            if v.is_null() {
                                continue;
                            }
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_sds_enabled_override" => {
                            if v.is_null() {
                                continue;
                            }
                            is_sds_enabled_override =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AIGuardMeta {
                    coding_agent,
                    confidence_threshold,
                    env,
                    is_sds_enabled_override,
                    service,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AIGuardMetaVisitor)
    }
}

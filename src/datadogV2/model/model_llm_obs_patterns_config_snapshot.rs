// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Snapshot of the configuration used for a patterns run.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsPatternsConfigSnapshot {
    /// Integration account ID used for a bring-your-own-model run.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// Query that selected the spans for the run.
    #[serde(rename = "evp_query")]
    pub evp_query: Option<String>,
    /// Depth of the topic hierarchy generated.
    #[serde(rename = "hierarchy_depth")]
    pub hierarchy_depth: Option<i32>,
    /// Integration provider used for a bring-your-own-model run.
    #[serde(rename = "integration_provider")]
    pub integration_provider: Option<String>,
    /// Model name used for a bring-your-own-model run.
    #[serde(rename = "model_name")]
    pub model_name: Option<String>,
    /// Maximum number of records processed for the run.
    #[serde(rename = "num_records")]
    pub num_records: Option<i32>,
    /// Fraction of matching spans sampled for the run.
    #[serde(rename = "sampling_ratio")]
    pub sampling_ratio: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsPatternsConfigSnapshot {
    pub fn new() -> LLMObsPatternsConfigSnapshot {
        LLMObsPatternsConfigSnapshot {
            account_id: None,
            evp_query: None,
            hierarchy_depth: None,
            integration_provider: None,
            model_name: None,
            num_records: None,
            sampling_ratio: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_id(mut self, value: String) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn evp_query(mut self, value: String) -> Self {
        self.evp_query = Some(value);
        self
    }

    pub fn hierarchy_depth(mut self, value: i32) -> Self {
        self.hierarchy_depth = Some(value);
        self
    }

    pub fn integration_provider(mut self, value: String) -> Self {
        self.integration_provider = Some(value);
        self
    }

    pub fn model_name(mut self, value: String) -> Self {
        self.model_name = Some(value);
        self
    }

    pub fn num_records(mut self, value: i32) -> Self {
        self.num_records = Some(value);
        self
    }

    pub fn sampling_ratio(mut self, value: f64) -> Self {
        self.sampling_ratio = Some(value);
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

impl Default for LLMObsPatternsConfigSnapshot {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsPatternsConfigSnapshot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsPatternsConfigSnapshotVisitor;
        impl<'a> Visitor<'a> for LLMObsPatternsConfigSnapshotVisitor {
            type Value = LLMObsPatternsConfigSnapshot;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut evp_query: Option<String> = None;
                let mut hierarchy_depth: Option<i32> = None;
                let mut integration_provider: Option<String> = None;
                let mut model_name: Option<String> = None;
                let mut num_records: Option<i32> = None;
                let mut sampling_ratio: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            if v.is_null() {
                                continue;
                            }
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evp_query" => {
                            if v.is_null() {
                                continue;
                            }
                            evp_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hierarchy_depth" => {
                            if v.is_null() {
                                continue;
                            }
                            hierarchy_depth =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_provider" => {
                            if v.is_null() {
                                continue;
                            }
                            integration_provider =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model_name" => {
                            if v.is_null() {
                                continue;
                            }
                            model_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "num_records" => {
                            if v.is_null() {
                                continue;
                            }
                            num_records =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sampling_ratio" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            sampling_ratio =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LLMObsPatternsConfigSnapshot {
                    account_id,
                    evp_query,
                    hierarchy_depth,
                    integration_provider,
                    model_name,
                    num_records,
                    sampling_ratio,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsPatternsConfigSnapshotVisitor)
    }
}

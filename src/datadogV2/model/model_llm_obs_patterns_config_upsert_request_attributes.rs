// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating or updating an LLM Observability patterns configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsPatternsConfigUpsertRequestAttributes {
    /// Integration account ID for a bring-your-own-model configuration.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// The ID of an existing configuration to update. If omitted, a new configuration is created.
    #[serde(rename = "config_id")]
    pub config_id: Option<String>,
    /// Query that selects the spans the patterns run analyzes.
    #[serde(rename = "evp_query")]
    pub evp_query: String,
    /// Depth of the topic hierarchy to generate.
    #[serde(rename = "hierarchy_depth")]
    pub hierarchy_depth: i32,
    /// Integration provider for a bring-your-own-model configuration.
    #[serde(rename = "integration_provider")]
    pub integration_provider: Option<String>,
    /// Model name for a bring-your-own-model configuration.
    #[serde(rename = "model_name")]
    pub model_name: Option<String>,
    /// Name of the configuration.
    #[serde(rename = "name")]
    pub name: String,
    /// Maximum number of records to process for the run.
    #[serde(rename = "num_records")]
    pub num_records: i32,
    /// Fraction of matching spans to sample for the run.
    #[serde(rename = "sampling_ratio")]
    pub sampling_ratio: f64,
    /// Scope of the configuration.
    #[serde(rename = "scope")]
    pub scope: Option<String>,
    /// Template used to guide topic generation.
    #[serde(rename = "template")]
    pub template: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsPatternsConfigUpsertRequestAttributes {
    pub fn new(
        evp_query: String,
        hierarchy_depth: i32,
        name: String,
        num_records: i32,
        sampling_ratio: f64,
    ) -> LLMObsPatternsConfigUpsertRequestAttributes {
        LLMObsPatternsConfigUpsertRequestAttributes {
            account_id: None,
            config_id: None,
            evp_query,
            hierarchy_depth,
            integration_provider: None,
            model_name: None,
            name,
            num_records,
            sampling_ratio,
            scope: None,
            template: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_id(mut self, value: String) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn config_id(mut self, value: String) -> Self {
        self.config_id = Some(value);
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

    pub fn scope(mut self, value: String) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn template(mut self, value: String) -> Self {
        self.template = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsPatternsConfigUpsertRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsPatternsConfigUpsertRequestAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsPatternsConfigUpsertRequestAttributesVisitor {
            type Value = LLMObsPatternsConfigUpsertRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut config_id: Option<String> = None;
                let mut evp_query: Option<String> = None;
                let mut hierarchy_depth: Option<i32> = None;
                let mut integration_provider: Option<String> = None;
                let mut model_name: Option<String> = None;
                let mut name: Option<String> = None;
                let mut num_records: Option<i32> = None;
                let mut sampling_ratio: Option<f64> = None;
                let mut scope: Option<String> = None;
                let mut template: Option<String> = None;
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
                        "config_id" => {
                            if v.is_null() {
                                continue;
                            }
                            config_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evp_query" => {
                            evp_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hierarchy_depth" => {
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
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "num_records" => {
                            num_records =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sampling_ratio" => {
                            sampling_ratio =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "template" => {
                            if v.is_null() {
                                continue;
                            }
                            template = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let evp_query = evp_query.ok_or_else(|| M::Error::missing_field("evp_query"))?;
                let hierarchy_depth =
                    hierarchy_depth.ok_or_else(|| M::Error::missing_field("hierarchy_depth"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let num_records =
                    num_records.ok_or_else(|| M::Error::missing_field("num_records"))?;
                let sampling_ratio =
                    sampling_ratio.ok_or_else(|| M::Error::missing_field("sampling_ratio"))?;

                let content = LLMObsPatternsConfigUpsertRequestAttributes {
                    account_id,
                    config_id,
                    evp_query,
                    hierarchy_depth,
                    integration_provider,
                    model_name,
                    name,
                    num_records,
                    sampling_ratio,
                    scope,
                    template,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsPatternsConfigUpsertRequestAttributesVisitor)
    }
}

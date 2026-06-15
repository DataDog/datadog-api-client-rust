// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an LLM Observability patterns configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsPatternsConfigAttributes {
    /// Integration account ID for a bring-your-own-model configuration.
    #[serde(
        rename = "account_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub account_id: Option<Option<String>>,
    /// Timestamp when the configuration was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Query that selects the spans the patterns run analyzes.
    #[serde(rename = "evp_query")]
    pub evp_query: String,
    /// Depth of the topic hierarchy to generate.
    #[serde(rename = "hierarchy_depth")]
    pub hierarchy_depth: i32,
    /// Integration provider for a bring-your-own-model configuration.
    #[serde(
        rename = "integration_provider",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub integration_provider: Option<Option<String>>,
    /// Model name for a bring-your-own-model configuration.
    #[serde(
        rename = "model_name",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub model_name: Option<Option<String>>,
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
    pub scope: String,
    /// Template used to guide topic generation.
    #[serde(
        rename = "template",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub template: Option<Option<String>>,
    /// Timestamp when the configuration was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsPatternsConfigAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        evp_query: String,
        hierarchy_depth: i32,
        name: String,
        num_records: i32,
        sampling_ratio: f64,
        scope: String,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> LLMObsPatternsConfigAttributes {
        LLMObsPatternsConfigAttributes {
            account_id: None,
            created_at,
            evp_query,
            hierarchy_depth,
            integration_provider: None,
            model_name: None,
            name,
            num_records,
            sampling_ratio,
            scope,
            template: None,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_id(mut self, value: Option<String>) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn integration_provider(mut self, value: Option<String>) -> Self {
        self.integration_provider = Some(value);
        self
    }

    pub fn model_name(mut self, value: Option<String>) -> Self {
        self.model_name = Some(value);
        self
    }

    pub fn template(mut self, value: Option<String>) -> Self {
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

impl<'de> Deserialize<'de> for LLMObsPatternsConfigAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsPatternsConfigAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsPatternsConfigAttributesVisitor {
            type Value = LLMObsPatternsConfigAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<Option<String>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut evp_query: Option<String> = None;
                let mut hierarchy_depth: Option<i32> = None;
                let mut integration_provider: Option<Option<String>> = None;
                let mut model_name: Option<Option<String>> = None;
                let mut name: Option<String> = None;
                let mut num_records: Option<i32> = None;
                let mut sampling_ratio: Option<f64> = None;
                let mut scope: Option<String> = None;
                let mut template: Option<Option<String>> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evp_query" => {
                            evp_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hierarchy_depth" => {
                            hierarchy_depth =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_provider" => {
                            integration_provider =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model_name" => {
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
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "template" => {
                            template = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let evp_query = evp_query.ok_or_else(|| M::Error::missing_field("evp_query"))?;
                let hierarchy_depth =
                    hierarchy_depth.ok_or_else(|| M::Error::missing_field("hierarchy_depth"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let num_records =
                    num_records.ok_or_else(|| M::Error::missing_field("num_records"))?;
                let sampling_ratio =
                    sampling_ratio.ok_or_else(|| M::Error::missing_field("sampling_ratio"))?;
                let scope = scope.ok_or_else(|| M::Error::missing_field("scope"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = LLMObsPatternsConfigAttributes {
                    account_id,
                    created_at,
                    evp_query,
                    hierarchy_depth,
                    integration_provider,
                    model_name,
                    name,
                    num_records,
                    sampling_ratio,
                    scope,
                    template,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsPatternsConfigAttributesVisitor)
    }
}

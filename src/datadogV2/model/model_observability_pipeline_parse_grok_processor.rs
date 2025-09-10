// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `parse_grok` processor extracts structured fields from unstructured log messages using Grok patterns.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineParseGrokProcessor {
    /// If set to `true`, disables the default Grok rules provided by Datadog.
    #[serde(rename = "disable_library_rules")]
    pub disable_library_rules: Option<bool>,
    /// The processor passes through all events if it is set to `false`. Defaults to `true`.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// A unique identifier for this processor.
    #[serde(rename = "id")]
    pub id: String,
    /// A Datadog search query used to determine which logs this processor targets.
    #[serde(rename = "include")]
    pub include: String,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// The list of Grok parsing rules. If multiple matching rules are provided, they are evaluated in order. The first successful match is applied.
    #[serde(rename = "rules")]
    pub rules: Vec<crate::datadogV2::model::ObservabilityPipelineParseGrokProcessorRule>,
    /// The processor type. The value should always be `parse_grok`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineParseGrokProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineParseGrokProcessor {
    pub fn new(
        id: String,
        include: String,
        inputs: Vec<String>,
        rules: Vec<crate::datadogV2::model::ObservabilityPipelineParseGrokProcessorRule>,
        type_: crate::datadogV2::model::ObservabilityPipelineParseGrokProcessorType,
    ) -> ObservabilityPipelineParseGrokProcessor {
        ObservabilityPipelineParseGrokProcessor {
            disable_library_rules: None,
            enabled: None,
            id,
            include,
            inputs,
            rules,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn disable_library_rules(mut self, value: bool) -> Self {
        self.disable_library_rules = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineParseGrokProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineParseGrokProcessorVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineParseGrokProcessorVisitor {
            type Value = ObservabilityPipelineParseGrokProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut disable_library_rules: Option<bool> = None;
                let mut enabled: Option<bool> = None;
                let mut id: Option<String> = None;
                let mut include: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut rules: Option<
                    Vec<crate::datadogV2::model::ObservabilityPipelineParseGrokProcessorRule>,
                > = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineParseGrokProcessorType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "disable_library_rules" => {
                            if v.is_null() {
                                continue;
                            }
                            disable_library_rules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include" => {
                            include = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rules" => {
                            rules = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineParseGrokProcessorType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let include = include.ok_or_else(|| M::Error::missing_field("include"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let rules = rules.ok_or_else(|| M::Error::missing_field("rules"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineParseGrokProcessor {
                    disable_library_rules,
                    enabled,
                    id,
                    include,
                    inputs,
                    rules,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineParseGrokProcessorVisitor)
    }
}

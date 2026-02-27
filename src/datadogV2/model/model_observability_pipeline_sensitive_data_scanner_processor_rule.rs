// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines a rule for detecting sensitive data, including matching pattern, scope, and the action to take.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSensitiveDataScannerProcessorRule {
    /// Configuration for keywords used to reinforce sensitive data pattern detection.
    #[serde(rename = "keyword_options")]
    pub keyword_options: Option<
        crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorKeywordOptions,
    >,
    /// A name identifying the rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Defines what action to take when sensitive data is matched.
    #[serde(rename = "on_match")]
    pub on_match: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorAction,
    /// Pattern detection configuration for identifying sensitive data using either a custom regex or a library reference.
    #[serde(rename = "pattern")]
    pub pattern: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorPattern,
    /// Determines which parts of the log the pattern-matching rule should be applied to.
    #[serde(rename = "scope")]
    pub scope: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorScope,
    /// Optional tags assigned to this rule for filtering and classification.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineSensitiveDataScannerProcessorRule {
    pub fn new(
        name: String,
        on_match: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorAction,
        pattern: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorPattern,
        scope: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorScope,
    ) -> ObservabilityPipelineSensitiveDataScannerProcessorRule {
        ObservabilityPipelineSensitiveDataScannerProcessorRule {
            keyword_options: None,
            name,
            on_match,
            pattern,
            scope,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn keyword_options(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorKeywordOptions,
    ) -> Self {
        self.keyword_options = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineSensitiveDataScannerProcessorRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSensitiveDataScannerProcessorRuleVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineSensitiveDataScannerProcessorRuleVisitor {
            type Value = ObservabilityPipelineSensitiveDataScannerProcessorRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut keyword_options: Option<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorKeywordOptions> = None;
                let mut name: Option<String> = None;
                let mut on_match: Option<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorAction> = None;
                let mut pattern: Option<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorPattern> = None;
                let mut scope: Option<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorScope> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "keyword_options" => {
                            if v.is_null() {
                                continue;
                            }
                            keyword_options =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "on_match" => {
                            on_match = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _on_match) = on_match {
                                match _on_match {
                                    crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorAction::UnparsedObject(_on_match) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "pattern" => {
                            pattern = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _pattern) = pattern {
                                match _pattern {
                                    crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorPattern::UnparsedObject(_pattern) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "scope" => {
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _scope) = scope {
                                match _scope {
                                    crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorScope::UnparsedObject(_scope) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let on_match = on_match.ok_or_else(|| M::Error::missing_field("on_match"))?;
                let pattern = pattern.ok_or_else(|| M::Error::missing_field("pattern"))?;
                let scope = scope.ok_or_else(|| M::Error::missing_field("scope"))?;

                let content = ObservabilityPipelineSensitiveDataScannerProcessorRule {
                    keyword_options,
                    name,
                    on_match,
                    pattern,
                    scope,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineSensitiveDataScannerProcessorRuleVisitor)
    }
}

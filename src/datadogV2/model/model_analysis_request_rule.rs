// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A static analysis rule to apply during code analysis.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnalysisRequestRule {
    /// The category of the rule (for example, `BEST_PRACTICES`, `SECURITY`).
    #[serde(rename = "category")]
    pub category: String,
    /// A checksum of the rule definition.
    #[serde(rename = "checksum")]
    pub checksum: String,
    /// The base64-encoded rule implementation code.
    #[serde(rename = "code")]
    pub code: String,
    /// The code entity type checked by the rule, applicable when rule type is `AST_CHECK`.
    #[serde(
        rename = "entity_checked",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub entity_checked: Option<Option<String>>,
    /// The unique identifier of the rule.
    #[serde(rename = "id")]
    pub id: String,
    /// The programming language this rule targets.
    #[serde(rename = "language")]
    pub language: String,
    /// A base64-encoded regex pattern used by the rule, applicable when rule type is `REGEX`.
    #[serde(rename = "regex", default, with = "::serde_with::rust::double_option")]
    pub regex: Option<Option<String>>,
    /// The severity of findings from this rule (for example, `ERROR`, `WARNING`).
    #[serde(rename = "severity")]
    pub severity: String,
    /// The base64-encoded tree-sitter query used by the rule.
    #[serde(rename = "tree_sitter_query")]
    pub tree_sitter_query: String,
    /// The rule type indicating the detection mechanism (for example, `TREE_SITTER_QUERY`).
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AnalysisRequestRule {
    pub fn new(
        category: String,
        checksum: String,
        code: String,
        id: String,
        language: String,
        severity: String,
        tree_sitter_query: String,
        type_: String,
    ) -> AnalysisRequestRule {
        AnalysisRequestRule {
            category,
            checksum,
            code,
            entity_checked: None,
            id,
            language,
            regex: None,
            severity,
            tree_sitter_query,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn entity_checked(mut self, value: Option<String>) -> Self {
        self.entity_checked = Some(value);
        self
    }

    pub fn regex(mut self, value: Option<String>) -> Self {
        self.regex = Some(value);
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

impl<'de> Deserialize<'de> for AnalysisRequestRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnalysisRequestRuleVisitor;
        impl<'a> Visitor<'a> for AnalysisRequestRuleVisitor {
            type Value = AnalysisRequestRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<String> = None;
                let mut checksum: Option<String> = None;
                let mut code: Option<String> = None;
                let mut entity_checked: Option<Option<String>> = None;
                let mut id: Option<String> = None;
                let mut language: Option<String> = None;
                let mut regex: Option<Option<String>> = None;
                let mut severity: Option<String> = None;
                let mut tree_sitter_query: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "checksum" => {
                            checksum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "code" => {
                            code = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entity_checked" => {
                            entity_checked =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "language" => {
                            language = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "regex" => {
                            regex = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severity" => {
                            severity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tree_sitter_query" => {
                            tree_sitter_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let category = category.ok_or_else(|| M::Error::missing_field("category"))?;
                let checksum = checksum.ok_or_else(|| M::Error::missing_field("checksum"))?;
                let code = code.ok_or_else(|| M::Error::missing_field("code"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let language = language.ok_or_else(|| M::Error::missing_field("language"))?;
                let severity = severity.ok_or_else(|| M::Error::missing_field("severity"))?;
                let tree_sitter_query = tree_sitter_query
                    .ok_or_else(|| M::Error::missing_field("tree_sitter_query"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = AnalysisRequestRule {
                    category,
                    checksum,
                    code,
                    entity_checked,
                    id,
                    language,
                    regex,
                    severity,
                    tree_sitter_query,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AnalysisRequestRuleVisitor)
    }
}

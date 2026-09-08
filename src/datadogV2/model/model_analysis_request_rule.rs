// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A static analysis rule to apply during code analysis. Clients forward complete rule
/// objects returned by the rulesets endpoints, so every member of that resource is
/// declared here; only `id`, `category`, `checksum`, `language`, `severity`,
/// `tree_sitter_query`, `entity_checked`, `regex`, `type` and `code` are read by this
/// operation and the rest are ignored. The schema stays open so that any member beyond
/// the forwarded rule resource is reported as a promotion candidate rather than
/// rejected; it can be closed once that telemetry confirms none remain.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnalysisRequestRule {
    /// The configurable arguments accepted by the rule. Forwarded from the rulesets endpoints; ignored by this operation.
    #[serde(rename = "arguments")]
    pub arguments: Option<Vec<crate::datadogV2::model::AnalysisRequestRuleArgument>>,
    /// The category of the rule (for example, `BEST_PRACTICES`, `SECURITY`).
    #[serde(rename = "category")]
    pub category: String,
    /// A checksum of the rule definition.
    #[serde(rename = "checksum")]
    pub checksum: String,
    /// The base64-encoded rule implementation code.
    #[serde(rename = "code")]
    pub code: String,
    /// The date and time when the rule was created. Server-assigned by the rulesets endpoints; ignored by this operation.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The identifier of the user or system that created the rule. Server-assigned by the rulesets endpoints; ignored by this operation.
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// The CVE identifier associated with the rule. Forwarded from the rulesets endpoints; ignored by this operation.
    #[serde(rename = "cve")]
    pub cve: Option<String>,
    /// The CWE identifier associated with the rule. Forwarded from the rulesets endpoints; ignored by this operation.
    #[serde(rename = "cwe")]
    pub cwe: Option<String>,
    /// A detailed explanation of what the rule detects. Forwarded from the rulesets endpoints; ignored by this operation.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// A URL pointing to the rule documentation. Forwarded from the rulesets endpoints; ignored by this operation.
    #[serde(rename = "documentation_url")]
    pub documentation_url: Option<String>,
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
    /// Whether the rule is published. Forwarded from the rulesets endpoints; ignored by this operation.
    #[serde(rename = "is_published")]
    pub is_published: Option<bool>,
    /// Whether the rule is in testing mode. Forwarded from the rulesets endpoints; ignored by this operation.
    #[serde(rename = "is_testing")]
    pub is_testing: Option<bool>,
    /// The programming language this rule targets.
    #[serde(rename = "language")]
    pub language: String,
    /// The date and time when the rule was last modified. Server-assigned by the rulesets endpoints; ignored by this operation.
    #[serde(rename = "last_updated_at")]
    pub last_updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The identifier of the user or system that last updated the rule. Server-assigned by the rulesets endpoints; ignored by this operation.
    #[serde(rename = "last_updated_by")]
    pub last_updated_by: Option<String>,
    /// The name of the rule. Forwarded from the rulesets endpoints; ignored by this operation.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// A base64-encoded regex pattern used by the rule, applicable when rule type is `REGEX`.
    #[serde(rename = "regex", default, with = "::serde_with::rust::double_option")]
    pub regex: Option<Option<String>>,
    /// The severity of findings from this rule (for example, `ERROR`, `WARNING`).
    #[serde(rename = "severity")]
    pub severity: String,
    /// A brief summary of what the rule detects. Forwarded from the rulesets endpoints; ignored by this operation.
    #[serde(rename = "short_description")]
    pub short_description: Option<String>,
    /// Whether an AI-generated fix should be offered. Forwarded from the rulesets endpoints; ignored by this operation.
    #[serde(rename = "should_use_ai_fix")]
    pub should_use_ai_fix: Option<bool>,
    /// The test cases associated with the rule. Forwarded from the rulesets endpoints; ignored by this operation.
    #[serde(rename = "tests")]
    pub tests: Option<Vec<crate::datadogV2::model::AnalysisRequestRuleTest>>,
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
            arguments: None,
            category,
            checksum,
            code,
            created_at: None,
            created_by: None,
            cve: None,
            cwe: None,
            description: None,
            documentation_url: None,
            entity_checked: None,
            id,
            is_published: None,
            is_testing: None,
            language,
            last_updated_at: None,
            last_updated_by: None,
            name: None,
            regex: None,
            severity,
            short_description: None,
            should_use_ai_fix: None,
            tests: None,
            tree_sitter_query,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn arguments(
        mut self,
        value: Vec<crate::datadogV2::model::AnalysisRequestRuleArgument>,
    ) -> Self {
        self.arguments = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: String) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn cve(mut self, value: String) -> Self {
        self.cve = Some(value);
        self
    }

    pub fn cwe(mut self, value: String) -> Self {
        self.cwe = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn documentation_url(mut self, value: String) -> Self {
        self.documentation_url = Some(value);
        self
    }

    pub fn entity_checked(mut self, value: Option<String>) -> Self {
        self.entity_checked = Some(value);
        self
    }

    pub fn is_published(mut self, value: bool) -> Self {
        self.is_published = Some(value);
        self
    }

    pub fn is_testing(mut self, value: bool) -> Self {
        self.is_testing = Some(value);
        self
    }

    pub fn last_updated_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.last_updated_at = Some(value);
        self
    }

    pub fn last_updated_by(mut self, value: String) -> Self {
        self.last_updated_by = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn regex(mut self, value: Option<String>) -> Self {
        self.regex = Some(value);
        self
    }

    pub fn short_description(mut self, value: String) -> Self {
        self.short_description = Some(value);
        self
    }

    pub fn should_use_ai_fix(mut self, value: bool) -> Self {
        self.should_use_ai_fix = Some(value);
        self
    }

    pub fn tests(mut self, value: Vec<crate::datadogV2::model::AnalysisRequestRuleTest>) -> Self {
        self.tests = Some(value);
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
                let mut arguments: Option<
                    Vec<crate::datadogV2::model::AnalysisRequestRuleArgument>,
                > = None;
                let mut category: Option<String> = None;
                let mut checksum: Option<String> = None;
                let mut code: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut cve: Option<String> = None;
                let mut cwe: Option<String> = None;
                let mut description: Option<String> = None;
                let mut documentation_url: Option<String> = None;
                let mut entity_checked: Option<Option<String>> = None;
                let mut id: Option<String> = None;
                let mut is_published: Option<bool> = None;
                let mut is_testing: Option<bool> = None;
                let mut language: Option<String> = None;
                let mut last_updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut last_updated_by: Option<String> = None;
                let mut name: Option<String> = None;
                let mut regex: Option<Option<String>> = None;
                let mut severity: Option<String> = None;
                let mut short_description: Option<String> = None;
                let mut should_use_ai_fix: Option<bool> = None;
                let mut tests: Option<Vec<crate::datadogV2::model::AnalysisRequestRuleTest>> = None;
                let mut tree_sitter_query: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "arguments" => {
                            if v.is_null() {
                                continue;
                            }
                            arguments = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "category" => {
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "checksum" => {
                            checksum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "code" => {
                            code = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cve" => {
                            if v.is_null() {
                                continue;
                            }
                            cve = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cwe" => {
                            if v.is_null() {
                                continue;
                            }
                            cwe = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "documentation_url" => {
                            if v.is_null() {
                                continue;
                            }
                            documentation_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entity_checked" => {
                            entity_checked =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_published" => {
                            if v.is_null() {
                                continue;
                            }
                            is_published =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_testing" => {
                            if v.is_null() {
                                continue;
                            }
                            is_testing = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "language" => {
                            language = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            last_updated_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_updated_by" => {
                            if v.is_null() {
                                continue;
                            }
                            last_updated_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "regex" => {
                            regex = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severity" => {
                            severity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "short_description" => {
                            if v.is_null() {
                                continue;
                            }
                            short_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "should_use_ai_fix" => {
                            if v.is_null() {
                                continue;
                            }
                            should_use_ai_fix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tests" => {
                            if v.is_null() {
                                continue;
                            }
                            tests = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                    arguments,
                    category,
                    checksum,
                    code,
                    created_at,
                    created_by,
                    cve,
                    cwe,
                    description,
                    documentation_url,
                    entity_checked,
                    id,
                    is_published,
                    is_testing,
                    language,
                    last_updated_at,
                    last_updated_by,
                    name,
                    regex,
                    severity,
                    short_description,
                    should_use_ai_fix,
                    tests,
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

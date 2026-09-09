// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A static analysis rule within a ruleset, including its definition, metadata, and associated test cases.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItems {
    /// The list of configurable arguments accepted by this rule.
    #[serde(rename = "arguments")]
    pub arguments: Vec<crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsArgumentsItems>,
    /// The category classifying the type of issue this rule detects (e.g., security, style, performance).
    #[serde(rename = "category")]
    pub category: String,
    /// A checksum of the rule definition used to detect changes.
    #[serde(rename = "checksum")]
    pub checksum: String,
    /// The rule implementation code used by the static analysis engine.
    #[serde(rename = "code")]
    pub code: String,
    /// The date and time when the rule was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The identifier of the user or system that created the rule.
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// The CVE identifier associated with the vulnerability this rule detects, if applicable.
    #[serde(rename = "cve")]
    pub cve: Option<String>,
    /// The CWE identifier associated with the weakness category this rule detects, if applicable.
    #[serde(rename = "cwe")]
    pub cwe: Option<String>,
    /// A detailed explanation of what the rule detects and why it matters.
    #[serde(rename = "description")]
    pub description: String,
    /// A URL pointing to additional documentation for this rule.
    #[serde(rename = "documentation_url")]
    pub documentation_url: Option<String>,
    /// The code entity type (e.g., function, class, variable) that this rule inspects.
    #[serde(rename = "entity_checked")]
    pub entity_checked: Option<String>,
    /// The unique identifier of the rule, which is the same as its name.
    #[serde(rename = "id")]
    pub id: String,
    /// Indicates whether the rule is publicly published and available to all users.
    #[serde(rename = "is_published")]
    pub is_published: bool,
    /// Indicates whether the rule is in testing mode and not yet promoted to production.
    #[serde(rename = "is_testing")]
    pub is_testing: bool,
    /// The programming language this rule applies to.
    #[serde(rename = "language")]
    pub language: String,
    /// The date and time when the rule was last modified.
    #[serde(rename = "last_updated_at")]
    pub last_updated_at: chrono::DateTime<chrono::Utc>,
    /// The identifier of the user or system that last updated the rule.
    #[serde(rename = "last_updated_by")]
    pub last_updated_by: String,
    /// The unique name identifying this rule within its ruleset.
    #[serde(rename = "name")]
    pub name: String,
    /// A regular expression pattern used by the rule for pattern-based detection.
    #[serde(rename = "regex")]
    pub regex: Option<String>,
    /// The severity level of findings produced by this rule (e.g., ERROR, WARNING, NOTICE).
    #[serde(rename = "severity")]
    pub severity: String,
    /// A brief summary of what the rule detects, suitable for display in listings.
    #[serde(rename = "short_description")]
    pub short_description: String,
    /// Indicates whether an AI-generated fix suggestion should be offered for findings from this rule.
    #[serde(rename = "should_use_ai_fix")]
    pub should_use_ai_fix: bool,
    /// The list of test cases used to validate the rule's behavior.
    #[serde(rename = "tests")]
    pub tests: Vec<crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsTestsItems>,
    /// The Tree-sitter query expression used by the rule to match code patterns in the AST.
    #[serde(rename = "tree_sitter_query")]
    pub tree_sitter_query: Option<String>,
    /// The rule type indicating the detection mechanism used (e.g., tree_sitter, regex).
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItems {
    pub fn new(
        arguments: Vec<crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsArgumentsItems>,
        category: String,
        checksum: String,
        code: String,
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: String,
        description: String,
        id: String,
        is_published: bool,
        is_testing: bool,
        language: String,
        last_updated_at: chrono::DateTime<chrono::Utc>,
        last_updated_by: String,
        name: String,
        severity: String,
        short_description: String,
        should_use_ai_fix: bool,
        tests: Vec<crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsTestsItems>,
        type_: String,
    ) -> GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItems {
        GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItems {
            arguments,
            category,
            checksum,
            code,
            created_at,
            created_by,
            cve: None,
            cwe: None,
            description,
            documentation_url: None,
            entity_checked: None,
            id,
            is_published,
            is_testing,
            language,
            last_updated_at,
            last_updated_by,
            name,
            regex: None,
            severity,
            short_description,
            should_use_ai_fix,
            tests,
            tree_sitter_query: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cve(mut self, value: String) -> Self {
        self.cve = Some(value);
        self
    }

    pub fn cwe(mut self, value: String) -> Self {
        self.cwe = Some(value);
        self
    }

    pub fn documentation_url(mut self, value: String) -> Self {
        self.documentation_url = Some(value);
        self
    }

    pub fn entity_checked(mut self, value: String) -> Self {
        self.entity_checked = Some(value);
        self
    }

    pub fn regex(mut self, value: String) -> Self {
        self.regex = Some(value);
        self
    }

    pub fn tree_sitter_query(mut self, value: String) -> Self {
        self.tree_sitter_query = Some(value);
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

impl<'de> Deserialize<'de> for GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsVisitor;
        impl<'a> Visitor<'a> for GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsVisitor {
            type Value = GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut arguments: Option<Vec<crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsArgumentsItems>> = None;
                let mut category: Option<String> = None;
                let mut checksum: Option<String> = None;
                let mut code: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut cve: Option<String> = None;
                let mut cwe: Option<String> = None;
                let mut description: Option<String> = None;
                let mut documentation_url: Option<String> = None;
                let mut entity_checked: Option<String> = None;
                let mut id: Option<String> = None;
                let mut is_published: Option<bool> = None;
                let mut is_testing: Option<bool> = None;
                let mut language: Option<String> = None;
                let mut last_updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut last_updated_by: Option<String> = None;
                let mut name: Option<String> = None;
                let mut regex: Option<String> = None;
                let mut severity: Option<String> = None;
                let mut short_description: Option<String> = None;
                let mut should_use_ai_fix: Option<bool> = None;
                let mut tests: Option<Vec<crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsTestsItems>> = None;
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
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
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
                            if v.is_null() {
                                continue;
                            }
                            entity_checked =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_published" => {
                            is_published =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_testing" => {
                            is_testing = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "language" => {
                            language = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_updated_at" => {
                            last_updated_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_updated_by" => {
                            last_updated_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "regex" => {
                            if v.is_null() {
                                continue;
                            }
                            regex = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severity" => {
                            severity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "short_description" => {
                            short_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "should_use_ai_fix" => {
                            should_use_ai_fix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tests" => {
                            tests = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tree_sitter_query" => {
                            if v.is_null() {
                                continue;
                            }
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
                let arguments = arguments.ok_or_else(|| M::Error::missing_field("arguments"))?;
                let category = category.ok_or_else(|| M::Error::missing_field("category"))?;
                let checksum = checksum.ok_or_else(|| M::Error::missing_field("checksum"))?;
                let code = code.ok_or_else(|| M::Error::missing_field("code"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let is_published =
                    is_published.ok_or_else(|| M::Error::missing_field("is_published"))?;
                let is_testing = is_testing.ok_or_else(|| M::Error::missing_field("is_testing"))?;
                let language = language.ok_or_else(|| M::Error::missing_field("language"))?;
                let last_updated_at =
                    last_updated_at.ok_or_else(|| M::Error::missing_field("last_updated_at"))?;
                let last_updated_by =
                    last_updated_by.ok_or_else(|| M::Error::missing_field("last_updated_by"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let severity = severity.ok_or_else(|| M::Error::missing_field("severity"))?;
                let short_description = short_description
                    .ok_or_else(|| M::Error::missing_field("short_description"))?;
                let should_use_ai_fix = should_use_ai_fix
                    .ok_or_else(|| M::Error::missing_field("should_use_ai_fix"))?;
                let tests = tests.ok_or_else(|| M::Error::missing_field("tests"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItems {
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

        deserializer.deserialize_any(
            GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsVisitor,
        )
    }
}

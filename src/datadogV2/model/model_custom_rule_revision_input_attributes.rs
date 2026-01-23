// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomRuleRevisionInputAttributes {
    /// Rule arguments
    #[serde(rename = "arguments")]
    pub arguments: Vec<crate::datadogV2::model::Argument>,
    /// Rule category
    #[serde(rename = "category")]
    pub category: crate::datadogV2::model::CustomRuleRevisionAttributesCategory,
    /// Rule code
    #[serde(rename = "code")]
    pub code: String,
    /// Revision creation message
    #[serde(rename = "creation_message")]
    pub creation_message: String,
    /// Associated CVE
    #[serialize_always]
    #[serde(rename = "cve")]
    pub cve: Option<String>,
    /// Associated CWE
    #[serialize_always]
    #[serde(rename = "cwe")]
    pub cwe: Option<String>,
    /// Full description
    #[serde(rename = "description")]
    pub description: String,
    /// Documentation URL
    #[serialize_always]
    #[serde(rename = "documentation_url")]
    pub documentation_url: Option<String>,
    /// Whether the revision is published
    #[serde(rename = "is_published")]
    pub is_published: bool,
    /// Whether this is a testing revision
    #[serde(rename = "is_testing")]
    pub is_testing: bool,
    /// Programming language
    #[serde(rename = "language")]
    pub language: crate::datadogV2::model::Language,
    /// Rule severity
    #[serde(rename = "severity")]
    pub severity: crate::datadogV2::model::CustomRuleRevisionAttributesSeverity,
    /// Short description
    #[serde(rename = "short_description")]
    pub short_description: String,
    /// Whether to use AI for fixes
    #[serde(rename = "should_use_ai_fix")]
    pub should_use_ai_fix: bool,
    /// Rule tags
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// Rule tests
    #[serde(rename = "tests")]
    pub tests: Vec<crate::datadogV2::model::CustomRuleRevisionTest>,
    /// Tree-sitter query
    #[serde(rename = "tree_sitter_query")]
    pub tree_sitter_query: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomRuleRevisionInputAttributes {
    pub fn new(
        arguments: Vec<crate::datadogV2::model::Argument>,
        category: crate::datadogV2::model::CustomRuleRevisionAttributesCategory,
        code: String,
        creation_message: String,
        cve: Option<String>,
        cwe: Option<String>,
        description: String,
        documentation_url: Option<String>,
        is_published: bool,
        is_testing: bool,
        language: crate::datadogV2::model::Language,
        severity: crate::datadogV2::model::CustomRuleRevisionAttributesSeverity,
        short_description: String,
        should_use_ai_fix: bool,
        tags: Vec<String>,
        tests: Vec<crate::datadogV2::model::CustomRuleRevisionTest>,
        tree_sitter_query: String,
    ) -> CustomRuleRevisionInputAttributes {
        CustomRuleRevisionInputAttributes {
            arguments,
            category,
            code,
            creation_message,
            cve,
            cwe,
            description,
            documentation_url,
            is_published,
            is_testing,
            language,
            severity,
            short_description,
            should_use_ai_fix,
            tags,
            tests,
            tree_sitter_query,
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

impl<'de> Deserialize<'de> for CustomRuleRevisionInputAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomRuleRevisionInputAttributesVisitor;
        impl<'a> Visitor<'a> for CustomRuleRevisionInputAttributesVisitor {
            type Value = CustomRuleRevisionInputAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut arguments: Option<Vec<crate::datadogV2::model::Argument>> = None;
                let mut category: Option<
                    crate::datadogV2::model::CustomRuleRevisionAttributesCategory,
                > = None;
                let mut code: Option<String> = None;
                let mut creation_message: Option<String> = None;
                let mut cve: Option<Option<String>> = None;
                let mut cwe: Option<Option<String>> = None;
                let mut description: Option<String> = None;
                let mut documentation_url: Option<Option<String>> = None;
                let mut is_published: Option<bool> = None;
                let mut is_testing: Option<bool> = None;
                let mut language: Option<crate::datadogV2::model::Language> = None;
                let mut severity: Option<
                    crate::datadogV2::model::CustomRuleRevisionAttributesSeverity,
                > = None;
                let mut short_description: Option<String> = None;
                let mut should_use_ai_fix: Option<bool> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut tests: Option<Vec<crate::datadogV2::model::CustomRuleRevisionTest>> = None;
                let mut tree_sitter_query: Option<String> = None;
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
                            if let Some(ref _category) = category {
                                match _category {
                                    crate::datadogV2::model::CustomRuleRevisionAttributesCategory::UnparsedObject(_category) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "code" => {
                            code = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creation_message" => {
                            creation_message =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cve" => {
                            cve = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cwe" => {
                            cwe = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "documentation_url" => {
                            documentation_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                            if let Some(ref _language) = language {
                                match _language {
                                    crate::datadogV2::model::Language::UnparsedObject(
                                        _language,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "severity" => {
                            severity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _severity) = severity {
                                match _severity {
                                    crate::datadogV2::model::CustomRuleRevisionAttributesSeverity::UnparsedObject(_severity) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "short_description" => {
                            short_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "should_use_ai_fix" => {
                            should_use_ai_fix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tests" => {
                            tests = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tree_sitter_query" => {
                            tree_sitter_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let code = code.ok_or_else(|| M::Error::missing_field("code"))?;
                let creation_message =
                    creation_message.ok_or_else(|| M::Error::missing_field("creation_message"))?;
                let cve = cve.ok_or_else(|| M::Error::missing_field("cve"))?;
                let cwe = cwe.ok_or_else(|| M::Error::missing_field("cwe"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let documentation_url = documentation_url
                    .ok_or_else(|| M::Error::missing_field("documentation_url"))?;
                let is_published =
                    is_published.ok_or_else(|| M::Error::missing_field("is_published"))?;
                let is_testing = is_testing.ok_or_else(|| M::Error::missing_field("is_testing"))?;
                let language = language.ok_or_else(|| M::Error::missing_field("language"))?;
                let severity = severity.ok_or_else(|| M::Error::missing_field("severity"))?;
                let short_description = short_description
                    .ok_or_else(|| M::Error::missing_field("short_description"))?;
                let should_use_ai_fix = should_use_ai_fix
                    .ok_or_else(|| M::Error::missing_field("should_use_ai_fix"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;
                let tests = tests.ok_or_else(|| M::Error::missing_field("tests"))?;
                let tree_sitter_query = tree_sitter_query
                    .ok_or_else(|| M::Error::missing_field("tree_sitter_query"))?;

                let content = CustomRuleRevisionInputAttributes {
                    arguments,
                    category,
                    code,
                    creation_message,
                    cve,
                    cwe,
                    description,
                    documentation_url,
                    is_published,
                    is_testing,
                    language,
                    severity,
                    short_description,
                    should_use_ai_fix,
                    tags,
                    tests,
                    tree_sitter_query,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomRuleRevisionInputAttributesVisitor)
    }
}

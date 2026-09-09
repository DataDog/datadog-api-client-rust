// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Input attributes for creating or updating a custom rule revision.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomRuleRevisionInputAttributes {
    /// Rule arguments
    #[serialize_always]
    #[serde(rename = "arguments")]
    pub arguments: Option<Vec<crate::datadogV2::model::Argument>>,
    /// Rule category
    #[serde(rename = "category")]
    pub category: crate::datadogV2::model::CustomRuleRevisionAttributesCategory,
    /// Code checksum. Derived by the API from `code`; ignored on write.
    #[serde(rename = "checksum")]
    pub checksum: Option<String>,
    /// Rule code
    #[serde(rename = "code")]
    pub code: String,
    /// Creation timestamp. Set by the API; ignored on write.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Creator identifier. Set by the API from the caller; ignored on write.
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// Revision creation message
    #[serde(rename = "creation_message")]
    pub creation_message: String,
    /// Associated CVE
    #[serde(rename = "cve", default, with = "::serde_with::rust::double_option")]
    pub cve: Option<Option<String>>,
    /// Associated CWE
    #[serde(rename = "cwe", default, with = "::serde_with::rust::double_option")]
    pub cwe: Option<Option<String>>,
    /// Full description
    #[serde(rename = "description")]
    pub description: String,
    /// Documentation URL
    #[serde(
        rename = "documentation_url",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub documentation_url: Option<Option<String>>,
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
    #[serialize_always]
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Rule tests
    #[serialize_always]
    #[serde(rename = "tests")]
    pub tests: Option<Vec<crate::datadogV2::model::CustomRuleRevisionTest>>,
    /// Tree-sitter query
    #[serde(rename = "tree_sitter_query")]
    pub tree_sitter_query: String,
    /// Monotonically increasing version number of the revision. Assigned by the API; ignored on write.
    #[serde(rename = "version_id")]
    pub version_id: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomRuleRevisionInputAttributes {
    pub fn new(
        arguments: Option<Vec<crate::datadogV2::model::Argument>>,
        category: crate::datadogV2::model::CustomRuleRevisionAttributesCategory,
        code: String,
        creation_message: String,
        description: String,
        is_published: bool,
        is_testing: bool,
        language: crate::datadogV2::model::Language,
        severity: crate::datadogV2::model::CustomRuleRevisionAttributesSeverity,
        short_description: String,
        should_use_ai_fix: bool,
        tags: Option<Vec<String>>,
        tests: Option<Vec<crate::datadogV2::model::CustomRuleRevisionTest>>,
        tree_sitter_query: String,
    ) -> CustomRuleRevisionInputAttributes {
        CustomRuleRevisionInputAttributes {
            arguments,
            category,
            checksum: None,
            code,
            created_at: None,
            created_by: None,
            creation_message,
            cve: None,
            cwe: None,
            description,
            documentation_url: None,
            is_published,
            is_testing,
            language,
            severity,
            short_description,
            should_use_ai_fix,
            tags,
            tests,
            tree_sitter_query,
            version_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn checksum(mut self, value: String) -> Self {
        self.checksum = Some(value);
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

    pub fn cve(mut self, value: Option<String>) -> Self {
        self.cve = Some(value);
        self
    }

    pub fn cwe(mut self, value: Option<String>) -> Self {
        self.cwe = Some(value);
        self
    }

    pub fn documentation_url(mut self, value: Option<String>) -> Self {
        self.documentation_url = Some(value);
        self
    }

    pub fn version_id(mut self, value: i64) -> Self {
        self.version_id = Some(value);
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
                let mut arguments: Option<Option<Vec<crate::datadogV2::model::Argument>>> = None;
                let mut category: Option<
                    crate::datadogV2::model::CustomRuleRevisionAttributesCategory,
                > = None;
                let mut checksum: Option<String> = None;
                let mut code: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
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
                let mut tags: Option<Option<Vec<String>>> = None;
                let mut tests: Option<
                    Option<Vec<crate::datadogV2::model::CustomRuleRevisionTest>>,
                > = None;
                let mut tree_sitter_query: Option<String> = None;
                let mut version_id: Option<i64> = None;
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
                        "checksum" => {
                            if v.is_null() {
                                continue;
                            }
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
                        "version_id" => {
                            if v.is_null() {
                                continue;
                            }
                            version_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
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
                    checksum,
                    code,
                    created_at,
                    created_by,
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
                    version_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomRuleRevisionInputAttributesVisitor)
    }
}

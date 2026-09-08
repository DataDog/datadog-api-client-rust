// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A revision of a custom static analysis rule as embedded in a rule supplied by a create
/// or update request. Nested revisions are sent flat, without a `data`/`type`/`attributes`
/// envelope. `id`, `version_id`, `checksum`, `created_at` and `created_by` are server-assigned
/// and read-only; they are declared so that a ruleset previously read back can be supplied
/// unchanged.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomRuleRevisionInput {
    /// Rule arguments
    #[serde(
        rename = "arguments",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub arguments: Option<Option<Vec<crate::datadogV2::model::Argument>>>,
    /// Rule category
    #[serde(rename = "category")]
    pub category: Option<crate::datadogV2::model::CustomRuleRevisionAttributesCategory>,
    /// Code checksum
    #[serde(rename = "checksum")]
    pub checksum: Option<String>,
    /// Rule code
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// Creation timestamp
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Creator identifier
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// Revision creation message
    #[serde(rename = "creation_message")]
    pub creation_message: Option<String>,
    /// Associated CVE
    #[serde(rename = "cve", default, with = "::serde_with::rust::double_option")]
    pub cve: Option<Option<String>>,
    /// Associated CWE
    #[serde(rename = "cwe", default, with = "::serde_with::rust::double_option")]
    pub cwe: Option<Option<String>>,
    /// Base64-encoded full description
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Documentation URL
    #[serde(
        rename = "documentation_url",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub documentation_url: Option<Option<String>>,
    /// Revision identifier
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Whether the revision should be published
    #[serde(rename = "is_published")]
    pub is_published: Option<bool>,
    /// Whether this is a testing revision
    #[serde(rename = "is_testing")]
    pub is_testing: Option<bool>,
    /// Programming language
    #[serde(rename = "language")]
    pub language: Option<crate::datadogV2::model::Language>,
    /// Rule severity
    #[serde(rename = "severity")]
    pub severity: Option<crate::datadogV2::model::CustomRuleRevisionAttributesSeverity>,
    /// Base64-encoded short description
    #[serde(rename = "short_description")]
    pub short_description: Option<String>,
    /// Whether to use AI for fixes
    #[serde(rename = "should_use_ai_fix")]
    pub should_use_ai_fix: Option<bool>,
    /// Rule tags
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<Vec<String>>>,
    /// Rule tests
    #[serde(rename = "tests", default, with = "::serde_with::rust::double_option")]
    pub tests: Option<Option<Vec<crate::datadogV2::model::CustomRuleRevisionTest>>>,
    /// Tree-sitter query
    #[serde(rename = "tree_sitter_query")]
    pub tree_sitter_query: Option<String>,
    /// Monotonically increasing version number of the revision.
    #[serde(rename = "version_id")]
    pub version_id: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomRuleRevisionInput {
    pub fn new() -> CustomRuleRevisionInput {
        CustomRuleRevisionInput {
            arguments: None,
            category: None,
            checksum: None,
            code: None,
            created_at: None,
            created_by: None,
            creation_message: None,
            cve: None,
            cwe: None,
            description: None,
            documentation_url: None,
            id: None,
            is_published: None,
            is_testing: None,
            language: None,
            severity: None,
            short_description: None,
            should_use_ai_fix: None,
            tags: None,
            tests: None,
            tree_sitter_query: None,
            version_id: None,
            _unparsed: false,
        }
    }

    pub fn arguments(mut self, value: Option<Vec<crate::datadogV2::model::Argument>>) -> Self {
        self.arguments = Some(value);
        self
    }

    pub fn category(
        mut self,
        value: crate::datadogV2::model::CustomRuleRevisionAttributesCategory,
    ) -> Self {
        self.category = Some(value);
        self
    }

    pub fn checksum(mut self, value: String) -> Self {
        self.checksum = Some(value);
        self
    }

    pub fn code(mut self, value: String) -> Self {
        self.code = Some(value);
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

    pub fn creation_message(mut self, value: String) -> Self {
        self.creation_message = Some(value);
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

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn documentation_url(mut self, value: Option<String>) -> Self {
        self.documentation_url = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
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

    pub fn language(mut self, value: crate::datadogV2::model::Language) -> Self {
        self.language = Some(value);
        self
    }

    pub fn severity(
        mut self,
        value: crate::datadogV2::model::CustomRuleRevisionAttributesSeverity,
    ) -> Self {
        self.severity = Some(value);
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

    pub fn tags(mut self, value: Option<Vec<String>>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn tests(
        mut self,
        value: Option<Vec<crate::datadogV2::model::CustomRuleRevisionTest>>,
    ) -> Self {
        self.tests = Some(value);
        self
    }

    pub fn tree_sitter_query(mut self, value: String) -> Self {
        self.tree_sitter_query = Some(value);
        self
    }

    pub fn version_id(mut self, value: i64) -> Self {
        self.version_id = Some(value);
        self
    }
}

impl Default for CustomRuleRevisionInput {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CustomRuleRevisionInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomRuleRevisionInputVisitor;
        impl<'a> Visitor<'a> for CustomRuleRevisionInputVisitor {
            type Value = CustomRuleRevisionInput;

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
                let mut id: Option<String> = None;
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
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "arguments" => {
                            arguments = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "category" => {
                            if v.is_null() {
                                continue;
                            }
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
                            if v.is_null() {
                                continue;
                            }
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
                            if v.is_null() {
                                continue;
                            }
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
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "documentation_url" => {
                            documentation_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
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
                            if v.is_null() {
                                continue;
                            }
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
                            if v.is_null() {
                                continue;
                            }
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
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "version_id" => {
                            if v.is_null() {
                                continue;
                            }
                            version_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = CustomRuleRevisionInput {
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
                    id,
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
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomRuleRevisionInputVisitor)
    }
}

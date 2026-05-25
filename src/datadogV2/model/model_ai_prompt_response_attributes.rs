// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response attributes of an AI prompt.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiPromptResponseAttributes {
    /// Rule category
    #[serde(rename = "category")]
    pub category: crate::datadogV2::model::CustomRuleRevisionAttributesCategory,
    /// Checksum of the prompt content.
    #[serde(rename = "checksum")]
    pub checksum: String,
    /// Base64-encoded AI prompt content.
    #[serde(rename = "content")]
    pub content: String,
    /// The CWE identifier associated with this prompt.
    #[serde(rename = "cwe")]
    pub cwe: Option<String>,
    /// Base64-encoded full description.
    #[serde(rename = "description")]
    pub description: String,
    /// Directory patterns this prompt applies to.
    #[serde(rename = "directories")]
    pub directories: Vec<String>,
    /// The execution mode for an AI rule revision.
    #[serde(rename = "execution_mode")]
    pub execution_mode: crate::datadogV2::model::AiCustomRuleRevisionExecutionMode,
    /// Keywords used to search for relevant files.
    #[serde(rename = "file_search_keywords")]
    pub file_search_keywords: Vec<String>,
    /// File glob patterns this prompt applies to.
    #[serde(rename = "globs")]
    pub globs: Vec<String>,
    /// Whether this is a default Datadog prompt.
    #[serde(rename = "is_default")]
    pub is_default: bool,
    /// Whether this prompt is for testing only.
    #[serde(rename = "is_testing")]
    pub is_testing: bool,
    /// Programming language
    #[serde(rename = "language")]
    pub language: Option<crate::datadogV2::model::Language>,
    /// Keywords to exclude from results.
    #[serde(rename = "result_keywords_exclude")]
    pub result_keywords_exclude: Vec<String>,
    /// The version of the rule this prompt is associated with.
    #[serde(rename = "rule_version")]
    pub rule_version: String,
    /// Rule severity
    #[serde(rename = "severity")]
    pub severity: crate::datadogV2::model::CustomRuleRevisionAttributesSeverity,
    /// Base64-encoded short description.
    #[serde(rename = "short_description")]
    pub short_description: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiPromptResponseAttributes {
    pub fn new(
        category: crate::datadogV2::model::CustomRuleRevisionAttributesCategory,
        checksum: String,
        content: String,
        description: String,
        directories: Vec<String>,
        execution_mode: crate::datadogV2::model::AiCustomRuleRevisionExecutionMode,
        file_search_keywords: Vec<String>,
        globs: Vec<String>,
        is_default: bool,
        is_testing: bool,
        result_keywords_exclude: Vec<String>,
        rule_version: String,
        severity: crate::datadogV2::model::CustomRuleRevisionAttributesSeverity,
        short_description: String,
    ) -> AiPromptResponseAttributes {
        AiPromptResponseAttributes {
            category,
            checksum,
            content,
            cwe: None,
            description,
            directories,
            execution_mode,
            file_search_keywords,
            globs,
            is_default,
            is_testing,
            language: None,
            result_keywords_exclude,
            rule_version,
            severity,
            short_description,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cwe(mut self, value: String) -> Self {
        self.cwe = Some(value);
        self
    }

    pub fn language(mut self, value: crate::datadogV2::model::Language) -> Self {
        self.language = Some(value);
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

impl<'de> Deserialize<'de> for AiPromptResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiPromptResponseAttributesVisitor;
        impl<'a> Visitor<'a> for AiPromptResponseAttributesVisitor {
            type Value = AiPromptResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<
                    crate::datadogV2::model::CustomRuleRevisionAttributesCategory,
                > = None;
                let mut checksum: Option<String> = None;
                let mut content: Option<String> = None;
                let mut cwe: Option<String> = None;
                let mut description: Option<String> = None;
                let mut directories: Option<Vec<String>> = None;
                let mut execution_mode: Option<
                    crate::datadogV2::model::AiCustomRuleRevisionExecutionMode,
                > = None;
                let mut file_search_keywords: Option<Vec<String>> = None;
                let mut globs: Option<Vec<String>> = None;
                let mut is_default: Option<bool> = None;
                let mut is_testing: Option<bool> = None;
                let mut language: Option<crate::datadogV2::model::Language> = None;
                let mut result_keywords_exclude: Option<Vec<String>> = None;
                let mut rule_version: Option<String> = None;
                let mut severity: Option<
                    crate::datadogV2::model::CustomRuleRevisionAttributesSeverity,
                > = None;
                let mut short_description: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                            checksum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "content" => {
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "directories" => {
                            directories =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "execution_mode" => {
                            execution_mode =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _execution_mode) = execution_mode {
                                match _execution_mode {
                                    crate::datadogV2::model::AiCustomRuleRevisionExecutionMode::UnparsedObject(_execution_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "file_search_keywords" => {
                            file_search_keywords =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "globs" => {
                            globs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_default" => {
                            is_default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_testing" => {
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
                        "result_keywords_exclude" => {
                            result_keywords_exclude =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_version" => {
                            rule_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let category = category.ok_or_else(|| M::Error::missing_field("category"))?;
                let checksum = checksum.ok_or_else(|| M::Error::missing_field("checksum"))?;
                let content = content.ok_or_else(|| M::Error::missing_field("content"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let directories =
                    directories.ok_or_else(|| M::Error::missing_field("directories"))?;
                let execution_mode =
                    execution_mode.ok_or_else(|| M::Error::missing_field("execution_mode"))?;
                let file_search_keywords = file_search_keywords
                    .ok_or_else(|| M::Error::missing_field("file_search_keywords"))?;
                let globs = globs.ok_or_else(|| M::Error::missing_field("globs"))?;
                let is_default = is_default.ok_or_else(|| M::Error::missing_field("is_default"))?;
                let is_testing = is_testing.ok_or_else(|| M::Error::missing_field("is_testing"))?;
                let result_keywords_exclude = result_keywords_exclude
                    .ok_or_else(|| M::Error::missing_field("result_keywords_exclude"))?;
                let rule_version =
                    rule_version.ok_or_else(|| M::Error::missing_field("rule_version"))?;
                let severity = severity.ok_or_else(|| M::Error::missing_field("severity"))?;
                let short_description = short_description
                    .ok_or_else(|| M::Error::missing_field("short_description"))?;

                let content = AiPromptResponseAttributes {
                    category,
                    checksum,
                    content,
                    cwe,
                    description,
                    directories,
                    execution_mode,
                    file_search_keywords,
                    globs,
                    is_default,
                    is_testing,
                    language,
                    result_keywords_exclude,
                    rule_version,
                    severity,
                    short_description,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiPromptResponseAttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating an AI custom rule revision.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiCustomRuleRevisionRequestAttributes {
    /// Rule category
    #[serde(rename = "category")]
    pub category: crate::datadogV2::model::CustomRuleRevisionAttributesCategory,
    /// Base64-encoded AI model content for this revision.
    #[serde(rename = "content")]
    pub content: String,
    /// The associated CWE identifier.
    #[serde(rename = "cwe", default, with = "::serde_with::rust::double_option")]
    pub cwe: Option<Option<String>>,
    /// Base64-encoded full description.
    #[serde(rename = "description")]
    pub description: String,
    /// Directory patterns this rule applies to.
    #[serde(rename = "directories")]
    pub directories: Vec<String>,
    /// The execution mode for an AI rule revision.
    #[serde(rename = "execution_mode")]
    pub execution_mode: crate::datadogV2::model::AiCustomRuleRevisionExecutionMode,
    /// File glob patterns this rule applies to.
    #[serde(rename = "globs")]
    pub globs: Vec<String>,
    /// Whether this revision is published.
    #[serde(rename = "is_published")]
    pub is_published: bool,
    /// Whether this revision is for testing only.
    #[serde(rename = "is_testing")]
    pub is_testing: bool,
    /// Rule severity
    #[serde(rename = "severity")]
    pub severity: crate::datadogV2::model::CustomRuleRevisionAttributesSeverity,
    /// Base64-encoded short description.
    #[serde(rename = "short_description")]
    pub short_description: String,
    /// The version identifier for this revision.
    #[serde(rename = "version_id")]
    pub version_id: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiCustomRuleRevisionRequestAttributes {
    pub fn new(
        category: crate::datadogV2::model::CustomRuleRevisionAttributesCategory,
        content: String,
        description: String,
        directories: Vec<String>,
        execution_mode: crate::datadogV2::model::AiCustomRuleRevisionExecutionMode,
        globs: Vec<String>,
        is_published: bool,
        is_testing: bool,
        severity: crate::datadogV2::model::CustomRuleRevisionAttributesSeverity,
        short_description: String,
    ) -> AiCustomRuleRevisionRequestAttributes {
        AiCustomRuleRevisionRequestAttributes {
            category,
            content,
            cwe: None,
            description,
            directories,
            execution_mode,
            globs,
            is_published,
            is_testing,
            severity,
            short_description,
            version_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cwe(mut self, value: Option<String>) -> Self {
        self.cwe = Some(value);
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

impl<'de> Deserialize<'de> for AiCustomRuleRevisionRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiCustomRuleRevisionRequestAttributesVisitor;
        impl<'a> Visitor<'a> for AiCustomRuleRevisionRequestAttributesVisitor {
            type Value = AiCustomRuleRevisionRequestAttributes;

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
                let mut content: Option<String> = None;
                let mut cwe: Option<Option<String>> = None;
                let mut description: Option<String> = None;
                let mut directories: Option<Vec<String>> = None;
                let mut execution_mode: Option<
                    crate::datadogV2::model::AiCustomRuleRevisionExecutionMode,
                > = None;
                let mut globs: Option<Vec<String>> = None;
                let mut is_published: Option<bool> = None;
                let mut is_testing: Option<bool> = None;
                let mut severity: Option<
                    crate::datadogV2::model::CustomRuleRevisionAttributesSeverity,
                > = None;
                let mut short_description: Option<String> = None;
                let mut version_id: Option<i64> = None;
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
                        "content" => {
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cwe" => {
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
                        "globs" => {
                            globs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_published" => {
                            is_published =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_testing" => {
                            is_testing = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let category = category.ok_or_else(|| M::Error::missing_field("category"))?;
                let content = content.ok_or_else(|| M::Error::missing_field("content"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let directories =
                    directories.ok_or_else(|| M::Error::missing_field("directories"))?;
                let execution_mode =
                    execution_mode.ok_or_else(|| M::Error::missing_field("execution_mode"))?;
                let globs = globs.ok_or_else(|| M::Error::missing_field("globs"))?;
                let is_published =
                    is_published.ok_or_else(|| M::Error::missing_field("is_published"))?;
                let is_testing = is_testing.ok_or_else(|| M::Error::missing_field("is_testing"))?;
                let severity = severity.ok_or_else(|| M::Error::missing_field("severity"))?;
                let short_description = short_description
                    .ok_or_else(|| M::Error::missing_field("short_description"))?;

                let content = AiCustomRuleRevisionRequestAttributes {
                    category,
                    content,
                    cwe,
                    description,
                    directories,
                    execution_mode,
                    globs,
                    is_published,
                    is_testing,
                    severity,
                    short_description,
                    version_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiCustomRuleRevisionRequestAttributesVisitor)
    }
}

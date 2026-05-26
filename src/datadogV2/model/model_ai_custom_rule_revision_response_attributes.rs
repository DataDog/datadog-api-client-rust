// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response attributes of an AI custom rule revision.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiCustomRuleRevisionResponseAttributes {
    /// Rule category
    #[serde(rename = "category")]
    pub category: crate::datadogV2::model::CustomRuleRevisionAttributesCategory,
    /// Checksum of the revision content.
    #[serde(rename = "checksum")]
    pub checksum: String,
    /// Base64-encoded AI model content for this revision.
    #[serde(rename = "content")]
    pub content: String,
    /// The creation timestamp.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The identifier of the user who created the revision.
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// The associated CWE identifier.
    #[serialize_always]
    #[serde(rename = "cwe")]
    pub cwe: Option<String>,
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
    /// Whether this is a default Datadog rule.
    #[serde(rename = "is_default")]
    pub is_default: bool,
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
    pub version_id: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiCustomRuleRevisionResponseAttributes {
    pub fn new(
        category: crate::datadogV2::model::CustomRuleRevisionAttributesCategory,
        checksum: String,
        content: String,
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: String,
        cwe: Option<String>,
        description: String,
        directories: Vec<String>,
        execution_mode: crate::datadogV2::model::AiCustomRuleRevisionExecutionMode,
        globs: Vec<String>,
        is_default: bool,
        is_published: bool,
        is_testing: bool,
        severity: crate::datadogV2::model::CustomRuleRevisionAttributesSeverity,
        short_description: String,
        version_id: i64,
    ) -> AiCustomRuleRevisionResponseAttributes {
        AiCustomRuleRevisionResponseAttributes {
            category,
            checksum,
            content,
            created_at,
            created_by,
            cwe,
            description,
            directories,
            execution_mode,
            globs,
            is_default,
            is_published,
            is_testing,
            severity,
            short_description,
            version_id,
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

impl<'de> Deserialize<'de> for AiCustomRuleRevisionResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiCustomRuleRevisionResponseAttributesVisitor;
        impl<'a> Visitor<'a> for AiCustomRuleRevisionResponseAttributesVisitor {
            type Value = AiCustomRuleRevisionResponseAttributes;

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
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut cwe: Option<Option<String>> = None;
                let mut description: Option<String> = None;
                let mut directories: Option<Vec<String>> = None;
                let mut execution_mode: Option<
                    crate::datadogV2::model::AiCustomRuleRevisionExecutionMode,
                > = None;
                let mut globs: Option<Vec<String>> = None;
                let mut is_default: Option<bool> = None;
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
                        "checksum" => {
                            checksum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "content" => {
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "is_default" => {
                            is_default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let checksum = checksum.ok_or_else(|| M::Error::missing_field("checksum"))?;
                let content = content.ok_or_else(|| M::Error::missing_field("content"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let cwe = cwe.ok_or_else(|| M::Error::missing_field("cwe"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let directories =
                    directories.ok_or_else(|| M::Error::missing_field("directories"))?;
                let execution_mode =
                    execution_mode.ok_or_else(|| M::Error::missing_field("execution_mode"))?;
                let globs = globs.ok_or_else(|| M::Error::missing_field("globs"))?;
                let is_default = is_default.ok_or_else(|| M::Error::missing_field("is_default"))?;
                let is_published =
                    is_published.ok_or_else(|| M::Error::missing_field("is_published"))?;
                let is_testing = is_testing.ok_or_else(|| M::Error::missing_field("is_testing"))?;
                let severity = severity.ok_or_else(|| M::Error::missing_field("severity"))?;
                let short_description = short_description
                    .ok_or_else(|| M::Error::missing_field("short_description"))?;
                let version_id = version_id.ok_or_else(|| M::Error::missing_field("version_id"))?;

                let content = AiCustomRuleRevisionResponseAttributes {
                    category,
                    checksum,
                    content,
                    created_at,
                    created_by,
                    cwe,
                    description,
                    directories,
                    execution_mode,
                    globs,
                    is_default,
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

        deserializer.deserialize_any(AiCustomRuleRevisionResponseAttributesVisitor)
    }
}

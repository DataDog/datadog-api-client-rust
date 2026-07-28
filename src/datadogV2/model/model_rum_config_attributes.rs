// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the RUM configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumConfigAttributes {
    /// Whether the RUM configuration is disabled for the organization.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
    /// Whether application tags are enforced for the RUM applications in the organization.
    #[serde(rename = "enforced_application_tags")]
    pub enforced_application_tags: bool,
    /// Timestamp of when the enforced application tags setting was last updated.
    #[serde(rename = "enforced_application_tags_updated_at")]
    pub enforced_application_tags_updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Handle of the user who last updated the enforced application tags setting.
    #[serde(rename = "enforced_application_tags_updated_by")]
    pub enforced_application_tags_updated_by: Option<String>,
    /// Version of the out-of-the-box metrics installed for the organization.
    #[serde(rename = "ootb_metrics_version")]
    pub ootb_metrics_version: Option<i64>,
    /// Timestamp of when the out-of-the-box metrics version was installed.
    #[serde(rename = "ootb_metrics_version_installed_at")]
    pub ootb_metrics_version_installed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Whether retention filters are enabled for the organization.
    #[serde(rename = "retention_filters_enabled")]
    pub retention_filters_enabled: bool,
    /// Timestamp of when the retention filters setting was last updated.
    #[serde(rename = "retention_filters_enabled_updated_at")]
    pub retention_filters_enabled_updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Handle of the user or job who last updated the retention filters setting.
    #[serde(rename = "retention_filters_enabled_updated_by")]
    pub retention_filters_enabled_updated_by: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumConfigAttributes {
    pub fn new(
        enforced_application_tags: bool,
        retention_filters_enabled: bool,
    ) -> RumConfigAttributes {
        RumConfigAttributes {
            disabled: None,
            enforced_application_tags,
            enforced_application_tags_updated_at: None,
            enforced_application_tags_updated_by: None,
            ootb_metrics_version: None,
            ootb_metrics_version_installed_at: None,
            retention_filters_enabled,
            retention_filters_enabled_updated_at: None,
            retention_filters_enabled_updated_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = Some(value);
        self
    }

    pub fn enforced_application_tags_updated_at(
        mut self,
        value: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.enforced_application_tags_updated_at = Some(value);
        self
    }

    pub fn enforced_application_tags_updated_by(mut self, value: String) -> Self {
        self.enforced_application_tags_updated_by = Some(value);
        self
    }

    pub fn ootb_metrics_version(mut self, value: i64) -> Self {
        self.ootb_metrics_version = Some(value);
        self
    }

    pub fn ootb_metrics_version_installed_at(
        mut self,
        value: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.ootb_metrics_version_installed_at = Some(value);
        self
    }

    pub fn retention_filters_enabled_updated_at(
        mut self,
        value: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.retention_filters_enabled_updated_at = Some(value);
        self
    }

    pub fn retention_filters_enabled_updated_by(mut self, value: String) -> Self {
        self.retention_filters_enabled_updated_by = Some(value);
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

impl<'de> Deserialize<'de> for RumConfigAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumConfigAttributesVisitor;
        impl<'a> Visitor<'a> for RumConfigAttributesVisitor {
            type Value = RumConfigAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut disabled: Option<bool> = None;
                let mut enforced_application_tags: Option<bool> = None;
                let mut enforced_application_tags_updated_at: Option<
                    chrono::DateTime<chrono::Utc>,
                > = None;
                let mut enforced_application_tags_updated_by: Option<String> = None;
                let mut ootb_metrics_version: Option<i64> = None;
                let mut ootb_metrics_version_installed_at: Option<chrono::DateTime<chrono::Utc>> =
                    None;
                let mut retention_filters_enabled: Option<bool> = None;
                let mut retention_filters_enabled_updated_at: Option<
                    chrono::DateTime<chrono::Utc>,
                > = None;
                let mut retention_filters_enabled_updated_by: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "disabled" => {
                            if v.is_null() {
                                continue;
                            }
                            disabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enforced_application_tags" => {
                            enforced_application_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enforced_application_tags_updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            enforced_application_tags_updated_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enforced_application_tags_updated_by" => {
                            if v.is_null() {
                                continue;
                            }
                            enforced_application_tags_updated_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ootb_metrics_version" => {
                            if v.is_null() {
                                continue;
                            }
                            ootb_metrics_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ootb_metrics_version_installed_at" => {
                            if v.is_null() {
                                continue;
                            }
                            ootb_metrics_version_installed_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retention_filters_enabled" => {
                            retention_filters_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retention_filters_enabled_updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            retention_filters_enabled_updated_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retention_filters_enabled_updated_by" => {
                            if v.is_null() {
                                continue;
                            }
                            retention_filters_enabled_updated_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let enforced_application_tags = enforced_application_tags
                    .ok_or_else(|| M::Error::missing_field("enforced_application_tags"))?;
                let retention_filters_enabled = retention_filters_enabled
                    .ok_or_else(|| M::Error::missing_field("retention_filters_enabled"))?;

                let content = RumConfigAttributes {
                    disabled,
                    enforced_application_tags,
                    enforced_application_tags_updated_at,
                    enforced_application_tags_updated_by,
                    ootb_metrics_version,
                    ootb_metrics_version_installed_at,
                    retention_filters_enabled,
                    retention_filters_enabled_updated_at,
                    retention_filters_enabled_updated_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumConfigAttributesVisitor)
    }
}

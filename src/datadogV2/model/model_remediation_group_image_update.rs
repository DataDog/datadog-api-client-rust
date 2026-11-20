// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An image or package to upgrade, for a group of host and container vulnerabilities (`cloudvm.image_update`).
/// The group gathers the open vulnerabilities of running production images that have a fix available, and groups
/// them by `grouping_location`, `remediation_kind`, and `action`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemediationGroupImageUpdate {
    /// Name of the container image, base image, or package to upgrade. The value comes from the first of these
    /// attributes that the findings have: `@remediation.container_image.latest_major.name`,
    /// `@remediation.base_image.latest_major.name`, or `@remediation.recommended.name`.
    #[serde(rename = "action")]
    pub action: String,
    /// Number of distinct values of `@advisory.id` across the group's findings.
    #[serde(rename = "advisory_count")]
    pub advisory_count: i64,
    /// Number of distinct values of `@advisory.id` across the group's findings that have
    /// `@risk.has_exploit_available` set to `true`.
    #[serde(rename = "exploitable_cve_count")]
    pub exploitable_cve_count: i64,
    /// Number of distinct values of `@code_location.filename` across the group's findings.
    #[serde(rename = "file_count")]
    pub file_count: i64,
    /// File that contains the findings, from their `@code_location.filename` attribute. When no file is known,
    /// the value is the findings' `@container_image.name`.
    #[serde(rename = "grouping_location")]
    pub grouping_location: String,
    /// Whether `grouping_location` is a `@code_location.filename` rather than a `@container_image.name`.
    #[serde(rename = "location_is_filename")]
    pub location_is_filename: bool,
    /// Kind of upgrade: `image_update` when the findings have a `@remediation.container_image.latest_major.name`,
    /// `base_image_bump` when they have `@vulnerability.is_inherited_from_base_image` set to `true`, and
    /// `package_update` otherwise.
    #[serde(rename = "remediation_kind")]
    pub remediation_kind: String,
    /// Version to upgrade to: the `@remediation.container_image.latest_major.tag` or
    /// `@remediation.base_image.latest_major.tag` of the image in `action`, or else the highest
    /// `@remediation.recommended.version` across the group's findings. Omitted when no version is recommended
    /// or when the recommended versions can't be ordered.
    #[serde(rename = "target_version")]
    pub target_version: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RemediationGroupImageUpdate {
    pub fn new(
        action: String,
        advisory_count: i64,
        exploitable_cve_count: i64,
        file_count: i64,
        grouping_location: String,
        location_is_filename: bool,
        remediation_kind: String,
    ) -> RemediationGroupImageUpdate {
        RemediationGroupImageUpdate {
            action,
            advisory_count,
            exploitable_cve_count,
            file_count,
            grouping_location,
            location_is_filename,
            remediation_kind,
            target_version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn target_version(mut self, value: String) -> Self {
        self.target_version = Some(value);
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

impl<'de> Deserialize<'de> for RemediationGroupImageUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RemediationGroupImageUpdateVisitor;
        impl<'a> Visitor<'a> for RemediationGroupImageUpdateVisitor {
            type Value = RemediationGroupImageUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<String> = None;
                let mut advisory_count: Option<i64> = None;
                let mut exploitable_cve_count: Option<i64> = None;
                let mut file_count: Option<i64> = None;
                let mut grouping_location: Option<String> = None;
                let mut location_is_filename: Option<bool> = None;
                let mut remediation_kind: Option<String> = None;
                let mut target_version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action" => {
                            action = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "advisory_count" => {
                            advisory_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exploitable_cve_count" => {
                            exploitable_cve_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file_count" => {
                            file_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "grouping_location" => {
                            grouping_location =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "location_is_filename" => {
                            location_is_filename =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "remediation_kind" => {
                            remediation_kind =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target_version" => {
                            if v.is_null() {
                                continue;
                            }
                            target_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let action = action.ok_or_else(|| M::Error::missing_field("action"))?;
                let advisory_count =
                    advisory_count.ok_or_else(|| M::Error::missing_field("advisory_count"))?;
                let exploitable_cve_count = exploitable_cve_count
                    .ok_or_else(|| M::Error::missing_field("exploitable_cve_count"))?;
                let file_count = file_count.ok_or_else(|| M::Error::missing_field("file_count"))?;
                let grouping_location = grouping_location
                    .ok_or_else(|| M::Error::missing_field("grouping_location"))?;
                let location_is_filename = location_is_filename
                    .ok_or_else(|| M::Error::missing_field("location_is_filename"))?;
                let remediation_kind =
                    remediation_kind.ok_or_else(|| M::Error::missing_field("remediation_kind"))?;

                let content = RemediationGroupImageUpdate {
                    action,
                    advisory_count,
                    exploitable_cve_count,
                    file_count,
                    grouping_location,
                    location_is_filename,
                    remediation_kind,
                    target_version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RemediationGroupImageUpdateVisitor)
    }
}

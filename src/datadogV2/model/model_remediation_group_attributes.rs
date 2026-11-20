// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a remediation group. Each group includes one kind object, `package_update` or `image_update`,
/// depending on its grouping definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemediationGroupAttributes {
    /// Counts of the group's open findings. The counts include every finding in the group, including findings that
    /// don't match `filter[query]`.
    #[serde(rename = "findings")]
    pub findings: crate::datadogV2::model::RemediationGroupFindings,
    /// Grouping definition that formed the group: `sca.package_update` or `cloudvm.image_update`.
    #[serde(rename = "grouping_definition_id")]
    pub grouping_definition_id: String,
    /// An image or package to upgrade, for a group of host and container vulnerabilities (`cloudvm.image_update`).
    /// The group gathers the open vulnerabilities of running production images that have a fix available, and groups
    /// them by `grouping_location`, `remediation_kind`, and `action`.
    #[serde(rename = "image_update")]
    pub image_update: Option<crate::datadogV2::model::RemediationGroupImageUpdate>,
    /// A package to update in a repository, for a group of library vulnerabilities (`sca.package_update`). The group
    /// gathers the open vulnerabilities of direct dependencies that have a recommended fix, and groups them by
    /// `@git.repository_id`, `@vulnerability.stack.ecosystem`, and `@remediation.recommended.name`.
    #[serde(rename = "package_update")]
    pub package_update: Option<crate::datadogV2::model::RemediationGroupPackageUpdate>,
    /// Title of the group, which describes the change to make.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RemediationGroupAttributes {
    pub fn new(
        findings: crate::datadogV2::model::RemediationGroupFindings,
        grouping_definition_id: String,
        title: String,
    ) -> RemediationGroupAttributes {
        RemediationGroupAttributes {
            findings,
            grouping_definition_id,
            image_update: None,
            package_update: None,
            title,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn image_update(
        mut self,
        value: crate::datadogV2::model::RemediationGroupImageUpdate,
    ) -> Self {
        self.image_update = Some(value);
        self
    }

    pub fn package_update(
        mut self,
        value: crate::datadogV2::model::RemediationGroupPackageUpdate,
    ) -> Self {
        self.package_update = Some(value);
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

impl<'de> Deserialize<'de> for RemediationGroupAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RemediationGroupAttributesVisitor;
        impl<'a> Visitor<'a> for RemediationGroupAttributesVisitor {
            type Value = RemediationGroupAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut findings: Option<crate::datadogV2::model::RemediationGroupFindings> = None;
                let mut grouping_definition_id: Option<String> = None;
                let mut image_update: Option<crate::datadogV2::model::RemediationGroupImageUpdate> =
                    None;
                let mut package_update: Option<
                    crate::datadogV2::model::RemediationGroupPackageUpdate,
                > = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "findings" => {
                            findings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "grouping_definition_id" => {
                            grouping_definition_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "image_update" => {
                            if v.is_null() {
                                continue;
                            }
                            image_update =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "package_update" => {
                            if v.is_null() {
                                continue;
                            }
                            package_update =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let findings = findings.ok_or_else(|| M::Error::missing_field("findings"))?;
                let grouping_definition_id = grouping_definition_id
                    .ok_or_else(|| M::Error::missing_field("grouping_definition_id"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = RemediationGroupAttributes {
                    findings,
                    grouping_definition_id,
                    image_update,
                    package_update,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RemediationGroupAttributesVisitor)
    }
}

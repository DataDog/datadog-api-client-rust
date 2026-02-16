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
pub struct JiraIssueFinding {
    /// Description of the finding.
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "ids")]
    pub ids: Vec<crate::datadogV2::model::JiraIssueFindingId>,
    /// Number of impacted resources.
    #[serde(rename = "impacted")]
    pub impacted: Option<i64>,
    /// References for the finding.
    #[serde(rename = "references")]
    pub references: String,
    /// Remediation instructions for the finding.
    #[serde(rename = "remediation")]
    pub remediation: String,
    /// The status of the finding.
    #[serde(rename = "severity")]
    pub severity: crate::datadogV2::model::FindingStatus,
    /// Title of the finding.
    #[serde(rename = "title")]
    pub title: String,
    /// Type of the finding.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JiraIssueFinding {
    pub fn new(
        description: String,
        ids: Vec<crate::datadogV2::model::JiraIssueFindingId>,
        references: String,
        remediation: String,
        severity: crate::datadogV2::model::FindingStatus,
        title: String,
        type_: String,
    ) -> JiraIssueFinding {
        JiraIssueFinding {
            description,
            ids,
            impacted: None,
            references,
            remediation,
            severity,
            title,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn impacted(mut self, value: i64) -> Self {
        self.impacted = Some(value);
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

impl<'de> Deserialize<'de> for JiraIssueFinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JiraIssueFindingVisitor;
        impl<'a> Visitor<'a> for JiraIssueFindingVisitor {
            type Value = JiraIssueFinding;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut ids: Option<Vec<crate::datadogV2::model::JiraIssueFindingId>> = None;
                let mut impacted: Option<i64> = None;
                let mut references: Option<String> = None;
                let mut remediation: Option<String> = None;
                let mut severity: Option<crate::datadogV2::model::FindingStatus> = None;
                let mut title: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ids" => {
                            ids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "impacted" => {
                            if v.is_null() {
                                continue;
                            }
                            impacted = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "references" => {
                            references = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "remediation" => {
                            remediation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severity" => {
                            severity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _severity) = severity {
                                match _severity {
                                    crate::datadogV2::model::FindingStatus::UnparsedObject(
                                        _severity,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let ids = ids.ok_or_else(|| M::Error::missing_field("ids"))?;
                let references = references.ok_or_else(|| M::Error::missing_field("references"))?;
                let remediation =
                    remediation.ok_or_else(|| M::Error::missing_field("remediation"))?;
                let severity = severity.ok_or_else(|| M::Error::missing_field("severity"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = JiraIssueFinding {
                    description,
                    ids,
                    impacted,
                    references,
                    remediation,
                    severity,
                    title,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JiraIssueFindingVisitor)
    }
}

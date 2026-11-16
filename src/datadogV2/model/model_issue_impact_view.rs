// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A RUM view where the issue occurred.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueImpactView {
    /// Distinct users, accounts, sessions, and view instances on a view. `affected` counts only those with the issue's
    /// errors. `baseline` counts all users, accounts, sessions, and view instances on the view in the time window. It
    /// is absent when RUM has no data for the view.
    #[serde(rename = "affected")]
    pub affected: crate::datadogV2::model::IssueImpactViewCounts,
    /// Distinct users, accounts, sessions, and view instances on a view. `affected` counts only those with the issue's
    /// errors. `baseline` counts all users, accounts, sessions, and view instances on the view in the time window. It
    /// is absent when RUM has no data for the view.
    #[serde(rename = "baseline")]
    pub baseline: Option<crate::datadogV2::model::IssueImpactViewCounts>,
    /// Number of errors reported by each source. Several sources can report the same error, such as an APM span and
    /// a log, so the counts overlap: don't add them together.
    #[serde(rename = "error_counts")]
    pub error_counts: crate::datadogV2::model::IssueImpactErrorCounts,
    /// View name.
    #[serde(rename = "name")]
    pub name: String,
    /// Whether the entry stands out as a likely driver of the issue, because it accounts for most of the issue's
    /// errors or for far more errors than the other listed entries.
    #[serde(rename = "significant")]
    pub significant: bool,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssueImpactView {
    pub fn new(
        affected: crate::datadogV2::model::IssueImpactViewCounts,
        error_counts: crate::datadogV2::model::IssueImpactErrorCounts,
        name: String,
        significant: bool,
    ) -> IssueImpactView {
        IssueImpactView {
            affected,
            baseline: None,
            error_counts,
            name,
            significant,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn baseline(mut self, value: crate::datadogV2::model::IssueImpactViewCounts) -> Self {
        self.baseline = Some(value);
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

impl<'de> Deserialize<'de> for IssueImpactView {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueImpactViewVisitor;
        impl<'a> Visitor<'a> for IssueImpactViewVisitor {
            type Value = IssueImpactView;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut affected: Option<crate::datadogV2::model::IssueImpactViewCounts> = None;
                let mut baseline: Option<crate::datadogV2::model::IssueImpactViewCounts> = None;
                let mut error_counts: Option<crate::datadogV2::model::IssueImpactErrorCounts> =
                    None;
                let mut name: Option<String> = None;
                let mut significant: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "affected" => {
                            affected = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "baseline" => {
                            if v.is_null() {
                                continue;
                            }
                            baseline = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_counts" => {
                            error_counts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "significant" => {
                            significant =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let affected = affected.ok_or_else(|| M::Error::missing_field("affected"))?;
                let error_counts =
                    error_counts.ok_or_else(|| M::Error::missing_field("error_counts"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let significant =
                    significant.ok_or_else(|| M::Error::missing_field("significant"))?;

                let content = IssueImpactView {
                    affected,
                    baseline,
                    error_counts,
                    name,
                    significant,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueImpactViewVisitor)
    }
}

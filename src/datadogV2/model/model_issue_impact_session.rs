// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A RUM session where the issue occurred.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueImpactSession {
    /// Number of errors reported by each source. Several sources can report the same error, such as an APM span and
    /// a log, so the counts overlap: don't add them together.
    #[serde(rename = "error_counts")]
    pub error_counts: crate::datadogV2::model::IssueImpactErrorCounts,
    /// Session identifier.
    #[serde(rename = "id")]
    pub id: String,
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

impl IssueImpactSession {
    pub fn new(
        error_counts: crate::datadogV2::model::IssueImpactErrorCounts,
        id: String,
        significant: bool,
    ) -> IssueImpactSession {
        IssueImpactSession {
            error_counts,
            id,
            significant,
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

impl<'de> Deserialize<'de> for IssueImpactSession {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueImpactSessionVisitor;
        impl<'a> Visitor<'a> for IssueImpactSessionVisitor {
            type Value = IssueImpactSession;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error_counts: Option<crate::datadogV2::model::IssueImpactErrorCounts> =
                    None;
                let mut id: Option<String> = None;
                let mut significant: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "error_counts" => {
                            error_counts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let error_counts =
                    error_counts.ok_or_else(|| M::Error::missing_field("error_counts"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let significant =
                    significant.ok_or_else(|| M::Error::missing_field("significant"))?;

                let content = IssueImpactSession {
                    error_counts,
                    id,
                    significant,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueImpactSessionVisitor)
    }
}

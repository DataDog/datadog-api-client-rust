// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A suppression version with a list of updates.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SuppressionVersions {
    /// A list of changes.
    #[serde(rename = "changes")]
    pub changes: Option<Vec<crate::datadogV2::model::VersionHistoryUpdate>>,
    /// The attributes of the suppression rule.
    #[serde(rename = "suppression")]
    pub suppression: Option<crate::datadogV2::model::SecurityMonitoringSuppressionAttributes>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SuppressionVersions {
    pub fn new() -> SuppressionVersions {
        SuppressionVersions {
            changes: None,
            suppression: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn changes(mut self, value: Vec<crate::datadogV2::model::VersionHistoryUpdate>) -> Self {
        self.changes = Some(value);
        self
    }

    pub fn suppression(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringSuppressionAttributes,
    ) -> Self {
        self.suppression = Some(value);
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

impl Default for SuppressionVersions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SuppressionVersions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SuppressionVersionsVisitor;
        impl<'a> Visitor<'a> for SuppressionVersionsVisitor {
            type Value = SuppressionVersions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut changes: Option<Vec<crate::datadogV2::model::VersionHistoryUpdate>> = None;
                let mut suppression: Option<
                    crate::datadogV2::model::SecurityMonitoringSuppressionAttributes,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "changes" => {
                            if v.is_null() {
                                continue;
                            }
                            changes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "suppression" => {
                            if v.is_null() {
                                continue;
                            }
                            suppression =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SuppressionVersions {
                    changes,
                    suppression,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SuppressionVersionsVisitor)
    }
}

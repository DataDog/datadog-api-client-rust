// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Number of the group's findings for each value of their `@severity` attribute. The response includes every
/// level, including levels with a count of zero.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemediationGroupFindingsBySeverity {
    /// Number of critical findings.
    #[serde(rename = "critical")]
    pub critical: i64,
    /// Number of high findings.
    #[serde(rename = "high")]
    pub high: i64,
    /// Number of info findings.
    #[serde(rename = "info")]
    pub info: i64,
    /// Number of low findings.
    #[serde(rename = "low")]
    pub low: i64,
    /// Number of medium findings.
    #[serde(rename = "medium")]
    pub medium: i64,
    /// Number of findings with no severity.
    #[serde(rename = "none")]
    pub none: i64,
    /// Number of findings with an unknown severity.
    #[serde(rename = "unknown")]
    pub unknown: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RemediationGroupFindingsBySeverity {
    pub fn new(
        critical: i64,
        high: i64,
        info: i64,
        low: i64,
        medium: i64,
        none: i64,
        unknown: i64,
    ) -> RemediationGroupFindingsBySeverity {
        RemediationGroupFindingsBySeverity {
            critical,
            high,
            info,
            low,
            medium,
            none,
            unknown,
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

impl<'de> Deserialize<'de> for RemediationGroupFindingsBySeverity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RemediationGroupFindingsBySeverityVisitor;
        impl<'a> Visitor<'a> for RemediationGroupFindingsBySeverityVisitor {
            type Value = RemediationGroupFindingsBySeverity;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut critical: Option<i64> = None;
                let mut high: Option<i64> = None;
                let mut info: Option<i64> = None;
                let mut low: Option<i64> = None;
                let mut medium: Option<i64> = None;
                let mut none: Option<i64> = None;
                let mut unknown: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "critical" => {
                            critical = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "high" => {
                            high = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "info" => {
                            info = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "low" => {
                            low = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "medium" => {
                            medium = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "none" => {
                            none = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unknown" => {
                            unknown = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let critical = critical.ok_or_else(|| M::Error::missing_field("critical"))?;
                let high = high.ok_or_else(|| M::Error::missing_field("high"))?;
                let info = info.ok_or_else(|| M::Error::missing_field("info"))?;
                let low = low.ok_or_else(|| M::Error::missing_field("low"))?;
                let medium = medium.ok_or_else(|| M::Error::missing_field("medium"))?;
                let none = none.ok_or_else(|| M::Error::missing_field("none"))?;
                let unknown = unknown.ok_or_else(|| M::Error::missing_field("unknown"))?;

                let content = RemediationGroupFindingsBySeverity {
                    critical,
                    high,
                    info,
                    low,
                    medium,
                    none,
                    unknown,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RemediationGroupFindingsBySeverityVisitor)
    }
}

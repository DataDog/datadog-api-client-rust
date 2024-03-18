// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Vulnerability counts associated with the Container Image.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ContainerImageVulnerabilities {
    /// ID of the Container Image.
    #[serde(rename = "asset_id")]
    pub asset_id: Option<String>,
    /// Number of vulnerabilities with CVSS Critical severity.
    #[serde(rename = "critical")]
    pub critical: Option<i64>,
    /// Number of vulnerabilities with CVSS High severity.
    #[serde(rename = "high")]
    pub high: Option<i64>,
    /// Number of vulnerabilities with CVSS Low severity.
    #[serde(rename = "low")]
    pub low: Option<i64>,
    /// Number of vulnerabilities with CVSS Medium severity.
    #[serde(rename = "medium")]
    pub medium: Option<i64>,
    /// Number of vulnerabilities with CVSS None severity.
    #[serde(rename = "none")]
    pub none: Option<i64>,
    /// Number of vulnerabilities with an unknown CVSS severity.
    #[serde(rename = "unknown")]
    pub unknown: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ContainerImageVulnerabilities {
    pub fn new() -> ContainerImageVulnerabilities {
        ContainerImageVulnerabilities {
            asset_id: None,
            critical: None,
            high: None,
            low: None,
            medium: None,
            none: None,
            unknown: None,
            _unparsed: false,
        }
    }

    pub fn asset_id(mut self, value: String) -> Self {
        self.asset_id = Some(value);
        self
    }

    pub fn critical(mut self, value: i64) -> Self {
        self.critical = Some(value);
        self
    }

    pub fn high(mut self, value: i64) -> Self {
        self.high = Some(value);
        self
    }

    pub fn low(mut self, value: i64) -> Self {
        self.low = Some(value);
        self
    }

    pub fn medium(mut self, value: i64) -> Self {
        self.medium = Some(value);
        self
    }

    pub fn none(mut self, value: i64) -> Self {
        self.none = Some(value);
        self
    }

    pub fn unknown(mut self, value: i64) -> Self {
        self.unknown = Some(value);
        self
    }
}

impl Default for ContainerImageVulnerabilities {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ContainerImageVulnerabilities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContainerImageVulnerabilitiesVisitor;
        impl<'a> Visitor<'a> for ContainerImageVulnerabilitiesVisitor {
            type Value = ContainerImageVulnerabilities;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut asset_id: Option<String> = None;
                let mut critical: Option<i64> = None;
                let mut high: Option<i64> = None;
                let mut low: Option<i64> = None;
                let mut medium: Option<i64> = None;
                let mut none: Option<i64> = None;
                let mut unknown: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "asset_id" => {
                            if v.is_null() {
                                continue;
                            }
                            asset_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "critical" => {
                            if v.is_null() {
                                continue;
                            }
                            critical = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "high" => {
                            if v.is_null() {
                                continue;
                            }
                            high = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "low" => {
                            if v.is_null() {
                                continue;
                            }
                            low = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "medium" => {
                            if v.is_null() {
                                continue;
                            }
                            medium = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "none" => {
                            if v.is_null() {
                                continue;
                            }
                            none = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unknown" => {
                            if v.is_null() {
                                continue;
                            }
                            unknown = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ContainerImageVulnerabilities {
                    asset_id,
                    critical,
                    high,
                    low,
                    medium,
                    none,
                    unknown,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ContainerImageVulnerabilitiesVisitor)
    }
}

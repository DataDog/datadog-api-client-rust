// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Meta response containing information about the API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SensitiveDataScannerMeta {
    /// Maximum number of scanning rules allowed for the org.
    #[serde(rename = "count_limit")]
    pub count_limit: Option<i64>,
    /// Maximum number of scanning groups allowed for the org.
    #[serde(rename = "group_count_limit")]
    pub group_count_limit: Option<i64>,
    /// Whether or not scanned events are highlighted in Logs or RUM for the org.
    #[serde(rename = "has_highlight_enabled")]
    pub has_highlight_enabled: Option<bool>,
    /// Whether or not scanned events have multi-pass enabled.
    #[serde(rename = "has_multi_pass_enabled")]
    pub has_multi_pass_enabled: Option<bool>,
    /// Whether or not the org is compliant to the payment card industry standard.
    #[serde(rename = "is_pci_compliant")]
    pub is_pci_compliant: Option<bool>,
    /// Version of the API.
    #[serde(rename = "version")]
    pub version: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SensitiveDataScannerMeta {
    pub fn new() -> SensitiveDataScannerMeta {
        SensitiveDataScannerMeta {
            count_limit: None,
            group_count_limit: None,
            has_highlight_enabled: None,
            has_multi_pass_enabled: None,
            is_pci_compliant: None,
            version: None,
            _unparsed: false,
        }
    }

    pub fn count_limit(mut self, value: i64) -> Self {
        self.count_limit = Some(value);
        self
    }

    pub fn group_count_limit(mut self, value: i64) -> Self {
        self.group_count_limit = Some(value);
        self
    }

    pub fn has_highlight_enabled(mut self, value: bool) -> Self {
        self.has_highlight_enabled = Some(value);
        self
    }

    pub fn has_multi_pass_enabled(mut self, value: bool) -> Self {
        self.has_multi_pass_enabled = Some(value);
        self
    }

    pub fn is_pci_compliant(mut self, value: bool) -> Self {
        self.is_pci_compliant = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SensitiveDataScannerMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SensitiveDataScannerMetaVisitor;
        impl<'a> Visitor<'a> for SensitiveDataScannerMetaVisitor {
            type Value = SensitiveDataScannerMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count_limit: Option<i64> = None;
                let mut group_count_limit: Option<i64> = None;
                let mut has_highlight_enabled: Option<bool> = None;
                let mut has_multi_pass_enabled: Option<bool> = None;
                let mut is_pci_compliant: Option<bool> = None;
                let mut version: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "count_limit" => {
                            if v.is_null() {
                                continue;
                            }
                            count_limit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_count_limit" => {
                            if v.is_null() {
                                continue;
                            }
                            group_count_limit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_highlight_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            has_highlight_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_multi_pass_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            has_multi_pass_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_pci_compliant" => {
                            if v.is_null() {
                                continue;
                            }
                            is_pci_compliant =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SensitiveDataScannerMeta {
                    count_limit,
                    group_count_limit,
                    has_highlight_enabled,
                    has_multi_pass_enabled,
                    is_pci_compliant,
                    version,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SensitiveDataScannerMetaVisitor)
    }
}

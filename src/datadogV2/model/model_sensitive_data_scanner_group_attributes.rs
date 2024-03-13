// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the Sensitive Data Scanner group.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SensitiveDataScannerGroupAttributes {
    /// Description of the group.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Filter for the Scanning Group.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::SensitiveDataScannerFilter>,
    /// Whether or not the group is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the group.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// List of products the scanning group applies.
    #[serde(rename = "product_list")]
    pub product_list: Option<Vec<crate::datadogV2::model::SensitiveDataScannerProduct>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SensitiveDataScannerGroupAttributes {
    pub fn new() -> SensitiveDataScannerGroupAttributes {
        SensitiveDataScannerGroupAttributes {
            description: None,
            filter: None,
            is_enabled: None,
            name: None,
            product_list: None,
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn filter(mut self, value: crate::datadogV2::model::SensitiveDataScannerFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn product_list(
        mut self,
        value: Vec<crate::datadogV2::model::SensitiveDataScannerProduct>,
    ) -> Self {
        self.product_list = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerGroupAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SensitiveDataScannerGroupAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SensitiveDataScannerGroupAttributesVisitor;
        impl<'a> Visitor<'a> for SensitiveDataScannerGroupAttributesVisitor {
            type Value = SensitiveDataScannerGroupAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut filter: Option<crate::datadogV2::model::SensitiveDataScannerFilter> = None;
                let mut is_enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut product_list: Option<
                    Vec<crate::datadogV2::model::SensitiveDataScannerProduct>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product_list" => {
                            if v.is_null() {
                                continue;
                            }
                            product_list =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SensitiveDataScannerGroupAttributes {
                    description,
                    filter,
                    is_enabled,
                    name,
                    product_list,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SensitiveDataScannerGroupAttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a scanning group of a target.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SensitiveDataScannerTargetGroupAttributes {
    /// Description of the group.
    #[serde(rename = "description")]
    pub description: String,
    /// Filter for the Scanning Group.
    #[serde(rename = "filter")]
    pub filter: crate::datadogV2::model::SensitiveDataScannerFilter,
    /// Whether or not the group is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: bool,
    /// Name of the group.
    #[serde(rename = "name")]
    pub name: String,
    /// Products the scanning group applies to.
    #[serde(rename = "product_list")]
    pub product_list: Vec<crate::datadogV2::model::SensitiveDataScannerGroupTarget>,
    /// Sampling rates per product type. Always empty for these targets.
    #[serde(rename = "samplings")]
    pub samplings: Vec<crate::datadogV2::model::SensitiveDataScannerSamplings>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SensitiveDataScannerTargetGroupAttributes {
    pub fn new(
        description: String,
        filter: crate::datadogV2::model::SensitiveDataScannerFilter,
        is_enabled: bool,
        name: String,
        product_list: Vec<crate::datadogV2::model::SensitiveDataScannerGroupTarget>,
        samplings: Vec<crate::datadogV2::model::SensitiveDataScannerSamplings>,
    ) -> SensitiveDataScannerTargetGroupAttributes {
        SensitiveDataScannerTargetGroupAttributes {
            description,
            filter,
            is_enabled,
            name,
            product_list,
            samplings,
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

impl<'de> Deserialize<'de> for SensitiveDataScannerTargetGroupAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SensitiveDataScannerTargetGroupAttributesVisitor;
        impl<'a> Visitor<'a> for SensitiveDataScannerTargetGroupAttributesVisitor {
            type Value = SensitiveDataScannerTargetGroupAttributes;

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
                    Vec<crate::datadogV2::model::SensitiveDataScannerGroupTarget>,
                > = None;
                let mut samplings: Option<
                    Vec<crate::datadogV2::model::SensitiveDataScannerSamplings>,
                > = None;
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
                        "filter" => {
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_enabled" => {
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product_list" => {
                            product_list =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "samplings" => {
                            samplings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let filter = filter.ok_or_else(|| M::Error::missing_field("filter"))?;
                let is_enabled = is_enabled.ok_or_else(|| M::Error::missing_field("is_enabled"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let product_list =
                    product_list.ok_or_else(|| M::Error::missing_field("product_list"))?;
                let samplings = samplings.ok_or_else(|| M::Error::missing_field("samplings"))?;

                let content = SensitiveDataScannerTargetGroupAttributes {
                    description,
                    filter,
                    is_enabled,
                    name,
                    product_list,
                    samplings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SensitiveDataScannerTargetGroupAttributesVisitor)
    }
}

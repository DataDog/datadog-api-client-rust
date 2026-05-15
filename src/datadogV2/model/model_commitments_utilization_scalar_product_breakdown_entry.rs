// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Per-product utilization data in a scalar utilization response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CommitmentsUtilizationScalarProductBreakdownEntry {
    /// The cloud product name.
    #[serde(rename = "product")]
    pub product: String,
    /// The utilization percentage for the product.
    #[serde(rename = "utilization")]
    pub utilization: f64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CommitmentsUtilizationScalarProductBreakdownEntry {
    pub fn new(
        product: String,
        utilization: f64,
    ) -> CommitmentsUtilizationScalarProductBreakdownEntry {
        CommitmentsUtilizationScalarProductBreakdownEntry {
            product,
            utilization,
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

impl<'de> Deserialize<'de> for CommitmentsUtilizationScalarProductBreakdownEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitmentsUtilizationScalarProductBreakdownEntryVisitor;
        impl<'a> Visitor<'a> for CommitmentsUtilizationScalarProductBreakdownEntryVisitor {
            type Value = CommitmentsUtilizationScalarProductBreakdownEntry;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut product: Option<String> = None;
                let mut utilization: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "product" => {
                            product = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "utilization" => {
                            utilization =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let product = product.ok_or_else(|| M::Error::missing_field("product"))?;
                let utilization =
                    utilization.ok_or_else(|| M::Error::missing_field("utilization"))?;

                let content = CommitmentsUtilizationScalarProductBreakdownEntry {
                    product,
                    utilization,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CommitmentsUtilizationScalarProductBreakdownEntryVisitor)
    }
}

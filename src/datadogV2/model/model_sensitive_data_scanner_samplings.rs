// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Sampling configurations for the Scanning Group.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SensitiveDataScannerSamplings {
    /// Datadog product onto which Sensitive Data Scanner can be activated.
    #[serde(rename = "product")]
    pub product: Option<crate::datadogV2::model::SensitiveDataScannerProduct>,
    /// Rate at which data in product type will be scanned, as a percentage.
    #[serde(rename = "rate")]
    pub rate: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SensitiveDataScannerSamplings {
    pub fn new() -> SensitiveDataScannerSamplings {
        SensitiveDataScannerSamplings {
            product: None,
            rate: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn product(mut self, value: crate::datadogV2::model::SensitiveDataScannerProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn rate(mut self, value: f64) -> Self {
        self.rate = Some(value);
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

impl Default for SensitiveDataScannerSamplings {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SensitiveDataScannerSamplings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SensitiveDataScannerSamplingsVisitor;
        impl<'a> Visitor<'a> for SensitiveDataScannerSamplingsVisitor {
            type Value = SensitiveDataScannerSamplings;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut product: Option<crate::datadogV2::model::SensitiveDataScannerProduct> =
                    None;
                let mut rate: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "product" => {
                            if v.is_null() {
                                continue;
                            }
                            product = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _product) = product {
                                match _product {
                                    crate::datadogV2::model::SensitiveDataScannerProduct::UnparsedObject(_product) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "rate" => {
                            if v.is_null() {
                                continue;
                            }
                            rate = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SensitiveDataScannerSamplings {
                    product,
                    rate,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SensitiveDataScannerSamplingsVisitor)
    }
}

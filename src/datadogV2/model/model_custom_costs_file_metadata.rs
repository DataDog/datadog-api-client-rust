// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Schema of a Custom Costs metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomCostsFileMetadata {
    /// Total cost in the cost file.
    #[serde(rename = "billed_cost")]
    pub billed_cost: Option<f64>,
    /// Currency used in the Custom Costs file.
    #[serde(rename = "billing_currency")]
    pub billing_currency: Option<String>,
    /// Usage charge period of a Custom Costs file.
    #[serde(rename = "charge_period")]
    pub charge_period: Option<crate::datadogV2::model::CustomCostsFileUsageChargePeriod>,
    /// Name of the Custom Costs file.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Providers contained in the Custom Costs file.
    #[serde(rename = "provider_names")]
    pub provider_names: Option<Vec<String>>,
    /// Status of the Custom Costs file.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Timestamp, in millisecond, of the upload time of the Custom Costs file.
    #[serde(rename = "uploaded_at")]
    pub uploaded_at: Option<f64>,
    /// Metadata of the user that has uploaded the Custom Costs file.
    #[serde(rename = "uploaded_by")]
    pub uploaded_by: Option<crate::datadogV2::model::CustomCostsUser>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomCostsFileMetadata {
    pub fn new() -> CustomCostsFileMetadata {
        CustomCostsFileMetadata {
            billed_cost: None,
            billing_currency: None,
            charge_period: None,
            name: None,
            provider_names: None,
            status: None,
            uploaded_at: None,
            uploaded_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn billed_cost(mut self, value: f64) -> Self {
        self.billed_cost = Some(value);
        self
    }

    pub fn billing_currency(mut self, value: String) -> Self {
        self.billing_currency = Some(value);
        self
    }

    pub fn charge_period(
        mut self,
        value: crate::datadogV2::model::CustomCostsFileUsageChargePeriod,
    ) -> Self {
        self.charge_period = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn provider_names(mut self, value: Vec<String>) -> Self {
        self.provider_names = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }

    pub fn uploaded_at(mut self, value: f64) -> Self {
        self.uploaded_at = Some(value);
        self
    }

    pub fn uploaded_by(mut self, value: crate::datadogV2::model::CustomCostsUser) -> Self {
        self.uploaded_by = Some(value);
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

impl Default for CustomCostsFileMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CustomCostsFileMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomCostsFileMetadataVisitor;
        impl<'a> Visitor<'a> for CustomCostsFileMetadataVisitor {
            type Value = CustomCostsFileMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut billed_cost: Option<f64> = None;
                let mut billing_currency: Option<String> = None;
                let mut charge_period: Option<
                    crate::datadogV2::model::CustomCostsFileUsageChargePeriod,
                > = None;
                let mut name: Option<String> = None;
                let mut provider_names: Option<Vec<String>> = None;
                let mut status: Option<String> = None;
                let mut uploaded_at: Option<f64> = None;
                let mut uploaded_by: Option<crate::datadogV2::model::CustomCostsUser> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "billed_cost" => {
                            if v.is_null() {
                                continue;
                            }
                            billed_cost =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "billing_currency" => {
                            if v.is_null() {
                                continue;
                            }
                            billing_currency =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "charge_period" => {
                            if v.is_null() {
                                continue;
                            }
                            charge_period =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider_names" => {
                            if v.is_null() {
                                continue;
                            }
                            provider_names =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uploaded_at" => {
                            if v.is_null() {
                                continue;
                            }
                            uploaded_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uploaded_by" => {
                            if v.is_null() {
                                continue;
                            }
                            uploaded_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CustomCostsFileMetadata {
                    billed_cost,
                    billing_currency,
                    charge_period,
                    name,
                    provider_names,
                    status,
                    uploaded_at,
                    uploaded_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomCostsFileMetadataVisitor)
    }
}

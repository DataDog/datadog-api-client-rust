// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of hourly usage for a product family for an org for a time period.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HourlyUsageAttributes {
    /// List of the measured usage values for the product family for the org for the time period.
    #[serde(rename = "measurements")]
    pub measurements: Option<Vec<crate::datadogV2::model::HourlyUsageMeasurement>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The product for which usage is being reported.
    #[serde(rename = "product_family")]
    pub product_family: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// The region of the Datadog instance that the organization belongs to.
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// Datetime in ISO-8601 format, UTC. The hour for the usage.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HourlyUsageAttributes {
    pub fn new() -> HourlyUsageAttributes {
        HourlyUsageAttributes {
            measurements: None,
            org_name: None,
            product_family: None,
            public_id: None,
            region: None,
            timestamp: None,
            _unparsed: false,
        }
    }

    pub fn measurements(
        mut self,
        value: Vec<crate::datadogV2::model::HourlyUsageMeasurement>,
    ) -> Self {
        self.measurements = Some(value);
        self
    }

    pub fn org_name(mut self, value: String) -> Self {
        self.org_name = Some(value);
        self
    }

    pub fn product_family(mut self, value: String) -> Self {
        self.product_family = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }

    pub fn region(mut self, value: String) -> Self {
        self.region = Some(value);
        self
    }

    pub fn timestamp(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.timestamp = Some(value);
        self
    }
}

impl Default for HourlyUsageAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HourlyUsageAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HourlyUsageAttributesVisitor;
        impl<'a> Visitor<'a> for HourlyUsageAttributesVisitor {
            type Value = HourlyUsageAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut measurements: Option<Vec<crate::datadogV2::model::HourlyUsageMeasurement>> =
                    None;
                let mut org_name: Option<String> = None;
                let mut product_family: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut region: Option<String> = None;
                let mut timestamp: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "measurements" => {
                            if v.is_null() {
                                continue;
                            }
                            measurements =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_name" => {
                            if v.is_null() {
                                continue;
                            }
                            org_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product_family" => {
                            if v.is_null() {
                                continue;
                            }
                            product_family =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region" => {
                            if v.is_null() {
                                continue;
                            }
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = HourlyUsageAttributes {
                    measurements,
                    org_name,
                    product_family,
                    public_id,
                    region,
                    timestamp,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HourlyUsageAttributesVisitor)
    }
}

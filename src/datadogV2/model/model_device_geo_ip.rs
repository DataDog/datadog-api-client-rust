// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Location derived from the device's public IP address.
/// Absent when no location data is available.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeviceGeoIP {
    /// City resolved from the device's public IP address.
    #[serde(rename = "geo_city")]
    pub geo_city: Option<String>,
    /// ISO 3166-1 alpha-2 country code resolved from the device's public IP address.
    #[serde(rename = "geo_country_iso_code")]
    pub geo_country_iso_code: Option<String>,
    /// Latitude resolved from the device's public IP address.
    #[serde(rename = "geo_latitude")]
    pub geo_latitude: Option<f64>,
    /// Longitude resolved from the device's public IP address.
    #[serde(rename = "geo_longitude")]
    pub geo_longitude: Option<f64>,
    /// ISO 3166-2 subdivision code (for example, a state or province)
    /// resolved from the device's public IP address.
    #[serde(rename = "geo_subdivision_iso_code")]
    pub geo_subdivision_iso_code: Option<String>,
    /// Public IPv4 or IPv6 address of the device.
    #[serde(rename = "public_ip_address")]
    pub public_ip_address: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeviceGeoIP {
    pub fn new() -> DeviceGeoIP {
        DeviceGeoIP {
            geo_city: None,
            geo_country_iso_code: None,
            geo_latitude: None,
            geo_longitude: None,
            geo_subdivision_iso_code: None,
            public_ip_address: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn geo_city(mut self, value: String) -> Self {
        self.geo_city = Some(value);
        self
    }

    pub fn geo_country_iso_code(mut self, value: String) -> Self {
        self.geo_country_iso_code = Some(value);
        self
    }

    pub fn geo_latitude(mut self, value: f64) -> Self {
        self.geo_latitude = Some(value);
        self
    }

    pub fn geo_longitude(mut self, value: f64) -> Self {
        self.geo_longitude = Some(value);
        self
    }

    pub fn geo_subdivision_iso_code(mut self, value: String) -> Self {
        self.geo_subdivision_iso_code = Some(value);
        self
    }

    pub fn public_ip_address(mut self, value: String) -> Self {
        self.public_ip_address = Some(value);
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

impl Default for DeviceGeoIP {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DeviceGeoIP {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeviceGeoIPVisitor;
        impl<'a> Visitor<'a> for DeviceGeoIPVisitor {
            type Value = DeviceGeoIP;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut geo_city: Option<String> = None;
                let mut geo_country_iso_code: Option<String> = None;
                let mut geo_latitude: Option<f64> = None;
                let mut geo_longitude: Option<f64> = None;
                let mut geo_subdivision_iso_code: Option<String> = None;
                let mut public_ip_address: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "geo_city" => {
                            if v.is_null() {
                                continue;
                            }
                            geo_city = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "geo_country_iso_code" => {
                            if v.is_null() {
                                continue;
                            }
                            geo_country_iso_code =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "geo_latitude" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            geo_latitude =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "geo_longitude" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            geo_longitude =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "geo_subdivision_iso_code" => {
                            if v.is_null() {
                                continue;
                            }
                            geo_subdivision_iso_code =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_ip_address" => {
                            if v.is_null() {
                                continue;
                            }
                            public_ip_address =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DeviceGeoIP {
                    geo_city,
                    geo_country_iso_code,
                    geo_latitude,
                    geo_longitude,
                    geo_subdivision_iso_code,
                    public_ip_address,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeviceGeoIPVisitor)
    }
}

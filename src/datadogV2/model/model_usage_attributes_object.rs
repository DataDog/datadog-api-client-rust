// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Usage attributes data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageAttributesObject {
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
    /// List of usage data reported for each requested hour.
    #[serde(rename = "timeseries")]
    pub timeseries: Option<Vec<crate::datadogV2::model::UsageTimeSeriesObject>>,
    /// Usage type that is being measured.
    #[serde(rename = "usage_type")]
    pub usage_type: Option<crate::datadogV2::model::HourlyUsageType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageAttributesObject {
    pub fn new() -> UsageAttributesObject {
        UsageAttributesObject {
            org_name: None,
            product_family: None,
            public_id: None,
            region: None,
            timeseries: None,
            usage_type: None,
            _unparsed: false,
        }
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

    pub fn timeseries(
        mut self,
        value: Vec<crate::datadogV2::model::UsageTimeSeriesObject>,
    ) -> Self {
        self.timeseries = Some(value);
        self
    }

    pub fn usage_type(mut self, value: crate::datadogV2::model::HourlyUsageType) -> Self {
        self.usage_type = Some(value);
        self
    }
}

impl Default for UsageAttributesObject {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageAttributesObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageAttributesObjectVisitor;
        impl<'a> Visitor<'a> for UsageAttributesObjectVisitor {
            type Value = UsageAttributesObject;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut org_name: Option<String> = None;
                let mut product_family: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut region: Option<String> = None;
                let mut timeseries: Option<Vec<crate::datadogV2::model::UsageTimeSeriesObject>> =
                    None;
                let mut usage_type: Option<crate::datadogV2::model::HourlyUsageType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        "timeseries" => {
                            if v.is_null() {
                                continue;
                            }
                            timeseries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "usage_type" => {
                            if v.is_null() {
                                continue;
                            }
                            usage_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _usage_type) = usage_type {
                                match _usage_type {
                                    crate::datadogV2::model::HourlyUsageType::UnparsedObject(
                                        _usage_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = UsageAttributesObject {
                    org_name,
                    product_family,
                    public_id,
                    region,
                    timeseries,
                    usage_type,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageAttributesObjectVisitor)
    }
}

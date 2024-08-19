// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A distribution points metric to submit to Datadog.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DistributionPointsSeries {
    /// The name of the host that produced the distribution point metric.
    #[serde(rename = "host")]
    pub host: Option<String>,
    /// The name of the distribution points metric.
    #[serde(rename = "metric")]
    pub metric: String,
    /// Points relating to the distribution point metric. All points must be tuples with timestamp and a list of values (cannot be a string). Timestamps should be in POSIX time in seconds.
    #[serde(rename = "points")]
    pub points: Vec<Vec<crate::datadogV1::model::DistributionPointItem>>,
    /// A list of tags associated with the distribution point metric.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The type of the distribution point.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::DistributionPointsType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DistributionPointsSeries {
    pub fn new(
        metric: String,
        points: Vec<Vec<crate::datadogV1::model::DistributionPointItem>>,
    ) -> DistributionPointsSeries {
        DistributionPointsSeries {
            host: None,
            metric,
            points,
            tags: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn host(mut self, value: String) -> Self {
        self.host = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV1::model::DistributionPointsType) -> Self {
        self.type_ = Some(value);
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

impl<'de> Deserialize<'de> for DistributionPointsSeries {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DistributionPointsSeriesVisitor;
        impl<'a> Visitor<'a> for DistributionPointsSeriesVisitor {
            type Value = DistributionPointsSeries;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut host: Option<String> = None;
                let mut metric: Option<String> = None;
                let mut points: Option<Vec<Vec<crate::datadogV1::model::DistributionPointItem>>> =
                    None;
                let mut tags: Option<Vec<String>> = None;
                let mut type_: Option<crate::datadogV1::model::DistributionPointsType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "host" => {
                            if v.is_null() {
                                continue;
                            }
                            host = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric" => {
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "points" => {
                            points = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::DistributionPointsType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let metric = metric.ok_or_else(|| M::Error::missing_field("metric"))?;
                let points = points.ok_or_else(|| M::Error::missing_field("points"))?;

                let content = DistributionPointsSeries {
                    host,
                    metric,
                    points,
                    tags,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DistributionPointsSeriesVisitor)
    }
}

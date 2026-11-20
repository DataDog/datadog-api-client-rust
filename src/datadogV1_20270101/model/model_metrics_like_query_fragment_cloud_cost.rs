// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricsLikeQueryFragmentCloudCost {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::MetricsLikeQueryFragmentCloudCostDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "filter")]
    pub filter: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "metric_name")]
    pub metric_name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl MetricsLikeQueryFragmentCloudCost {
    pub fn new(data_source: crate::datadogV1_20270101::model::MetricsLikeQueryFragmentCloudCostDataSource, metric_name: String, name: String) -> MetricsLikeQueryFragmentCloudCost {
        MetricsLikeQueryFragmentCloudCost {
            alias: None,
            data_source,
            filter: None,
            metric_name,
            name,
            _unparsed: false,
        }
    }
    
    pub fn alias(mut self, value: String) -> Self {
        self.alias = Some(value);
        self
    }
    
    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for MetricsLikeQueryFragmentCloudCost {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricsLikeQueryFragmentCloudCostVisitor;
        impl<'a> Visitor<'a> for MetricsLikeQueryFragmentCloudCostVisitor {
            type Value = MetricsLikeQueryFragmentCloudCost;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alias: Option<String> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::MetricsLikeQueryFragmentCloudCostDataSource> = None;
                let mut filter: Option<String> = None;
                let mut metric_name: Option<String> = None;
                let mut name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alias" => {
                            if v.is_null() {
                                continue;
                            }
                            alias = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::MetricsLikeQueryFragmentCloudCostDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "metric_name" => {
                            metric_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let metric_name = metric_name.ok_or_else(|| M::Error::missing_field("metric_name"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = MetricsLikeQueryFragmentCloudCost {
                    alias,
                    data_source,
                    filter,
                    metric_name,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricsLikeQueryFragmentCloudCostVisitor)
    }
}

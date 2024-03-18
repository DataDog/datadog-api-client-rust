// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A time and space aggregation combination for use in query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricCustomAggregation {
    /// A space aggregation for use in query.
    #[serde(rename = "space")]
    pub space: crate::datadogV2::model::MetricCustomSpaceAggregation,
    /// A time aggregation for use in query.
    #[serde(rename = "time")]
    pub time: crate::datadogV2::model::MetricCustomTimeAggregation,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricCustomAggregation {
    pub fn new(
        space: crate::datadogV2::model::MetricCustomSpaceAggregation,
        time: crate::datadogV2::model::MetricCustomTimeAggregation,
    ) -> MetricCustomAggregation {
        MetricCustomAggregation {
            space,
            time,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for MetricCustomAggregation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricCustomAggregationVisitor;
        impl<'a> Visitor<'a> for MetricCustomAggregationVisitor {
            type Value = MetricCustomAggregation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut space: Option<crate::datadogV2::model::MetricCustomSpaceAggregation> = None;
                let mut time: Option<crate::datadogV2::model::MetricCustomTimeAggregation> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "space" => {
                            space = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _space) = space {
                                match _space {
                                    crate::datadogV2::model::MetricCustomSpaceAggregation::UnparsedObject(_space) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "time" => {
                            time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _time) = time {
                                match _time {
                                    crate::datadogV2::model::MetricCustomTimeAggregation::UnparsedObject(_time) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let space = space.ok_or_else(|| M::Error::missing_field("space"))?;
                let time = time.ok_or_else(|| M::Error::missing_field("time"))?;

                let content = MetricCustomAggregation {
                    space,
                    time,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricCustomAggregationVisitor)
    }
}

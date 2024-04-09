// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A time-slice SLI specification.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOTimeSliceSpec {
    /// The time-slice condition, composed of 3 parts: 1. the metric timeseries query, 2. the comparator,
    /// and 3. the threshold. Optionally, a fourth part, the query interval, can be provided.
    #[serde(rename = "time_slice")]
    pub time_slice: crate::datadogV1::model::SLOTimeSliceCondition,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOTimeSliceSpec {
    pub fn new(time_slice: crate::datadogV1::model::SLOTimeSliceCondition) -> SLOTimeSliceSpec {
        SLOTimeSliceSpec {
            time_slice,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SLOTimeSliceSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOTimeSliceSpecVisitor;
        impl<'a> Visitor<'a> for SLOTimeSliceSpecVisitor {
            type Value = SLOTimeSliceSpec;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut time_slice: Option<crate::datadogV1::model::SLOTimeSliceCondition> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "time_slice" => {
                            time_slice = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let time_slice = time_slice.ok_or_else(|| M::Error::missing_field("time_slice"))?;

                let content = SLOTimeSliceSpec {
                    time_slice,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOTimeSliceSpecVisitor)
    }
}

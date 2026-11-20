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
pub struct CalendarShiftSplitConfig {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "shifts")]
    pub shifts: Vec<crate::datadogV1_20270101::model::CalendarShift>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CalendarShiftSplitConfig {
    pub fn new(shifts: Vec<crate::datadogV1_20270101::model::CalendarShift>) -> CalendarShiftSplitConfig {
        CalendarShiftSplitConfig {
            shifts,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for CalendarShiftSplitConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CalendarShiftSplitConfigVisitor;
        impl<'a> Visitor<'a> for CalendarShiftSplitConfigVisitor {
            type Value = CalendarShiftSplitConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut shifts: Option<Vec<crate::datadogV1_20270101::model::CalendarShift>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "shifts" => {
                            shifts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let shifts = shifts.ok_or_else(|| M::Error::missing_field("shifts"))?;

                let content = CalendarShiftSplitConfig {
                    shifts,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CalendarShiftSplitConfigVisitor)
    }
}

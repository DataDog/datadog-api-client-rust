// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing timeframes and timezone used for advanced scheduling.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestOptionsScheduling {
    /// Array containing objects describing the scheduling pattern to apply to each day.
    #[serde(rename = "timeframes")]
    pub timeframes: Option<Vec<crate::datadogV1::model::SyntheticsTestOptionsSchedulingTimeframe>>,
    /// Timezone in which the timeframe is based.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestOptionsScheduling {
    pub fn new() -> SyntheticsTestOptionsScheduling {
        SyntheticsTestOptionsScheduling {
            timeframes: None,
            timezone: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn timeframes(
        mut self,
        value: Vec<crate::datadogV1::model::SyntheticsTestOptionsSchedulingTimeframe>,
    ) -> Self {
        self.timeframes = Some(value);
        self
    }

    pub fn timezone(mut self, value: String) -> Self {
        self.timezone = Some(value);
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

impl Default for SyntheticsTestOptionsScheduling {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestOptionsScheduling {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestOptionsSchedulingVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestOptionsSchedulingVisitor {
            type Value = SyntheticsTestOptionsScheduling;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut timeframes: Option<
                    Vec<crate::datadogV1::model::SyntheticsTestOptionsSchedulingTimeframe>,
                > = None;
                let mut timezone: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "timeframes" => {
                            if v.is_null() {
                                continue;
                            }
                            timeframes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timezone" => {
                            if v.is_null() {
                                continue;
                            }
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestOptionsScheduling {
                    timeframes,
                    timezone,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestOptionsSchedulingVisitor)
    }
}

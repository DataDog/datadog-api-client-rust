// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes portion of the SLO report request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SloReportCreateRequestAttributes {
    /// The `from` timestamp for the report in epoch seconds.
    #[serde(rename = "from_ts")]
    pub from_ts: i64,
    /// The frequency at which report data is to be generated.
    #[serde(rename = "interval")]
    pub interval: Option<crate::datadogV2::model::SLOReportInterval>,
    /// The query string used to filter SLO results. Some examples of queries include `service:<service-name>` and `slo-name`.
    #[serde(rename = "query")]
    pub query: String,
    /// The timezone used to determine the start and end of each interval. For example, weekly intervals start at 12am on Sunday in the specified timezone.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    /// The `to` timestamp for the report in epoch seconds.
    #[serde(rename = "to_ts")]
    pub to_ts: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SloReportCreateRequestAttributes {
    pub fn new(from_ts: i64, query: String, to_ts: i64) -> SloReportCreateRequestAttributes {
        SloReportCreateRequestAttributes {
            from_ts,
            interval: None,
            query,
            timezone: None,
            to_ts,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn interval(mut self, value: crate::datadogV2::model::SLOReportInterval) -> Self {
        self.interval = Some(value);
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

impl<'de> Deserialize<'de> for SloReportCreateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SloReportCreateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for SloReportCreateRequestAttributesVisitor {
            type Value = SloReportCreateRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from_ts: Option<i64> = None;
                let mut interval: Option<crate::datadogV2::model::SLOReportInterval> = None;
                let mut query: Option<String> = None;
                let mut timezone: Option<String> = None;
                let mut to_ts: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "from_ts" => {
                            from_ts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interval" => {
                            if v.is_null() {
                                continue;
                            }
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _interval) = interval {
                                match _interval {
                                    crate::datadogV2::model::SLOReportInterval::UnparsedObject(
                                        _interval,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timezone" => {
                            if v.is_null() {
                                continue;
                            }
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to_ts" => {
                            to_ts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let from_ts = from_ts.ok_or_else(|| M::Error::missing_field("from_ts"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let to_ts = to_ts.ok_or_else(|| M::Error::missing_field("to_ts"))?;

                let content = SloReportCreateRequestAttributes {
                    from_ts,
                    interval,
                    query,
                    timezone,
                    to_ts,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SloReportCreateRequestAttributesVisitor)
    }
}

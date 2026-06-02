// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for an aggregated long tasks query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AggregatedLongTasksRequestAttributes {
    /// The RUM application ID to analyze.
    #[serde(rename = "application_id")]
    pub application_id: String,
    /// Performance criteria to filter view instances by a metric threshold.
    #[serde(rename = "criteria")]
    pub criteria: Option<crate::datadogV2::model::AggregatedWaterfallPerformanceCriteria>,
    /// RUM query string to filter events (for example, @session.type:user @geo.country:US).
    #[serde(rename = "filter")]
    pub filter: Option<String>,
    /// Start of the time range as a Unix timestamp in seconds.
    #[serde(rename = "from")]
    pub from: i64,
    /// Number of view instances to sample, between 1 and 500.
    #[serde(rename = "sample_size")]
    pub sample_size: i32,
    /// End of the time range as a Unix timestamp in seconds.
    #[serde(rename = "to")]
    pub to: i64,
    /// The RUM view name to analyze (for example, /account/login).
    #[serde(rename = "view_name")]
    pub view_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AggregatedLongTasksRequestAttributes {
    pub fn new(
        application_id: String,
        from: i64,
        sample_size: i32,
        to: i64,
        view_name: String,
    ) -> AggregatedLongTasksRequestAttributes {
        AggregatedLongTasksRequestAttributes {
            application_id,
            criteria: None,
            filter: None,
            from,
            sample_size,
            to,
            view_name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn criteria(
        mut self,
        value: crate::datadogV2::model::AggregatedWaterfallPerformanceCriteria,
    ) -> Self {
        self.criteria = Some(value);
        self
    }

    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
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

impl<'de> Deserialize<'de> for AggregatedLongTasksRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AggregatedLongTasksRequestAttributesVisitor;
        impl<'a> Visitor<'a> for AggregatedLongTasksRequestAttributesVisitor {
            type Value = AggregatedLongTasksRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<String> = None;
                let mut criteria: Option<
                    crate::datadogV2::model::AggregatedWaterfallPerformanceCriteria,
                > = None;
                let mut filter: Option<String> = None;
                let mut from: Option<i64> = None;
                let mut sample_size: Option<i32> = None;
                let mut to: Option<i64> = None;
                let mut view_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "application_id" => {
                            application_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "criteria" => {
                            if v.is_null() {
                                continue;
                            }
                            criteria = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "from" => {
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sample_size" => {
                            sample_size =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to" => {
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "view_name" => {
                            view_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let application_id =
                    application_id.ok_or_else(|| M::Error::missing_field("application_id"))?;
                let from = from.ok_or_else(|| M::Error::missing_field("from"))?;
                let sample_size =
                    sample_size.ok_or_else(|| M::Error::missing_field("sample_size"))?;
                let to = to.ok_or_else(|| M::Error::missing_field("to"))?;
                let view_name = view_name.ok_or_else(|| M::Error::missing_field("view_name"))?;

                let content = AggregatedLongTasksRequestAttributes {
                    application_id,
                    criteria,
                    filter,
                    from,
                    sample_size,
                    to,
                    view_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AggregatedLongTasksRequestAttributesVisitor)
    }
}

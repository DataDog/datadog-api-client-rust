// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Journey Paths query definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JourneyMapQuery {
    /// Whether to show the paths that convert to the end step or drop off before it.
    #[serde(rename = "conversion_type")]
    pub conversion_type: Option<crate::datadogV1::model::JourneyMapConversionType>,
    /// Whether to count unique journeys or all journeys.
    #[serde(rename = "counting_method")]
    pub counting_method: Option<crate::datadogV1::model::JourneyMapCountingMethod>,
    /// Data source for user journey funnel queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::ProductAnalyticsFunnelDataSource,
    /// Maximum number of paths shown per step.
    #[serde(rename = "entries_per_step")]
    pub entries_per_step: Option<i64>,
    /// Filter applied to the events shown along the paths.
    #[serde(rename = "event_filter")]
    pub event_filter: Option<String>,
    /// Paths expanded to show the events that follow them.
    #[serde(rename = "expanded_prefixes")]
    pub expanded_prefixes: Option<Vec<crate::datadogV1::model::JourneyMapExpandedPrefix>>,
    /// Group by configuration.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1::model::JourneyMapGroupBy>>,
    /// User journey search configuration.
    #[serde(rename = "search")]
    pub search: crate::datadogV1::model::UserJourneySearch,
    /// The pair of journey steps the paths are visualized between.
    #[serde(rename = "target")]
    pub target: crate::datadogV1::model::JourneyMapTarget,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JourneyMapQuery {
    pub fn new(
        data_source: crate::datadogV1::model::ProductAnalyticsFunnelDataSource,
        search: crate::datadogV1::model::UserJourneySearch,
        target: crate::datadogV1::model::JourneyMapTarget,
    ) -> JourneyMapQuery {
        JourneyMapQuery {
            conversion_type: None,
            counting_method: None,
            data_source,
            entries_per_step: None,
            event_filter: None,
            expanded_prefixes: None,
            group_by: None,
            search,
            target,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn conversion_type(
        mut self,
        value: crate::datadogV1::model::JourneyMapConversionType,
    ) -> Self {
        self.conversion_type = Some(value);
        self
    }

    pub fn counting_method(
        mut self,
        value: crate::datadogV1::model::JourneyMapCountingMethod,
    ) -> Self {
        self.counting_method = Some(value);
        self
    }

    pub fn entries_per_step(mut self, value: i64) -> Self {
        self.entries_per_step = Some(value);
        self
    }

    pub fn event_filter(mut self, value: String) -> Self {
        self.event_filter = Some(value);
        self
    }

    pub fn expanded_prefixes(
        mut self,
        value: Vec<crate::datadogV1::model::JourneyMapExpandedPrefix>,
    ) -> Self {
        self.expanded_prefixes = Some(value);
        self
    }

    pub fn group_by(mut self, value: Vec<crate::datadogV1::model::JourneyMapGroupBy>) -> Self {
        self.group_by = Some(value);
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

impl<'de> Deserialize<'de> for JourneyMapQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JourneyMapQueryVisitor;
        impl<'a> Visitor<'a> for JourneyMapQueryVisitor {
            type Value = JourneyMapQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut conversion_type: Option<crate::datadogV1::model::JourneyMapConversionType> =
                    None;
                let mut counting_method: Option<crate::datadogV1::model::JourneyMapCountingMethod> =
                    None;
                let mut data_source: Option<
                    crate::datadogV1::model::ProductAnalyticsFunnelDataSource,
                > = None;
                let mut entries_per_step: Option<i64> = None;
                let mut event_filter: Option<String> = None;
                let mut expanded_prefixes: Option<
                    Vec<crate::datadogV1::model::JourneyMapExpandedPrefix>,
                > = None;
                let mut group_by: Option<Vec<crate::datadogV1::model::JourneyMapGroupBy>> = None;
                let mut search: Option<crate::datadogV1::model::UserJourneySearch> = None;
                let mut target: Option<crate::datadogV1::model::JourneyMapTarget> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "conversion_type" => {
                            if v.is_null() {
                                continue;
                            }
                            conversion_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _conversion_type) = conversion_type {
                                match _conversion_type {
                                    crate::datadogV1::model::JourneyMapConversionType::UnparsedObject(_conversion_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "counting_method" => {
                            if v.is_null() {
                                continue;
                            }
                            counting_method =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _counting_method) = counting_method {
                                match _counting_method {
                                    crate::datadogV1::model::JourneyMapCountingMethod::UnparsedObject(_counting_method) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::ProductAnalyticsFunnelDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "entries_per_step" => {
                            if v.is_null() {
                                continue;
                            }
                            entries_per_step =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_filter" => {
                            if v.is_null() {
                                continue;
                            }
                            event_filter =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expanded_prefixes" => {
                            if v.is_null() {
                                continue;
                            }
                            expanded_prefixes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search" => {
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;

                let content = JourneyMapQuery {
                    conversion_type,
                    counting_method,
                    data_source,
                    entries_per_step,
                    event_filter,
                    expanded_prefixes,
                    group_by,
                    search,
                    target,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JourneyMapQueryVisitor)
    }
}

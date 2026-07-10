// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query for available values using an events-based data source (spans, logs, or rum).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardAvailableValuesEventsQuery {
    /// The events-based data source for the query.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::DashboardAvailableValuesEventsDataSource,
    /// The fields to group by in the query.
    #[serde(rename = "group_by")]
    pub group_by: Vec<crate::datadogV1::model::DashboardAvailableValuesEventsQueryGroupByItems>,
    /// The search filter for the query.
    #[serde(rename = "search")]
    pub search: crate::datadogV1::model::DashboardAvailableValuesEventsQuerySearch,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardAvailableValuesEventsQuery {
    pub fn new(
        data_source: crate::datadogV1::model::DashboardAvailableValuesEventsDataSource,
        group_by: Vec<crate::datadogV1::model::DashboardAvailableValuesEventsQueryGroupByItems>,
        search: crate::datadogV1::model::DashboardAvailableValuesEventsQuerySearch,
    ) -> DashboardAvailableValuesEventsQuery {
        DashboardAvailableValuesEventsQuery {
            data_source,
            group_by,
            search,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for DashboardAvailableValuesEventsQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardAvailableValuesEventsQueryVisitor;
        impl<'a> Visitor<'a> for DashboardAvailableValuesEventsQueryVisitor {
            type Value = DashboardAvailableValuesEventsQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<
                    crate::datadogV1::model::DashboardAvailableValuesEventsDataSource,
                > = None;
                let mut group_by: Option<
                    Vec<crate::datadogV1::model::DashboardAvailableValuesEventsQueryGroupByItems>,
                > = None;
                let mut search: Option<
                    crate::datadogV1::model::DashboardAvailableValuesEventsQuerySearch,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::DashboardAvailableValuesEventsDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "group_by" => {
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search" => {
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let group_by = group_by.ok_or_else(|| M::Error::missing_field("group_by"))?;
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;

                let content = DashboardAvailableValuesEventsQuery {
                    data_source,
                    group_by,
                    search,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardAvailableValuesEventsQueryVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Retention query definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RetentionQuery {
    /// Compute configuration for retention queries.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV1::model::RetentionCompute,
    /// Data source for retention queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::RetentionDataSource,
    /// Filters for retention queries.
    #[serde(rename = "filters")]
    pub filters: Option<crate::datadogV1::model::RetentionFilters>,
    /// Group by configuration.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1::model::RetentionGroupBy>>,
    /// Name of the query.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Search configuration for retention queries.
    #[serde(rename = "search")]
    pub search: crate::datadogV1::model::RetentionSearch,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RetentionQuery {
    pub fn new(
        compute: crate::datadogV1::model::RetentionCompute,
        data_source: crate::datadogV1::model::RetentionDataSource,
        search: crate::datadogV1::model::RetentionSearch,
    ) -> RetentionQuery {
        RetentionQuery {
            compute,
            data_source,
            filters: None,
            group_by: None,
            name: None,
            search,
            _unparsed: false,
        }
    }

    pub fn filters(mut self, value: crate::datadogV1::model::RetentionFilters) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn group_by(mut self, value: Vec<crate::datadogV1::model::RetentionGroupBy>) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for RetentionQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RetentionQueryVisitor;
        impl<'a> Visitor<'a> for RetentionQueryVisitor {
            type Value = RetentionQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<crate::datadogV1::model::RetentionCompute> = None;
                let mut data_source: Option<crate::datadogV1::model::RetentionDataSource> = None;
                let mut filters: Option<crate::datadogV1::model::RetentionFilters> = None;
                let mut group_by: Option<Vec<crate::datadogV1::model::RetentionGroupBy>> = None;
                let mut name: Option<String> = None;
                let mut search: Option<crate::datadogV1::model::RetentionSearch> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute" => {
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::RetentionDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "filters" => {
                            if v.is_null() {
                                continue;
                            }
                            filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;

                let content = RetentionQuery {
                    compute,
                    data_source,
                    filters,
                    group_by,
                    name,
                    search,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RetentionQueryVisitor)
    }
}

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
pub struct ProductAnalyticsJourneyDataset {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "audience_filters")]
    pub audience_filters: Option<crate::datadogV1_20270101::model::AudienceFilters>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "columns")]
    pub columns: Vec<crate::datadogV1_20270101::model::DatasetColumn>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "conversion_time_ms", default, with = "::serde_with::rust::double_option")]
    pub conversion_time_ms: Option<Option<f64>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::ProductAnalyticsJourneyDatasetDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "entity")]
    pub entity: crate::datadogV1_20270101::model::ProductAnalyticsJourneyDatasetEntity,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "expression")]
    pub expression: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "node_objects")]
    pub node_objects: std::collections::BTreeMap<String, crate::datadogV1_20270101::model::ProductAnalyticsEventQuery>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "search")]
    pub search: Option<crate::datadogV1_20270101::model::ProductAnalyticsJourneyDatasetSearch>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "time_window")]
    pub time_window: Option<crate::datadogV1_20270101::model::TimeWindow>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ProductAnalyticsJourneyDataset {
    pub fn new(columns: Vec<crate::datadogV1_20270101::model::DatasetColumn>, data_source: crate::datadogV1_20270101::model::ProductAnalyticsJourneyDatasetDataSource, entity: crate::datadogV1_20270101::model::ProductAnalyticsJourneyDatasetEntity, expression: String, name: String, node_objects: std::collections::BTreeMap<String, crate::datadogV1_20270101::model::ProductAnalyticsEventQuery>) -> ProductAnalyticsJourneyDataset {
        ProductAnalyticsJourneyDataset {
            audience_filters: None,
            columns,
            conversion_time_ms: None,
            data_source,
            entity,
            expression,
            name,
            node_objects,
            search: None,
            time_window: None,
            _unparsed: false,
        }
    }
    
    pub fn audience_filters(mut self, value: crate::datadogV1_20270101::model::AudienceFilters) -> Self {
        self.audience_filters = Some(value);
        self
    }
    
    pub fn conversion_time_ms(mut self, value: Option<f64>) -> Self {
        self.conversion_time_ms = Some(value);
        self
    }
    
    pub fn search(mut self, value: crate::datadogV1_20270101::model::ProductAnalyticsJourneyDatasetSearch) -> Self {
        self.search = Some(value);
        self
    }
    
    pub fn time_window(mut self, value: crate::datadogV1_20270101::model::TimeWindow) -> Self {
        self.time_window = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ProductAnalyticsJourneyDataset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsJourneyDatasetVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsJourneyDatasetVisitor {
            type Value = ProductAnalyticsJourneyDataset;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut audience_filters: Option<crate::datadogV1_20270101::model::AudienceFilters> = None;
                let mut columns: Option<Vec<crate::datadogV1_20270101::model::DatasetColumn>> = None;
                let mut conversion_time_ms: Option<Option<f64>> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::ProductAnalyticsJourneyDatasetDataSource> = None;
                let mut entity: Option<crate::datadogV1_20270101::model::ProductAnalyticsJourneyDatasetEntity> = None;
                let mut expression: Option<String> = None;
                let mut name: Option<String> = None;
                let mut node_objects: Option<std::collections::BTreeMap<String, crate::datadogV1_20270101::model::ProductAnalyticsEventQuery>> = None;
                let mut search: Option<crate::datadogV1_20270101::model::ProductAnalyticsJourneyDatasetSearch> = None;
                let mut time_window: Option<crate::datadogV1_20270101::model::TimeWindow> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "audience_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            audience_filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "columns" => {
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "conversion_time_ms" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            conversion_time_ms = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::ProductAnalyticsJourneyDatasetDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "entity" => {
                            entity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _entity) = entity {
                                match _entity {
                                    crate::datadogV1_20270101::model::ProductAnalyticsJourneyDatasetEntity::UnparsedObject(_entity) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "expression" => {
                            expression = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "node_objects" => {
                            node_objects = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "search" => {
                            if v.is_null() {
                                continue;
                            }
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "time_window" => {
                            if v.is_null() {
                                continue;
                            }
                            time_window = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let columns = columns.ok_or_else(|| M::Error::missing_field("columns"))?;
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let entity = entity.ok_or_else(|| M::Error::missing_field("entity"))?;
                let expression = expression.ok_or_else(|| M::Error::missing_field("expression"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let node_objects = node_objects.ok_or_else(|| M::Error::missing_field("node_objects"))?;

                let content = ProductAnalyticsJourneyDataset {
                    audience_filters,
                    columns,
                    conversion_time_ms,
                    data_source,
                    entity,
                    expression,
                    name,
                    node_objects,
                    search,
                    time_window,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsJourneyDatasetVisitor)
    }
}

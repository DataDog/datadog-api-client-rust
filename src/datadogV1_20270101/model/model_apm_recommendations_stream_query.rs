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
pub struct ApmRecommendationsStreamQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "cross_org_uuids")]
    pub cross_org_uuids: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "custom_service_tags")]
    pub custom_service_tags: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::ApmRecommendationsStreamQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "env")]
    pub env: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Option<crate::datadogV1::model::ApmRecommendationsStreamQuery20270101GroupBy>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "priorities")]
    pub priorities: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query_string")]
    pub query_string: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "recommendation_types")]
    pub recommendation_types: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "services")]
    pub services: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1_20270101::model::ApmRecommendationsStreamQuery20270101Sort>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "statuses")]
    pub statuses: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "teams")]
    pub teams: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ApmRecommendationsStreamQuery {
    pub fn new(data_source: crate::datadogV1_20270101::model::ApmRecommendationsStreamQueryDataSource, query_string: String) -> ApmRecommendationsStreamQuery {
        ApmRecommendationsStreamQuery {
            cross_org_uuids: None,
            custom_service_tags: None,
            data_source,
            env: None,
            group_by: None,
            indexes: None,
            priorities: None,
            query_string,
            recommendation_types: None,
            services: None,
            sort: None,
            statuses: None,
            teams: None,
            _unparsed: false,
        }
    }
    
    pub fn cross_org_uuids(mut self, value: Vec<String>) -> Self {
        self.cross_org_uuids = Some(value);
        self
    }
    
    pub fn custom_service_tags(mut self, value: Vec<String>) -> Self {
        self.custom_service_tags = Some(value);
        self
    }
    
    pub fn env(mut self, value: String) -> Self {
        self.env = Some(value);
        self
    }
    
    pub fn group_by(mut self, value: crate::datadogV1::model::ApmRecommendationsStreamQuery20270101GroupBy) -> Self {
        self.group_by = Some(value);
        self
    }
    
    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }
    
    pub fn priorities(mut self, value: Vec<String>) -> Self {
        self.priorities = Some(value);
        self
    }
    
    pub fn recommendation_types(mut self, value: Vec<String>) -> Self {
        self.recommendation_types = Some(value);
        self
    }
    
    pub fn services(mut self, value: Vec<String>) -> Self {
        self.services = Some(value);
        self
    }
    
    pub fn sort(mut self, value: crate::datadogV1_20270101::model::ApmRecommendationsStreamQuery20270101Sort) -> Self {
        self.sort = Some(value);
        self
    }
    
    pub fn statuses(mut self, value: Vec<String>) -> Self {
        self.statuses = Some(value);
        self
    }
    
    pub fn teams(mut self, value: Vec<String>) -> Self {
        self.teams = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ApmRecommendationsStreamQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApmRecommendationsStreamQueryVisitor;
        impl<'a> Visitor<'a> for ApmRecommendationsStreamQueryVisitor {
            type Value = ApmRecommendationsStreamQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cross_org_uuids: Option<Vec<String>> = None;
                let mut custom_service_tags: Option<Vec<String>> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::ApmRecommendationsStreamQueryDataSource> = None;
                let mut env: Option<String> = None;
                let mut group_by: Option<crate::datadogV1::model::ApmRecommendationsStreamQuery20270101GroupBy> = None;
                let mut indexes: Option<Vec<String>> = None;
                let mut priorities: Option<Vec<String>> = None;
                let mut query_string: Option<String> = None;
                let mut recommendation_types: Option<Vec<String>> = None;
                let mut services: Option<Vec<String>> = None;
                let mut sort: Option<crate::datadogV1_20270101::model::ApmRecommendationsStreamQuery20270101Sort> = None;
                let mut statuses: Option<Vec<String>> = None;
                let mut teams: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cross_org_uuids" => {
                            if v.is_null() {
                                continue;
                            }
                            cross_org_uuids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "custom_service_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_service_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::ApmRecommendationsStreamQueryDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "env" => {
                            if v.is_null() {
                                continue;
                            }
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _group_by) = group_by {
                                match _group_by {
                                    crate::datadogV1::model::ApmRecommendationsStreamQuery20270101GroupBy::UnparsedObject(_group_by) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "indexes" => {
                            if v.is_null() {
                                continue;
                            }
                            indexes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "priorities" => {
                            if v.is_null() {
                                continue;
                            }
                            priorities = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "query_string" => {
                            query_string = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "recommendation_types" => {
                            if v.is_null() {
                                continue;
                            }
                            recommendation_types = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "services" => {
                            if v.is_null() {
                                continue;
                            }
                            services = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _sort) = sort {
                                match _sort {
                                    crate::datadogV1_20270101::model::ApmRecommendationsStreamQuery20270101Sort::UnparsedObject(_sort) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "statuses" => {
                            if v.is_null() {
                                continue;
                            }
                            statuses = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "teams" => {
                            if v.is_null() {
                                continue;
                            }
                            teams = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let query_string = query_string.ok_or_else(|| M::Error::missing_field("query_string"))?;

                let content = ApmRecommendationsStreamQuery {
                    cross_org_uuids,
                    custom_service_tags,
                    data_source,
                    env,
                    group_by,
                    indexes,
                    priorities,
                    query_string,
                    recommendation_types,
                    services,
                    sort,
                    statuses,
                    teams,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApmRecommendationsStreamQueryVisitor)
    }
}

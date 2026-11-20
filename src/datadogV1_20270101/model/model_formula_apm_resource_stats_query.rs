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
pub struct FormulaApmResourceStatsQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::FormulaApmResourceStatsQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "env")]
    pub env: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Vec<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "operation_name")]
    pub operation_name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "peer.service")]
    pub peer_service: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "primary_tag_name")]
    pub primary_tag_name: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "primary_tag_value")]
    pub primary_tag_value: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "resource_name")]
    pub resource_name: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "service")]
    pub service: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "stat")]
    pub stat: crate::datadogV1_20270101::model::ApmResourceStat,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FormulaApmResourceStatsQuery {
    pub fn new(data_source: crate::datadogV1_20270101::model::FormulaApmResourceStatsQueryDataSource, env: String, group_by: Vec<String>, name: String, operation_name: String, service: String, stat: crate::datadogV1_20270101::model::ApmResourceStat) -> FormulaApmResourceStatsQuery {
        FormulaApmResourceStatsQuery {
            data_source,
            env,
            group_by,
            name,
            operation_name,
            peer_service: None,
            primary_tag_name: None,
            primary_tag_value: None,
            resource_name: None,
            service,
            stat,
            version: None,
            _unparsed: false,
        }
    }
    
    pub fn peer_service(mut self, value: String) -> Self {
        self.peer_service = Some(value);
        self
    }
    
    pub fn primary_tag_name(mut self, value: String) -> Self {
        self.primary_tag_name = Some(value);
        self
    }
    
    pub fn primary_tag_value(mut self, value: String) -> Self {
        self.primary_tag_value = Some(value);
        self
    }
    
    pub fn resource_name(mut self, value: String) -> Self {
        self.resource_name = Some(value);
        self
    }
    
    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for FormulaApmResourceStatsQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaApmResourceStatsQueryVisitor;
        impl<'a> Visitor<'a> for FormulaApmResourceStatsQueryVisitor {
            type Value = FormulaApmResourceStatsQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<crate::datadogV1_20270101::model::FormulaApmResourceStatsQueryDataSource> = None;
                let mut env: Option<String> = None;
                let mut group_by: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut operation_name: Option<String> = None;
                let mut peer_service: Option<String> = None;
                let mut primary_tag_name: Option<String> = None;
                let mut primary_tag_value: Option<String> = None;
                let mut resource_name: Option<String> = None;
                let mut service: Option<String> = None;
                let mut stat: Option<crate::datadogV1_20270101::model::ApmResourceStat> = None;
                let mut version: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::FormulaApmResourceStatsQueryDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "env" => {
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "group_by" => {
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "operation_name" => {
                            operation_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "peer.service" => {
                            if v.is_null() {
                                continue;
                            }
                            peer_service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "primary_tag_name" => {
                            if v.is_null() {
                                continue;
                            }
                            primary_tag_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "primary_tag_value" => {
                            if v.is_null() {
                                continue;
                            }
                            primary_tag_value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "resource_name" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "service" => {
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "stat" => {
                            stat = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _stat) = stat {
                                match _stat {
                                    crate::datadogV1_20270101::model::ApmResourceStat::UnparsedObject(_stat) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let env = env.ok_or_else(|| M::Error::missing_field("env"))?;
                let group_by = group_by.ok_or_else(|| M::Error::missing_field("group_by"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let operation_name = operation_name.ok_or_else(|| M::Error::missing_field("operation_name"))?;
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;
                let stat = stat.ok_or_else(|| M::Error::missing_field("stat"))?;

                let content = FormulaApmResourceStatsQuery {
                    data_source,
                    env,
                    group_by,
                    name,
                    operation_name,
                    peer_service,
                    primary_tag_name,
                    primary_tag_value,
                    resource_name,
                    service,
                    stat,
                    version,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaApmResourceStatsQueryVisitor)
    }
}

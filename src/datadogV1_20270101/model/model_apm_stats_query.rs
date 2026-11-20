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
pub struct ApmStatsQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<crate::datadogV1_20270101::model::QueryTableColumnRequest>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "env")]
    pub env: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "primary_tag")]
    pub primary_tag: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "resource")]
    pub resource: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "row_type")]
    pub row_type: crate::datadogV1_20270101::model::ApmStatsQueryRowType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "service")]
    pub service: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ApmStatsQuery {
    pub fn new(env: String, name: String, primary_tag: String, row_type: crate::datadogV1_20270101::model::ApmStatsQueryRowType, service: String) -> ApmStatsQuery {
        ApmStatsQuery {
            columns: None,
            env,
            name,
            primary_tag,
            resource: None,
            row_type,
            service,
            _unparsed: false,
        }
    }
    
    pub fn columns(mut self, value: Vec<crate::datadogV1_20270101::model::QueryTableColumnRequest>) -> Self {
        self.columns = Some(value);
        self
    }
    
    pub fn resource(mut self, value: String) -> Self {
        self.resource = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ApmStatsQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApmStatsQueryVisitor;
        impl<'a> Visitor<'a> for ApmStatsQueryVisitor {
            type Value = ApmStatsQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<Vec<crate::datadogV1_20270101::model::QueryTableColumnRequest>> = None;
                let mut env: Option<String> = None;
                let mut name: Option<String> = None;
                let mut primary_tag: Option<String> = None;
                let mut resource: Option<String> = None;
                let mut row_type: Option<crate::datadogV1_20270101::model::ApmStatsQueryRowType> = None;
                let mut service: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns" => {
                            if v.is_null() {
                                continue;
                            }
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "env" => {
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "primary_tag" => {
                            primary_tag = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "resource" => {
                            if v.is_null() {
                                continue;
                            }
                            resource = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "row_type" => {
                            row_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _row_type) = row_type {
                                match _row_type {
                                    crate::datadogV1_20270101::model::ApmStatsQueryRowType::UnparsedObject(_row_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "service" => {
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let env = env.ok_or_else(|| M::Error::missing_field("env"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let primary_tag = primary_tag.ok_or_else(|| M::Error::missing_field("primary_tag"))?;
                let row_type = row_type.ok_or_else(|| M::Error::missing_field("row_type"))?;
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;

                let content = ApmStatsQuery {
                    columns,
                    env,
                    name,
                    primary_tag,
                    resource,
                    row_type,
                    service,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApmStatsQueryVisitor)
    }
}

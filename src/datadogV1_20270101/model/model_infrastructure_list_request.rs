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
pub struct InfrastructureListRequest {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "columns")]
    pub columns: Vec<crate::datadogV1_20270101::model::InfrastructureListRequestColumnsItems>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query")]
    pub query: crate::datadogV1_20270101::model::InfrastructureListQuery,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1_20270101::model::InfrastructureListRequestRequestType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1_20270101::model::ListStreamSort>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl InfrastructureListRequest {
    pub fn new(columns: Vec<crate::datadogV1_20270101::model::InfrastructureListRequestColumnsItems>, query: crate::datadogV1_20270101::model::InfrastructureListQuery, request_type: crate::datadogV1_20270101::model::InfrastructureListRequestRequestType) -> InfrastructureListRequest {
        InfrastructureListRequest {
            columns,
            query,
            request_type,
            sort: None,
            _unparsed: false,
        }
    }
    
    pub fn sort(mut self, value: crate::datadogV1_20270101::model::ListStreamSort) -> Self {
        self.sort = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for InfrastructureListRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct InfrastructureListRequestVisitor;
        impl<'a> Visitor<'a> for InfrastructureListRequestVisitor {
            type Value = InfrastructureListRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<Vec<crate::datadogV1_20270101::model::InfrastructureListRequestColumnsItems>> = None;
                let mut query: Option<crate::datadogV1_20270101::model::InfrastructureListQuery> = None;
                let mut request_type: Option<crate::datadogV1_20270101::model::InfrastructureListRequestRequestType> = None;
                let mut sort: Option<crate::datadogV1_20270101::model::ListStreamSort> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns" => {
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "request_type" => {
                            request_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV1_20270101::model::InfrastructureListRequestRequestType::UnparsedObject(_request_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let columns = columns.ok_or_else(|| M::Error::missing_field("columns"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let request_type = request_type.ok_or_else(|| M::Error::missing_field("request_type"))?;

                let content = InfrastructureListRequest {
                    columns,
                    query,
                    request_type,
                    sort,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(InfrastructureListRequestVisitor)
    }
}

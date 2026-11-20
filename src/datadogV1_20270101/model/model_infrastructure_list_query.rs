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
pub struct InfrastructureListQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::InfrastructureListQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query_string")]
    pub query_string: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "resource_type")]
    pub resource_type: crate::datadogV1_20270101::model::ContainerView,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl InfrastructureListQuery {
    pub fn new(data_source: crate::datadogV1_20270101::model::InfrastructureListQueryDataSource, query_string: String, resource_type: crate::datadogV1_20270101::model::ContainerView) -> InfrastructureListQuery {
        InfrastructureListQuery {
            data_source,
            query_string,
            resource_type,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for InfrastructureListQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct InfrastructureListQueryVisitor;
        impl<'a> Visitor<'a> for InfrastructureListQueryVisitor {
            type Value = InfrastructureListQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<crate::datadogV1_20270101::model::InfrastructureListQueryDataSource> = None;
                let mut query_string: Option<String> = None;
                let mut resource_type: Option<crate::datadogV1_20270101::model::ContainerView> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::InfrastructureListQueryDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "query_string" => {
                            query_string = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "resource_type" => {
                            resource_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _resource_type) = resource_type {
                                match _resource_type {
                                    crate::datadogV1_20270101::model::ContainerView::UnparsedObject(_resource_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let query_string = query_string.ok_or_else(|| M::Error::missing_field("query_string"))?;
                let resource_type = resource_type.ok_or_else(|| M::Error::missing_field("resource_type"))?;

                let content = InfrastructureListQuery {
                    data_source,
                    query_string,
                    resource_type,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(InfrastructureListQueryVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query to the service map topology data source.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TopologyQueryServiceMap {
    /// Name of the data source.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::TopologyQueryServiceMapDataSource,
    /// Your environment and primary tag (or * if enabled for your account).
    #[serde(rename = "filters")]
    pub filters: Vec<String>,
    /// A search string for filtering services. When set, this replaces the `service` field.
    #[serde(rename = "query_string")]
    pub query_string: Option<String>,
    /// (deprecated) Name of the service. Leave this empty and use query_string instead.
    #[serde(rename = "service")]
    pub service: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TopologyQueryServiceMap {
    pub fn new(
        data_source: crate::datadogV1::model::TopologyQueryServiceMapDataSource,
        filters: Vec<String>,
        service: String,
    ) -> TopologyQueryServiceMap {
        TopologyQueryServiceMap {
            data_source,
            filters,
            query_string: None,
            service,
            _unparsed: false,
        }
    }

    pub fn query_string(mut self, value: String) -> Self {
        self.query_string = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for TopologyQueryServiceMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TopologyQueryServiceMapVisitor;
        impl<'a> Visitor<'a> for TopologyQueryServiceMapVisitor {
            type Value = TopologyQueryServiceMap;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<
                    crate::datadogV1::model::TopologyQueryServiceMapDataSource,
                > = None;
                let mut filters: Option<Vec<String>> = None;
                let mut query_string: Option<String> = None;
                let mut service: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::TopologyQueryServiceMapDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "filters" => {
                            filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_string" => {
                            if v.is_null() {
                                continue;
                            }
                            query_string =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let filters = filters.ok_or_else(|| M::Error::missing_field("filters"))?;
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;

                let content = TopologyQueryServiceMap {
                    data_source,
                    filters,
                    query_string,
                    service,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TopologyQueryServiceMapVisitor)
    }
}

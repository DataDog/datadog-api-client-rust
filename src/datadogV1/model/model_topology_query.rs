// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query to service-based topology data sources like the service map or data streams.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TopologyQuery {
    /// Name of the data source
    #[serde(rename = "data_source")]
    pub data_source: Option<crate::datadogV1::model::TopologyQueryDataSource>,
    /// Your environment and primary tag (or * if enabled for your account).
    #[serde(rename = "filters")]
    pub filters: Option<Vec<String>>,
    /// Name of the service
    #[serde(rename = "service")]
    pub service: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TopologyQuery {
    pub fn new() -> TopologyQuery {
        TopologyQuery {
            data_source: None,
            filters: None,
            service: None,
            _unparsed: false,
        }
    }

    pub fn data_source(
        &mut self,
        value: crate::datadogV1::model::TopologyQueryDataSource,
    ) -> &mut Self {
        self.data_source = Some(value);
        self
    }

    pub fn filters(&mut self, value: Vec<String>) -> &mut Self {
        self.filters = Some(value);
        self
    }

    pub fn service(&mut self, value: String) -> &mut Self {
        self.service = Some(value);
        self
    }
}

impl Default for TopologyQuery {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TopologyQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TopologyQueryVisitor;
        impl<'a> Visitor<'a> for TopologyQueryVisitor {
            type Value = TopologyQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<crate::datadogV1::model::TopologyQueryDataSource> =
                    None;
                let mut filters: Option<Vec<String>> = None;
                let mut service: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            if v.is_null() {
                                continue;
                            }
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::TopologyQueryDataSource::UnparsedObject(_data_source) => {
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
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = TopologyQuery {
                    data_source,
                    filters,
                    service,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TopologyQueryVisitor)
    }
}

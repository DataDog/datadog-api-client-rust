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
pub struct TopologyQueryDataStreamsOrServiceMap {
    /// Name of the data source
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::TopologyQueryDataStreamsOrServiceMapDataSource,
    /// Your environment and primary tag (or * if enabled for your account).
    #[serde(rename = "filters")]
    pub filters: Vec<String>,
    /// A search string for filtering services, used in `data_streams` queries only. When set, this replaces the `service` field
    #[serde(rename = "query_string")]
    pub query_string: Option<String>,
    /// Name of the service
    #[serde(rename = "service")]
    pub service: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TopologyQueryDataStreamsOrServiceMap {
    pub fn new(
        data_source: crate::datadogV1::model::TopologyQueryDataStreamsOrServiceMapDataSource,
        filters: Vec<String>,
    ) -> TopologyQueryDataStreamsOrServiceMap {
        TopologyQueryDataStreamsOrServiceMap {
            data_source,
            filters,
            query_string: None,
            service: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn query_string(mut self, value: String) -> Self {
        self.query_string = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for TopologyQueryDataStreamsOrServiceMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TopologyQueryDataStreamsOrServiceMapVisitor;
        impl<'a> Visitor<'a> for TopologyQueryDataStreamsOrServiceMapVisitor {
            type Value = TopologyQueryDataStreamsOrServiceMap;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<
                    crate::datadogV1::model::TopologyQueryDataStreamsOrServiceMapDataSource,
                > = None;
                let mut filters: Option<Vec<String>> = None;
                let mut query_string: Option<String> = None;
                let mut service: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::TopologyQueryDataStreamsOrServiceMapDataSource::UnparsedObject(_data_source) => {
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
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let filters = filters.ok_or_else(|| M::Error::missing_field("filters"))?;

                let content = TopologyQueryDataStreamsOrServiceMap {
                    data_source,
                    filters,
                    query_string,
                    service,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TopologyQueryDataStreamsOrServiceMapVisitor)
    }
}

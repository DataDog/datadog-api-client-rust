// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Updated funnel widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FunnelQuery {
    /// Source from which to query items to display in the funnel.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::FunnelSource,
    /// The widget query.
    #[serde(rename = "query_string")]
    pub query_string: String,
    /// List of funnel steps.
    #[serde(rename = "steps")]
    pub steps: Vec<crate::datadogV1::model::FunnelStep>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FunnelQuery {
    pub fn new(
        data_source: crate::datadogV1::model::FunnelSource,
        query_string: String,
        steps: Vec<crate::datadogV1::model::FunnelStep>,
    ) -> FunnelQuery {
        FunnelQuery {
            data_source,
            query_string,
            steps,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for FunnelQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FunnelQueryVisitor;
        impl<'a> Visitor<'a> for FunnelQueryVisitor {
            type Value = FunnelQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<crate::datadogV1::model::FunnelSource> = None;
                let mut query_string: Option<String> = None;
                let mut steps: Option<Vec<crate::datadogV1::model::FunnelStep>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::FunnelSource::UnparsedObject(
                                        _data_source,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "query_string" => {
                            query_string =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "steps" => {
                            steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let query_string =
                    query_string.ok_or_else(|| M::Error::missing_field("query_string"))?;
                let steps = steps.ok_or_else(|| M::Error::missing_field("steps"))?;

                let content = FunnelQuery {
                    data_source,
                    query_string,
                    steps,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FunnelQueryVisitor)
    }
}

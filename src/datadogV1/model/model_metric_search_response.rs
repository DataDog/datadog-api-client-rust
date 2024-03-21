// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the list of metrics matching the search query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricSearchResponse {
    /// Search result.
    #[serde(rename = "results")]
    pub results: Option<crate::datadogV1::model::MetricSearchResponseResults>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricSearchResponse {
    pub fn new() -> MetricSearchResponse {
        MetricSearchResponse {
            results: None,
            _unparsed: false,
        }
    }

    pub fn results(mut self, value: crate::datadogV1::model::MetricSearchResponseResults) -> Self {
        self.results = Some(value);
        self
    }
}

impl Default for MetricSearchResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricSearchResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricSearchResponseVisitor;
        impl<'a> Visitor<'a> for MetricSearchResponseVisitor {
            type Value = MetricSearchResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut results: Option<crate::datadogV1::model::MetricSearchResponseResults> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "results" => {
                            if v.is_null() {
                                continue;
                            }
                            results = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MetricSearchResponse { results, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricSearchResponseVisitor)
    }
}

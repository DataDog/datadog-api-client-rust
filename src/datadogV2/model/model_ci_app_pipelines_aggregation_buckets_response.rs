// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The query results.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CIAppPipelinesAggregationBucketsResponse {
    /// The list of matching buckets, one item per bucket.
    #[serde(rename = "buckets")]
    pub buckets: Option<Vec<crate::datadogV2::model::CIAppPipelinesBucketResponse>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CIAppPipelinesAggregationBucketsResponse {
    pub fn new() -> CIAppPipelinesAggregationBucketsResponse {
        CIAppPipelinesAggregationBucketsResponse {
            buckets: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn buckets(
        mut self,
        value: Vec<crate::datadogV2::model::CIAppPipelinesBucketResponse>,
    ) -> Self {
        self.buckets = Some(value);
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

impl Default for CIAppPipelinesAggregationBucketsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CIAppPipelinesAggregationBucketsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CIAppPipelinesAggregationBucketsResponseVisitor;
        impl<'a> Visitor<'a> for CIAppPipelinesAggregationBucketsResponseVisitor {
            type Value = CIAppPipelinesAggregationBucketsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut buckets: Option<
                    Vec<crate::datadogV2::model::CIAppPipelinesBucketResponse>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "buckets" => {
                            if v.is_null() {
                                continue;
                            }
                            buckets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CIAppPipelinesAggregationBucketsResponse {
                    buckets,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CIAppPipelinesAggregationBucketsResponseVisitor)
    }
}

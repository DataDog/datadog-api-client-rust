// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Bucket values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CIAppPipelinesBucketResponse {
    /// The key-value pairs for each group-by.
    #[serde(rename = "by")]
    pub by: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// A map of the metric name to value for regular compute, or a list of values for a timeseries.
    #[serde(rename = "computes")]
    pub computes: Option<
        std::collections::BTreeMap<String, crate::datadogV2::model::CIAppAggregateBucketValue>,
    >,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CIAppPipelinesBucketResponse {
    pub fn new() -> CIAppPipelinesBucketResponse {
        CIAppPipelinesBucketResponse {
            by: None,
            computes: None,
            _unparsed: false,
        }
    }

    pub fn by(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.by = Some(value);
        self
    }

    pub fn computes(
        mut self,
        value: std::collections::BTreeMap<
            String,
            crate::datadogV2::model::CIAppAggregateBucketValue,
        >,
    ) -> Self {
        self.computes = Some(value);
        self
    }
}

impl Default for CIAppPipelinesBucketResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CIAppPipelinesBucketResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CIAppPipelinesBucketResponseVisitor;
        impl<'a> Visitor<'a> for CIAppPipelinesBucketResponseVisitor {
            type Value = CIAppPipelinesBucketResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut by: Option<std::collections::BTreeMap<String, serde_json::Value>> = None;
                let mut computes: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::datadogV2::model::CIAppAggregateBucketValue,
                    >,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "by" => {
                            if v.is_null() {
                                continue;
                            }
                            by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "computes" => {
                            if v.is_null() {
                                continue;
                            }
                            computes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = CIAppPipelinesBucketResponse {
                    by,
                    computes,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CIAppPipelinesBucketResponseVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Bucket values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CIAppTestsBucketResponse {
    /// The key-value pairs for each group-by.
    #[serde(rename = "by")]
    pub by: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// A map of the metric name to value for regular compute, or a list of values for a timeseries.
    #[serde(rename = "computes")]
    pub computes: Option<
        std::collections::BTreeMap<String, crate::datadogV2::model::CIAppAggregateBucketValue>,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CIAppTestsBucketResponse {
    pub fn new() -> CIAppTestsBucketResponse {
        CIAppTestsBucketResponse {
            by: None,
            computes: None,
            additional_properties: std::collections::BTreeMap::new(),
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for CIAppTestsBucketResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CIAppTestsBucketResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CIAppTestsBucketResponseVisitor;
        impl<'a> Visitor<'a> for CIAppTestsBucketResponseVisitor {
            type Value = CIAppTestsBucketResponse;

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
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CIAppTestsBucketResponse {
                    by,
                    computes,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CIAppTestsBucketResponseVisitor)
    }
}

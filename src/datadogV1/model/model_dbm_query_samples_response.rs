// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response object with DBM query sample events matching the request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DbmQuerySamplesResponse {
    /// Elapsed time of the request in milliseconds.
    #[serde(rename = "elapsed")]
    pub elapsed: Option<i64>,
    /// Total number of events matching the request.
    #[serde(rename = "hitCount")]
    pub hit_count: Option<i64>,
    /// Unique identifier for this request.
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    /// Result data container for a DBM query response.
    #[serde(rename = "result")]
    pub result: Option<crate::datadogV1::model::DbmResponseResult>,
    /// Status of the response.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Type of the response object.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DbmQuerySamplesResponse {
    pub fn new() -> DbmQuerySamplesResponse {
        DbmQuerySamplesResponse {
            elapsed: None,
            hit_count: None,
            request_id: None,
            result: None,
            status: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn elapsed(mut self, value: i64) -> Self {
        self.elapsed = Some(value);
        self
    }

    pub fn hit_count(mut self, value: i64) -> Self {
        self.hit_count = Some(value);
        self
    }

    pub fn request_id(mut self, value: String) -> Self {
        self.request_id = Some(value);
        self
    }

    pub fn result(mut self, value: crate::datadogV1::model::DbmResponseResult) -> Self {
        self.result = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
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

impl Default for DbmQuerySamplesResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DbmQuerySamplesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DbmQuerySamplesResponseVisitor;
        impl<'a> Visitor<'a> for DbmQuerySamplesResponseVisitor {
            type Value = DbmQuerySamplesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut elapsed: Option<i64> = None;
                let mut hit_count: Option<i64> = None;
                let mut request_id: Option<String> = None;
                let mut result: Option<crate::datadogV1::model::DbmResponseResult> = None;
                let mut status: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "elapsed" => {
                            if v.is_null() {
                                continue;
                            }
                            elapsed = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hitCount" => {
                            if v.is_null() {
                                continue;
                            }
                            hit_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requestId" => {
                            if v.is_null() {
                                continue;
                            }
                            request_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "result" => {
                            if v.is_null() {
                                continue;
                            }
                            result = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DbmQuerySamplesResponse {
                    elapsed,
                    hit_count,
                    request_id,
                    result,
                    status,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DbmQuerySamplesResponseVisitor)
    }
}

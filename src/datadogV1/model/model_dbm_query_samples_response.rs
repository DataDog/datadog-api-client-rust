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
    /// Array of DBM query sample events matching the request.
    #[serde(rename = "logs")]
    pub logs: Option<Vec<crate::datadogV1::model::Log>>,
    /// Hash identifier of the next event to return in the list.
    #[serde(
        rename = "nextLogId",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub next_log_id: Option<Option<String>>,
    /// Status of the response.
    #[serde(rename = "status")]
    pub status: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DbmQuerySamplesResponse {
    pub fn new() -> DbmQuerySamplesResponse {
        DbmQuerySamplesResponse {
            logs: None,
            next_log_id: None,
            status: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn logs(mut self, value: Vec<crate::datadogV1::model::Log>) -> Self {
        self.logs = Some(value);
        self
    }

    pub fn next_log_id(mut self, value: Option<String>) -> Self {
        self.next_log_id = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
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
                let mut logs: Option<Vec<crate::datadogV1::model::Log>> = None;
                let mut next_log_id: Option<Option<String>> = None;
                let mut status: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "logs" => {
                            if v.is_null() {
                                continue;
                            }
                            logs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "nextLogId" => {
                            next_log_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DbmQuerySamplesResponse {
                    logs,
                    next_log_id,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DbmQuerySamplesResponseVisitor)
    }
}

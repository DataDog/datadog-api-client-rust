// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A service level objective history response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOHistoryResponse {
    /// An array of service level objective objects.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV1::model::SLOHistoryResponseData>,
    /// A list of errors while querying the history data for the service level objective.
    #[serde(rename = "errors", default, with = "::serde_with::rust::double_option")]
    pub errors: Option<Option<Vec<crate::datadogV1::model::SLOHistoryResponseError>>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOHistoryResponse {
    pub fn new() -> SLOHistoryResponse {
        SLOHistoryResponse {
            data: None,
            errors: None,
            _unparsed: false,
        }
    }

    pub fn data(mut self, value: crate::datadogV1::model::SLOHistoryResponseData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn errors(
        mut self,
        value: Option<Vec<crate::datadogV1::model::SLOHistoryResponseError>>,
    ) -> Self {
        self.errors = Some(value);
        self
    }
}

impl Default for SLOHistoryResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SLOHistoryResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOHistoryResponseVisitor;
        impl<'a> Visitor<'a> for SLOHistoryResponseVisitor {
            type Value = SLOHistoryResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV1::model::SLOHistoryResponseData> = None;
                let mut errors: Option<
                    Option<Vec<crate::datadogV1::model::SLOHistoryResponseError>>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "errors" => {
                            errors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SLOHistoryResponse {
                    data,
                    errors,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOHistoryResponseVisitor)
    }
}

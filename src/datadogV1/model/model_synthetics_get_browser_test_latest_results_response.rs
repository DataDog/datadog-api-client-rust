// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object with the latest Synthetic browser test run.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsGetBrowserTestLatestResultsResponse {
    /// Timestamp of the latest browser test run.
    #[serde(rename = "last_timestamp_fetched")]
    pub last_timestamp_fetched: Option<i64>,
    /// Result of the latest browser test run.
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::datadogV1::model::SyntheticsBrowserTestResultShort>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsGetBrowserTestLatestResultsResponse {
    pub fn new() -> SyntheticsGetBrowserTestLatestResultsResponse {
        SyntheticsGetBrowserTestLatestResultsResponse {
            last_timestamp_fetched: None,
            results: None,
            _unparsed: false,
        }
    }

    pub fn last_timestamp_fetched(mut self, value: i64) -> Self {
        self.last_timestamp_fetched = Some(value);
        self
    }

    pub fn results(
        mut self,
        value: Vec<crate::datadogV1::model::SyntheticsBrowserTestResultShort>,
    ) -> Self {
        self.results = Some(value);
        self
    }
}

impl Default for SyntheticsGetBrowserTestLatestResultsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsGetBrowserTestLatestResultsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsGetBrowserTestLatestResultsResponseVisitor;
        impl<'a> Visitor<'a> for SyntheticsGetBrowserTestLatestResultsResponseVisitor {
            type Value = SyntheticsGetBrowserTestLatestResultsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut last_timestamp_fetched: Option<i64> = None;
                let mut results: Option<
                    Vec<crate::datadogV1::model::SyntheticsBrowserTestResultShort>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "last_timestamp_fetched" => {
                            if v.is_null() {
                                continue;
                            }
                            last_timestamp_fetched =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "results" => {
                            if v.is_null() {
                                continue;
                            }
                            results = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsGetBrowserTestLatestResultsResponse {
                    last_timestamp_fetched,
                    results,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsGetBrowserTestLatestResultsResponseVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing information about the tests triggered.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTriggerCITestsResponse {
    /// The public ID of the batch triggered.
    #[serde(
        rename = "batch_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub batch_id: Option<Option<String>>,
    /// List of Synthetic locations.
    #[serde(rename = "locations")]
    pub locations: Option<Vec<crate::datadogV1::model::SyntheticsTriggerCITestLocation>>,
    /// Information about the tests runs.
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::datadogV1::model::SyntheticsTriggerCITestRunResult>>,
    /// The public IDs of the Synthetic test triggered.
    #[serde(rename = "triggered_check_ids")]
    pub triggered_check_ids: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTriggerCITestsResponse {
    pub fn new() -> SyntheticsTriggerCITestsResponse {
        SyntheticsTriggerCITestsResponse {
            batch_id: None,
            locations: None,
            results: None,
            triggered_check_ids: None,
            _unparsed: false,
        }
    }

    pub fn batch_id(mut self, value: Option<String>) -> Self {
        self.batch_id = Some(value);
        self
    }

    pub fn locations(
        mut self,
        value: Vec<crate::datadogV1::model::SyntheticsTriggerCITestLocation>,
    ) -> Self {
        self.locations = Some(value);
        self
    }

    pub fn results(
        mut self,
        value: Vec<crate::datadogV1::model::SyntheticsTriggerCITestRunResult>,
    ) -> Self {
        self.results = Some(value);
        self
    }

    pub fn triggered_check_ids(mut self, value: Vec<String>) -> Self {
        self.triggered_check_ids = Some(value);
        self
    }
}

impl Default for SyntheticsTriggerCITestsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTriggerCITestsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTriggerCITestsResponseVisitor;
        impl<'a> Visitor<'a> for SyntheticsTriggerCITestsResponseVisitor {
            type Value = SyntheticsTriggerCITestsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut batch_id: Option<Option<String>> = None;
                let mut locations: Option<
                    Vec<crate::datadogV1::model::SyntheticsTriggerCITestLocation>,
                > = None;
                let mut results: Option<
                    Vec<crate::datadogV1::model::SyntheticsTriggerCITestRunResult>,
                > = None;
                let mut triggered_check_ids: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "batch_id" => {
                            batch_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "locations" => {
                            if v.is_null() {
                                continue;
                            }
                            locations = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "results" => {
                            if v.is_null() {
                                continue;
                            }
                            results = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "triggered_check_ids" => {
                            if v.is_null() {
                                continue;
                            }
                            triggered_check_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsTriggerCITestsResponse {
                    batch_id,
                    locations,
                    results,
                    triggered_check_ids,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTriggerCITestsResponseVisitor)
    }
}

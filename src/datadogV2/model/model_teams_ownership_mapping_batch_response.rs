// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The response body for the bulk create and remove operation. On success, `atomic:results`
/// contains one entry per operation. Add results appear before remove results and may not match
/// request order. Correlate add results by their `type` and `id` rather than by array position.
/// On failure, no operations were applied and `errors` describes what went wrong.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamsOwnershipMappingBatchResponse {
    /// The result of each operation.
    /// Add operations are processed first, then remove operations, so results may not appear
    /// in the same order as the request. Present only on success.
    #[serde(rename = "atomic:results")]
    pub atomic_results: Option<Vec<crate::datadogV2::model::TeamsOwnershipMappingBatchResult>>,
    /// The validation or processing errors encountered. Present only when the request could not be completed.
    #[serde(rename = "errors")]
    pub errors: Option<Vec<crate::datadogV2::model::TeamsOwnershipMappingBatchError>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamsOwnershipMappingBatchResponse {
    pub fn new() -> TeamsOwnershipMappingBatchResponse {
        TeamsOwnershipMappingBatchResponse {
            atomic_results: None,
            errors: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn atomic_results(
        mut self,
        value: Vec<crate::datadogV2::model::TeamsOwnershipMappingBatchResult>,
    ) -> Self {
        self.atomic_results = Some(value);
        self
    }

    pub fn errors(
        mut self,
        value: Vec<crate::datadogV2::model::TeamsOwnershipMappingBatchError>,
    ) -> Self {
        self.errors = Some(value);
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

impl Default for TeamsOwnershipMappingBatchResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TeamsOwnershipMappingBatchResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamsOwnershipMappingBatchResponseVisitor;
        impl<'a> Visitor<'a> for TeamsOwnershipMappingBatchResponseVisitor {
            type Value = TeamsOwnershipMappingBatchResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut atomic_results: Option<
                    Vec<crate::datadogV2::model::TeamsOwnershipMappingBatchResult>,
                > = None;
                let mut errors: Option<
                    Vec<crate::datadogV2::model::TeamsOwnershipMappingBatchError>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "atomic:results" => {
                            if v.is_null() {
                                continue;
                            }
                            atomic_results =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "errors" => {
                            if v.is_null() {
                                continue;
                            }
                            errors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TeamsOwnershipMappingBatchResponse {
                    atomic_results,
                    errors,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamsOwnershipMappingBatchResponseVisitor)
    }
}

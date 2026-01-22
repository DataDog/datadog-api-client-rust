// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Result of updating a single flaky test state.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateFlakyTestsResponseResult {
    /// Error message if the update failed.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// The ID of the flaky test from the request. This is the same ID returned by the Search flaky tests endpoint and corresponds to the test_fingerprint_fqn field in test run events.
    #[serde(rename = "id")]
    pub id: String,
    /// `True` if the update was successful, `False` if there were any errors.
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpdateFlakyTestsResponseResult {
    pub fn new(id: String, success: bool) -> UpdateFlakyTestsResponseResult {
        UpdateFlakyTestsResponseResult {
            error: None,
            id,
            success,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn error(mut self, value: String) -> Self {
        self.error = Some(value);
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

impl<'de> Deserialize<'de> for UpdateFlakyTestsResponseResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateFlakyTestsResponseResultVisitor;
        impl<'a> Visitor<'a> for UpdateFlakyTestsResponseResultVisitor {
            type Value = UpdateFlakyTestsResponseResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error: Option<String> = None;
                let mut id: Option<String> = None;
                let mut success: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "error" => {
                            if v.is_null() {
                                continue;
                            }
                            error = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "success" => {
                            success = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let success = success.ok_or_else(|| M::Error::missing_field("success"))?;

                let content = UpdateFlakyTestsResponseResult {
                    error,
                    id,
                    success,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpdateFlakyTestsResponseResultVisitor)
    }
}

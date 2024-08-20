// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A JSON list of the ID or IDs of the Synthetic tests that you want
/// to delete.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsDeleteTestsPayload {
    /// Delete the Synthetic test even if it's referenced by other resources
    /// (for example, SLOs and composite monitors).
    #[serde(rename = "force_delete_dependencies")]
    pub force_delete_dependencies: Option<bool>,
    /// An array of Synthetic test IDs you want to delete.
    #[serde(rename = "public_ids")]
    pub public_ids: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsDeleteTestsPayload {
    pub fn new() -> SyntheticsDeleteTestsPayload {
        SyntheticsDeleteTestsPayload {
            force_delete_dependencies: None,
            public_ids: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn force_delete_dependencies(mut self, value: bool) -> Self {
        self.force_delete_dependencies = Some(value);
        self
    }

    pub fn public_ids(mut self, value: Vec<String>) -> Self {
        self.public_ids = Some(value);
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

impl Default for SyntheticsDeleteTestsPayload {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsDeleteTestsPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsDeleteTestsPayloadVisitor;
        impl<'a> Visitor<'a> for SyntheticsDeleteTestsPayloadVisitor {
            type Value = SyntheticsDeleteTestsPayload;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut force_delete_dependencies: Option<bool> = None;
                let mut public_ids: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "force_delete_dependencies" => {
                            if v.is_null() {
                                continue;
                            }
                            force_delete_dependencies =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_ids" => {
                            if v.is_null() {
                                continue;
                            }
                            public_ids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsDeleteTestsPayload {
                    force_delete_dependencies,
                    public_ids,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsDeleteTestsPayloadVisitor)
    }
}

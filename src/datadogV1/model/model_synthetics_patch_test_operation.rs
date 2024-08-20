// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single [JSON Patch](<https://jsonpatch.com>) operation to perform on the test
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsPatchTestOperation {
    /// The operation to perform
    #[serde(rename = "op")]
    pub op: Option<crate::datadogV1::model::SyntheticsPatchTestOperationName>,
    /// The path to the value to modify
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// A value to use in a [JSON Patch](<https://jsonpatch.com>) operation
    #[serde(rename = "value")]
    pub value: Option<serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsPatchTestOperation {
    pub fn new() -> SyntheticsPatchTestOperation {
        SyntheticsPatchTestOperation {
            op: None,
            path: None,
            value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn op(mut self, value: crate::datadogV1::model::SyntheticsPatchTestOperationName) -> Self {
        self.op = Some(value);
        self
    }

    pub fn path(mut self, value: String) -> Self {
        self.path = Some(value);
        self
    }

    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
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

impl Default for SyntheticsPatchTestOperation {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsPatchTestOperation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsPatchTestOperationVisitor;
        impl<'a> Visitor<'a> for SyntheticsPatchTestOperationVisitor {
            type Value = SyntheticsPatchTestOperation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut op: Option<crate::datadogV1::model::SyntheticsPatchTestOperationName> =
                    None;
                let mut path: Option<String> = None;
                let mut value: Option<serde_json::Value> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "op" => {
                            if v.is_null() {
                                continue;
                            }
                            op = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _op) = op {
                                match _op {
                                    crate::datadogV1::model::SyntheticsPatchTestOperationName::UnparsedObject(_op) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "path" => {
                            if v.is_null() {
                                continue;
                            }
                            path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsPatchTestOperation {
                    op,
                    path,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsPatchTestOperationVisitor)
    }
}

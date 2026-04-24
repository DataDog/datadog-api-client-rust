// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details about the failure of a Synthetic test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultFailure {
    /// Error code for the failure.
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// Internal error code used for debugging.
    #[serde(rename = "internal_code")]
    pub internal_code: Option<String>,
    /// Internal error message used for debugging.
    #[serde(rename = "internal_message")]
    pub internal_message: Option<String>,
    /// Error message for the failure.
    #[serde(rename = "message")]
    pub message: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultFailure {
    pub fn new() -> SyntheticsTestResultFailure {
        SyntheticsTestResultFailure {
            code: None,
            internal_code: None,
            internal_message: None,
            message: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn code(mut self, value: String) -> Self {
        self.code = Some(value);
        self
    }

    pub fn internal_code(mut self, value: String) -> Self {
        self.internal_code = Some(value);
        self
    }

    pub fn internal_message(mut self, value: String) -> Self {
        self.internal_message = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
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

impl Default for SyntheticsTestResultFailure {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultFailure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultFailureVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultFailureVisitor {
            type Value = SyntheticsTestResultFailure;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut code: Option<String> = None;
                let mut internal_code: Option<String> = None;
                let mut internal_message: Option<String> = None;
                let mut message: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "code" => {
                            if v.is_null() {
                                continue;
                            }
                            code = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "internal_code" => {
                            if v.is_null() {
                                continue;
                            }
                            internal_code =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "internal_message" => {
                            if v.is_null() {
                                continue;
                            }
                            internal_message =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultFailure {
                    code,
                    internal_code,
                    internal_message,
                    message,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultFailureVisitor)
    }
}

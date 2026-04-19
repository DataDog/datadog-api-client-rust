// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An individual assertion result from a Synthetic test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultAssertionResult {
    /// Actual value observed during the test. Its type depends on the assertion type.
    #[serde(rename = "actual")]
    pub actual: Option<serde_json::Value>,
    /// Error message if the assertion failed.
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
    /// Expected value for the assertion. Its type depends on the assertion type.
    #[serde(rename = "expected")]
    pub expected: Option<serde_json::Value>,
    /// Operator used for the assertion (for example, `is`, `contains`).
    #[serde(rename = "operator")]
    pub operator: Option<String>,
    /// Property targeted by the assertion, when applicable.
    #[serde(rename = "property")]
    pub property: Option<String>,
    /// Target value for the assertion. Its type depends on the assertion type.
    #[serde(rename = "target")]
    pub target: Option<serde_json::Value>,
    /// JSON path or XPath evaluated for the assertion.
    #[serde(rename = "target_path")]
    pub target_path: Option<String>,
    /// Operator used for the target path assertion.
    #[serde(rename = "target_path_operator")]
    pub target_path_operator: Option<String>,
    /// Type of the assertion (for example, `responseTime`, `statusCode`, `body`).
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Whether the assertion passed.
    #[serde(rename = "valid")]
    pub valid: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultAssertionResult {
    pub fn new() -> SyntheticsTestResultAssertionResult {
        SyntheticsTestResultAssertionResult {
            actual: None,
            error_message: None,
            expected: None,
            operator: None,
            property: None,
            target: None,
            target_path: None,
            target_path_operator: None,
            type_: None,
            valid: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn actual(mut self, value: serde_json::Value) -> Self {
        self.actual = Some(value);
        self
    }

    pub fn error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
        self
    }

    pub fn expected(mut self, value: serde_json::Value) -> Self {
        self.expected = Some(value);
        self
    }

    pub fn operator(mut self, value: String) -> Self {
        self.operator = Some(value);
        self
    }

    pub fn property(mut self, value: String) -> Self {
        self.property = Some(value);
        self
    }

    pub fn target(mut self, value: serde_json::Value) -> Self {
        self.target = Some(value);
        self
    }

    pub fn target_path(mut self, value: String) -> Self {
        self.target_path = Some(value);
        self
    }

    pub fn target_path_operator(mut self, value: String) -> Self {
        self.target_path_operator = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn valid(mut self, value: bool) -> Self {
        self.valid = Some(value);
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

impl Default for SyntheticsTestResultAssertionResult {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultAssertionResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultAssertionResultVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultAssertionResultVisitor {
            type Value = SyntheticsTestResultAssertionResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut actual: Option<serde_json::Value> = None;
                let mut error_message: Option<String> = None;
                let mut expected: Option<serde_json::Value> = None;
                let mut operator: Option<String> = None;
                let mut property: Option<String> = None;
                let mut target: Option<serde_json::Value> = None;
                let mut target_path: Option<String> = None;
                let mut target_path_operator: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut valid: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "actual" => {
                            if v.is_null() {
                                continue;
                            }
                            actual = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_message" => {
                            if v.is_null() {
                                continue;
                            }
                            error_message =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expected" => {
                            if v.is_null() {
                                continue;
                            }
                            expected = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operator" => {
                            if v.is_null() {
                                continue;
                            }
                            operator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "property" => {
                            if v.is_null() {
                                continue;
                            }
                            property = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target_path" => {
                            if v.is_null() {
                                continue;
                            }
                            target_path =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target_path_operator" => {
                            if v.is_null() {
                                continue;
                            }
                            target_path_operator =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "valid" => {
                            if v.is_null() {
                                continue;
                            }
                            valid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultAssertionResult {
                    actual,
                    error_message,
                    expected,
                    operator,
                    property,
                    target,
                    target_path,
                    target_path_operator,
                    type_,
                    valid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultAssertionResultVisitor)
    }
}

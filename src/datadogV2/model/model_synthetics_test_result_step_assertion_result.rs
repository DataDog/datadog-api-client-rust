// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Assertion result for a browser or mobile step.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultStepAssertionResult {
    /// Actual value observed during the step assertion. Its type depends on the check type.
    #[serde(rename = "actual")]
    pub actual: Option<serde_json::Value>,
    /// Type of the step assertion check.
    #[serde(rename = "check_type")]
    pub check_type: Option<String>,
    /// Expected value for the step assertion. Its type depends on the check type.
    #[serde(rename = "expected")]
    pub expected: Option<serde_json::Value>,
    /// Whether the assertion involves secure variables.
    #[serde(rename = "has_secure_variables")]
    pub has_secure_variables: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultStepAssertionResult {
    pub fn new() -> SyntheticsTestResultStepAssertionResult {
        SyntheticsTestResultStepAssertionResult {
            actual: None,
            check_type: None,
            expected: None,
            has_secure_variables: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn actual(mut self, value: serde_json::Value) -> Self {
        self.actual = Some(value);
        self
    }

    pub fn check_type(mut self, value: String) -> Self {
        self.check_type = Some(value);
        self
    }

    pub fn expected(mut self, value: serde_json::Value) -> Self {
        self.expected = Some(value);
        self
    }

    pub fn has_secure_variables(mut self, value: bool) -> Self {
        self.has_secure_variables = Some(value);
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

impl Default for SyntheticsTestResultStepAssertionResult {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultStepAssertionResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultStepAssertionResultVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultStepAssertionResultVisitor {
            type Value = SyntheticsTestResultStepAssertionResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut actual: Option<serde_json::Value> = None;
                let mut check_type: Option<String> = None;
                let mut expected: Option<serde_json::Value> = None;
                let mut has_secure_variables: Option<bool> = None;
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
                        "check_type" => {
                            if v.is_null() {
                                continue;
                            }
                            check_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expected" => {
                            if v.is_null() {
                                continue;
                            }
                            expected = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_secure_variables" => {
                            if v.is_null() {
                                continue;
                            }
                            has_secure_variables =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultStepAssertionResult {
                    actual,
                    check_type,
                    expected,
                    has_secure_variables,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultStepAssertionResultVisitor)
    }
}

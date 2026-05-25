// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The result of applying a single static analysis rule to the analyzed source code.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnalysisRuleResponse {
    /// A list of error messages encountered while executing the rule.
    #[serde(rename = "errors")]
    pub errors: Vec<String>,
    /// An error message if the rule execution failed, or null if execution succeeded.
    #[serialize_always]
    #[serde(rename = "execution_error")]
    pub execution_error: Option<String>,
    /// The time taken to execute the rule, in milliseconds.
    #[serde(rename = "execution_time_ms")]
    pub execution_time_ms: i64,
    /// The identifier of the rule that produced this response.
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// The raw output produced by the rule engine during execution.
    #[serde(rename = "output")]
    pub output: String,
    /// The list of violations found by this rule.
    #[serde(rename = "violations")]
    pub violations: Vec<crate::datadogV2::model::AnalysisViolation>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AnalysisRuleResponse {
    pub fn new(
        errors: Vec<String>,
        execution_error: Option<String>,
        execution_time_ms: i64,
        identifier: String,
        output: String,
        violations: Vec<crate::datadogV2::model::AnalysisViolation>,
    ) -> AnalysisRuleResponse {
        AnalysisRuleResponse {
            errors,
            execution_error,
            execution_time_ms,
            identifier,
            output,
            violations,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for AnalysisRuleResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnalysisRuleResponseVisitor;
        impl<'a> Visitor<'a> for AnalysisRuleResponseVisitor {
            type Value = AnalysisRuleResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut errors: Option<Vec<String>> = None;
                let mut execution_error: Option<Option<String>> = None;
                let mut execution_time_ms: Option<i64> = None;
                let mut identifier: Option<String> = None;
                let mut output: Option<String> = None;
                let mut violations: Option<Vec<crate::datadogV2::model::AnalysisViolation>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "errors" => {
                            errors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "execution_error" => {
                            execution_error =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "execution_time_ms" => {
                            execution_time_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "identifier" => {
                            identifier = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "output" => {
                            output = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "violations" => {
                            violations = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let errors = errors.ok_or_else(|| M::Error::missing_field("errors"))?;
                let execution_error =
                    execution_error.ok_or_else(|| M::Error::missing_field("execution_error"))?;
                let execution_time_ms = execution_time_ms
                    .ok_or_else(|| M::Error::missing_field("execution_time_ms"))?;
                let identifier = identifier.ok_or_else(|| M::Error::missing_field("identifier"))?;
                let output = output.ok_or_else(|| M::Error::missing_field("output"))?;
                let violations = violations.ok_or_else(|| M::Error::missing_field("violations"))?;

                let content = AnalysisRuleResponse {
                    errors,
                    execution_error,
                    execution_time_ms,
                    identifier,
                    output,
                    violations,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AnalysisRuleResponseVisitor)
    }
}

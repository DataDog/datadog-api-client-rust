// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A rule violation found in the analyzed source code.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnalysisViolation {
    /// The category of the violation.
    #[serde(rename = "category")]
    pub category: String,
    /// A position in source code, identified by line and column numbers.
    #[serde(rename = "end")]
    pub end: crate::datadogV2::model::AnalysisPosition,
    /// The list of suggested fixes for this violation.
    #[serde(rename = "fixes")]
    pub fixes: Vec<crate::datadogV2::model::AnalysisFix>,
    /// A human-readable description of the violation.
    #[serde(rename = "message")]
    pub message: String,
    /// The severity level of the violation.
    #[serde(rename = "severity")]
    pub severity: String,
    /// A position in source code, identified by line and column numbers.
    #[serde(rename = "start")]
    pub start: crate::datadogV2::model::AnalysisPosition,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AnalysisViolation {
    pub fn new(
        category: String,
        end: crate::datadogV2::model::AnalysisPosition,
        fixes: Vec<crate::datadogV2::model::AnalysisFix>,
        message: String,
        severity: String,
        start: crate::datadogV2::model::AnalysisPosition,
    ) -> AnalysisViolation {
        AnalysisViolation {
            category,
            end,
            fixes,
            message,
            severity,
            start,
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

impl<'de> Deserialize<'de> for AnalysisViolation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnalysisViolationVisitor;
        impl<'a> Visitor<'a> for AnalysisViolationVisitor {
            type Value = AnalysisViolation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<String> = None;
                let mut end: Option<crate::datadogV2::model::AnalysisPosition> = None;
                let mut fixes: Option<Vec<crate::datadogV2::model::AnalysisFix>> = None;
                let mut message: Option<String> = None;
                let mut severity: Option<String> = None;
                let mut start: Option<crate::datadogV2::model::AnalysisPosition> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end" => {
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fixes" => {
                            fixes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severity" => {
                            severity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let category = category.ok_or_else(|| M::Error::missing_field("category"))?;
                let end = end.ok_or_else(|| M::Error::missing_field("end"))?;
                let fixes = fixes.ok_or_else(|| M::Error::missing_field("fixes"))?;
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;
                let severity = severity.ok_or_else(|| M::Error::missing_field("severity"))?;
                let start = start.ok_or_else(|| M::Error::missing_field("start"))?;

                let content = AnalysisViolation {
                    category,
                    end,
                    fixes,
                    message,
                    severity,
                    start,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AnalysisViolationVisitor)
    }
}

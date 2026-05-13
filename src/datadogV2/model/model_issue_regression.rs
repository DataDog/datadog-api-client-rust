// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Regression information for an issue that was previously resolved and then reopened.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueRegression {
    /// Timestamp when the issue was reopened (regressed).
    #[serde(rename = "regressed_at")]
    pub regressed_at: chrono::DateTime<chrono::Utc>,
    /// Application version where the regression was observed.
    #[serde(rename = "regressed_at_version")]
    pub regressed_at_version: Option<String>,
    /// Timestamp when the issue was resolved before the regression.
    #[serde(rename = "resolved_at")]
    pub resolved_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssueRegression {
    pub fn new(
        regressed_at: chrono::DateTime<chrono::Utc>,
        resolved_at: chrono::DateTime<chrono::Utc>,
    ) -> IssueRegression {
        IssueRegression {
            regressed_at,
            regressed_at_version: None,
            resolved_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn regressed_at_version(mut self, value: String) -> Self {
        self.regressed_at_version = Some(value);
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

impl<'de> Deserialize<'de> for IssueRegression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueRegressionVisitor;
        impl<'a> Visitor<'a> for IssueRegressionVisitor {
            type Value = IssueRegression;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut regressed_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut regressed_at_version: Option<String> = None;
                let mut resolved_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "regressed_at" => {
                            regressed_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "regressed_at_version" => {
                            if v.is_null() {
                                continue;
                            }
                            regressed_at_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resolved_at" => {
                            resolved_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let regressed_at =
                    regressed_at.ok_or_else(|| M::Error::missing_field("regressed_at"))?;
                let resolved_at =
                    resolved_at.ok_or_else(|| M::Error::missing_field("resolved_at"))?;

                let content = IssueRegression {
                    regressed_at,
                    regressed_at_version,
                    resolved_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueRegressionVisitor)
    }
}

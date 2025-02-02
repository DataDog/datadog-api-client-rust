// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// CI/CD options for a Synthetic test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestCiOptions {
    /// Execution rule for a Synthetic test.
    #[serde(rename = "executionRule")]
    pub execution_rule: crate::datadogV1::model::SyntheticsTestExecutionRule,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestCiOptions {
    pub fn new(
        execution_rule: crate::datadogV1::model::SyntheticsTestExecutionRule,
    ) -> SyntheticsTestCiOptions {
        SyntheticsTestCiOptions {
            execution_rule,
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

impl<'de> Deserialize<'de> for SyntheticsTestCiOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestCiOptionsVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestCiOptionsVisitor {
            type Value = SyntheticsTestCiOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut execution_rule: Option<
                    crate::datadogV1::model::SyntheticsTestExecutionRule,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "executionRule" => {
                            execution_rule =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _execution_rule) = execution_rule {
                                match _execution_rule {
                                    crate::datadogV1::model::SyntheticsTestExecutionRule::UnparsedObject(_execution_rule) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let execution_rule =
                    execution_rule.ok_or_else(|| M::Error::missing_field("execution_rule"))?;

                let content = SyntheticsTestCiOptions {
                    execution_rule,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestCiOptionsVisitor)
    }
}

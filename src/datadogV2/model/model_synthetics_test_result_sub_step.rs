// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Information about a sub-step in a nested test execution.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultSubStep {
    /// Depth of the sub-step in the execution tree.
    #[serde(rename = "level")]
    pub level: Option<i64>,
    /// Reference to the parent step of a sub-step.
    #[serde(rename = "parent_step")]
    pub parent_step: Option<crate::datadogV2::model::SyntheticsTestResultParentStep>,
    /// Reference to the parent test of a sub-step.
    #[serde(rename = "parent_test")]
    pub parent_test: Option<crate::datadogV2::model::SyntheticsTestResultParentTest>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultSubStep {
    pub fn new() -> SyntheticsTestResultSubStep {
        SyntheticsTestResultSubStep {
            level: None,
            parent_step: None,
            parent_test: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn level(mut self, value: i64) -> Self {
        self.level = Some(value);
        self
    }

    pub fn parent_step(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultParentStep,
    ) -> Self {
        self.parent_step = Some(value);
        self
    }

    pub fn parent_test(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultParentTest,
    ) -> Self {
        self.parent_test = Some(value);
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

impl Default for SyntheticsTestResultSubStep {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultSubStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultSubStepVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultSubStepVisitor {
            type Value = SyntheticsTestResultSubStep;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut level: Option<i64> = None;
                let mut parent_step: Option<
                    crate::datadogV2::model::SyntheticsTestResultParentStep,
                > = None;
                let mut parent_test: Option<
                    crate::datadogV2::model::SyntheticsTestResultParentTest,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "level" => {
                            if v.is_null() {
                                continue;
                            }
                            level = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parent_step" => {
                            if v.is_null() {
                                continue;
                            }
                            parent_step =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parent_test" => {
                            if v.is_null() {
                                continue;
                            }
                            parent_test =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultSubStep {
                    level,
                    parent_step,
                    parent_test,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultSubStepVisitor)
    }
}

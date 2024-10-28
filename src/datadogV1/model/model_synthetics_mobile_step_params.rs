// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The parameters of a mobile step.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsMobileStepParams {
    /// Type of assertion to apply in an API test.
    #[serde(rename = "check")]
    pub check: Option<crate::datadogV1::model::SyntheticsCheckType>,
    /// Number of milliseconds to wait between inputs in a `typeText` step type.
    #[serde(rename = "delay")]
    pub delay: Option<i64>,
    /// The direction of the scroll for a `scrollToElement` step type.
    #[serde(rename = "direction")]
    pub direction: Option<crate::datadogV1::model::SyntheticsMobileStepParamsDirection>,
    /// Information about the element used for a step.
    #[serde(rename = "element")]
    pub element: Option<crate::datadogV1::model::SyntheticsMobileStepParamsElement>,
    /// Boolean to change the state of the wifi for a `toggleWiFi` step type.
    #[serde(rename = "enable")]
    pub enable: Option<bool>,
    /// Maximum number of scrolls to do for a `scrollToElement` step type.
    #[serde(rename = "maxScrolls")]
    pub max_scrolls: Option<i64>,
    /// List of positions for the `flick` step type. The maximum is 10 flicks per step
    #[serde(rename = "positions")]
    pub positions: Option<Vec<crate::datadogV1::model::SyntheticsMobileStepParamsPositionsItems>>,
    /// Public ID of the test to be played as part of a `playSubTest` step type.
    #[serde(rename = "subtestPublicId")]
    pub subtest_public_id: Option<String>,
    /// Values used in the step. Used in multiple step types.
    #[serde(rename = "value")]
    pub value: Option<String>,
    /// Variable object for `extractVariable` step type.
    #[serde(rename = "variable")]
    pub variable: Option<crate::datadogV1::model::SyntheticsMobileStepParamsVariable>,
    /// Boolean to indicate if `Enter` should be pressed at the end of the `typeText` step type.
    #[serde(rename = "withEnter")]
    pub with_enter: Option<bool>,
    /// Amount to scroll by on the `x` axis for a `scroll` step type.
    #[serde(rename = "x")]
    pub x: Option<i64>,
    /// Amount to scroll by on the `y` axis for a `scroll` step type.
    #[serde(rename = "y")]
    pub y: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsMobileStepParams {
    pub fn new() -> SyntheticsMobileStepParams {
        SyntheticsMobileStepParams {
            check: None,
            delay: None,
            direction: None,
            element: None,
            enable: None,
            max_scrolls: None,
            positions: None,
            subtest_public_id: None,
            value: None,
            variable: None,
            with_enter: None,
            x: None,
            y: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn check(mut self, value: crate::datadogV1::model::SyntheticsCheckType) -> Self {
        self.check = Some(value);
        self
    }

    pub fn delay(mut self, value: i64) -> Self {
        self.delay = Some(value);
        self
    }

    pub fn direction(
        mut self,
        value: crate::datadogV1::model::SyntheticsMobileStepParamsDirection,
    ) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn element(
        mut self,
        value: crate::datadogV1::model::SyntheticsMobileStepParamsElement,
    ) -> Self {
        self.element = Some(value);
        self
    }

    pub fn enable(mut self, value: bool) -> Self {
        self.enable = Some(value);
        self
    }

    pub fn max_scrolls(mut self, value: i64) -> Self {
        self.max_scrolls = Some(value);
        self
    }

    pub fn positions(
        mut self,
        value: Vec<crate::datadogV1::model::SyntheticsMobileStepParamsPositionsItems>,
    ) -> Self {
        self.positions = Some(value);
        self
    }

    pub fn subtest_public_id(mut self, value: String) -> Self {
        self.subtest_public_id = Some(value);
        self
    }

    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }

    pub fn variable(
        mut self,
        value: crate::datadogV1::model::SyntheticsMobileStepParamsVariable,
    ) -> Self {
        self.variable = Some(value);
        self
    }

    pub fn with_enter(mut self, value: bool) -> Self {
        self.with_enter = Some(value);
        self
    }

    pub fn x(mut self, value: i64) -> Self {
        self.x = Some(value);
        self
    }

    pub fn y(mut self, value: i64) -> Self {
        self.y = Some(value);
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

impl Default for SyntheticsMobileStepParams {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsMobileStepParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsMobileStepParamsVisitor;
        impl<'a> Visitor<'a> for SyntheticsMobileStepParamsVisitor {
            type Value = SyntheticsMobileStepParams;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut check: Option<crate::datadogV1::model::SyntheticsCheckType> = None;
                let mut delay: Option<i64> = None;
                let mut direction: Option<
                    crate::datadogV1::model::SyntheticsMobileStepParamsDirection,
                > = None;
                let mut element: Option<
                    crate::datadogV1::model::SyntheticsMobileStepParamsElement,
                > = None;
                let mut enable: Option<bool> = None;
                let mut max_scrolls: Option<i64> = None;
                let mut positions: Option<
                    Vec<crate::datadogV1::model::SyntheticsMobileStepParamsPositionsItems>,
                > = None;
                let mut subtest_public_id: Option<String> = None;
                let mut value: Option<String> = None;
                let mut variable: Option<
                    crate::datadogV1::model::SyntheticsMobileStepParamsVariable,
                > = None;
                let mut with_enter: Option<bool> = None;
                let mut x: Option<i64> = None;
                let mut y: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "check" => {
                            if v.is_null() {
                                continue;
                            }
                            check = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _check) = check {
                                match _check {
                                    crate::datadogV1::model::SyntheticsCheckType::UnparsedObject(_check) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "delay" => {
                            if v.is_null() {
                                continue;
                            }
                            delay = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "direction" => {
                            if v.is_null() {
                                continue;
                            }
                            direction = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _direction) = direction {
                                match _direction {
                                    crate::datadogV1::model::SyntheticsMobileStepParamsDirection::UnparsedObject(_direction) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "element" => {
                            if v.is_null() {
                                continue;
                            }
                            element = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enable" => {
                            if v.is_null() {
                                continue;
                            }
                            enable = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "maxScrolls" => {
                            if v.is_null() {
                                continue;
                            }
                            max_scrolls =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "positions" => {
                            if v.is_null() {
                                continue;
                            }
                            positions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subtestPublicId" => {
                            if v.is_null() {
                                continue;
                            }
                            subtest_public_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variable" => {
                            if v.is_null() {
                                continue;
                            }
                            variable = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "withEnter" => {
                            if v.is_null() {
                                continue;
                            }
                            with_enter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "x" => {
                            if v.is_null() {
                                continue;
                            }
                            x = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "y" => {
                            if v.is_null() {
                                continue;
                            }
                            y = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsMobileStepParams {
                    check,
                    delay,
                    direction,
                    element,
                    enable,
                    max_scrolls,
                    positions,
                    subtest_public_id,
                    value,
                    variable,
                    with_enter,
                    x,
                    y,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsMobileStepParamsVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The steps used in a Synthetic mobile test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsMobileStep {
    /// A boolean set to allow this step to fail.
    #[serde(rename = "allowFailure")]
    pub allow_failure: Option<bool>,
    /// A boolean set to determine if the step has a new step element.
    #[serde(rename = "hasNewStepElement")]
    pub has_new_step_element: Option<bool>,
    /// A boolean to use in addition to `allowFailure` to determine if the test should be marked as failed when the step fails.
    #[serde(rename = "isCritical")]
    pub is_critical: Option<bool>,
    /// The name of the step.
    #[serde(rename = "name")]
    pub name: String,
    /// A boolean set to not take a screenshot for the step.
    #[serde(rename = "noScreenshot")]
    pub no_screenshot: Option<bool>,
    /// The parameters of a mobile step.
    #[serde(rename = "params")]
    pub params: crate::datadogV1::model::SyntheticsMobileStepParams,
    /// The public ID of the step.
    #[serde(rename = "publicId")]
    pub public_id: Option<String>,
    /// The time before declaring a step failed.
    #[serde(rename = "timeout")]
    pub timeout: Option<i64>,
    /// Step type used in your mobile Synthetic test.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsMobileStepType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsMobileStep {
    pub fn new(
        name: String,
        params: crate::datadogV1::model::SyntheticsMobileStepParams,
        type_: crate::datadogV1::model::SyntheticsMobileStepType,
    ) -> SyntheticsMobileStep {
        SyntheticsMobileStep {
            allow_failure: None,
            has_new_step_element: None,
            is_critical: None,
            name,
            no_screenshot: None,
            params,
            public_id: None,
            timeout: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allow_failure(mut self, value: bool) -> Self {
        self.allow_failure = Some(value);
        self
    }

    pub fn has_new_step_element(mut self, value: bool) -> Self {
        self.has_new_step_element = Some(value);
        self
    }

    pub fn is_critical(mut self, value: bool) -> Self {
        self.is_critical = Some(value);
        self
    }

    pub fn no_screenshot(mut self, value: bool) -> Self {
        self.no_screenshot = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }

    pub fn timeout(mut self, value: i64) -> Self {
        self.timeout = Some(value);
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

impl<'de> Deserialize<'de> for SyntheticsMobileStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsMobileStepVisitor;
        impl<'a> Visitor<'a> for SyntheticsMobileStepVisitor {
            type Value = SyntheticsMobileStep;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allow_failure: Option<bool> = None;
                let mut has_new_step_element: Option<bool> = None;
                let mut is_critical: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut no_screenshot: Option<bool> = None;
                let mut params: Option<crate::datadogV1::model::SyntheticsMobileStepParams> = None;
                let mut public_id: Option<String> = None;
                let mut timeout: Option<i64> = None;
                let mut type_: Option<crate::datadogV1::model::SyntheticsMobileStepType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allowFailure" => {
                            if v.is_null() {
                                continue;
                            }
                            allow_failure =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hasNewStepElement" => {
                            if v.is_null() {
                                continue;
                            }
                            has_new_step_element =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "isCritical" => {
                            if v.is_null() {
                                continue;
                            }
                            is_critical =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "noScreenshot" => {
                            if v.is_null() {
                                continue;
                            }
                            no_screenshot =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "params" => {
                            params = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "publicId" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeout" => {
                            if v.is_null() {
                                continue;
                            }
                            timeout = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsMobileStepType::UnparsedObject(_type_) => {
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
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let params = params.ok_or_else(|| M::Error::missing_field("params"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SyntheticsMobileStep {
                    allow_failure,
                    has_new_step_element,
                    is_critical,
                    name,
                    no_screenshot,
                    params,
                    public_id,
                    timeout,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsMobileStepVisitor)
    }
}

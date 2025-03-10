// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Test step used in a Synthetic multi-step API test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsAPITestStep {
    /// Determines whether or not to continue with test if this step fails.
    #[serde(rename = "allowFailure")]
    pub allow_failure: Option<bool>,
    /// Array of assertions used for the test.
    #[serde(rename = "assertions")]
    pub assertions: Vec<crate::datadogV1::model::SyntheticsAssertion>,
    /// Determines whether or not to exit the test if the step succeeds.
    #[serde(rename = "exitIfSucceed")]
    pub exit_if_succeed: Option<bool>,
    /// Array of values to parse and save as variables from the response.
    #[serde(rename = "extractedValues")]
    pub extracted_values: Option<Vec<crate::datadogV1::model::SyntheticsParsingOptions>>,
    /// Generate variables using JavaScript.
    #[serde(rename = "extractedValuesFromScript")]
    pub extracted_values_from_script: Option<String>,
    /// Determines whether or not to consider the entire test as failed if this step fails.
    /// Can be used only if `allowFailure` is `true`.
    #[serde(rename = "isCritical")]
    pub is_critical: Option<bool>,
    /// The name of the step.
    #[serde(rename = "name")]
    pub name: String,
    /// Object describing the Synthetic test request.
    #[serde(rename = "request")]
    pub request: crate::datadogV1::model::SyntheticsTestRequest,
    /// Object describing the retry strategy to apply to a Synthetic test.
    #[serde(rename = "retry")]
    pub retry: Option<crate::datadogV1::model::SyntheticsTestOptionsRetry>,
    /// The subtype of the Synthetic multi-step API test step.
    #[serde(rename = "subtype")]
    pub subtype: crate::datadogV1::model::SyntheticsAPITestStepSubtype,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsAPITestStep {
    pub fn new(
        assertions: Vec<crate::datadogV1::model::SyntheticsAssertion>,
        name: String,
        request: crate::datadogV1::model::SyntheticsTestRequest,
        subtype: crate::datadogV1::model::SyntheticsAPITestStepSubtype,
    ) -> SyntheticsAPITestStep {
        SyntheticsAPITestStep {
            allow_failure: None,
            assertions,
            exit_if_succeed: None,
            extracted_values: None,
            extracted_values_from_script: None,
            is_critical: None,
            name,
            request,
            retry: None,
            subtype,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allow_failure(mut self, value: bool) -> Self {
        self.allow_failure = Some(value);
        self
    }

    pub fn exit_if_succeed(mut self, value: bool) -> Self {
        self.exit_if_succeed = Some(value);
        self
    }

    pub fn extracted_values(
        mut self,
        value: Vec<crate::datadogV1::model::SyntheticsParsingOptions>,
    ) -> Self {
        self.extracted_values = Some(value);
        self
    }

    pub fn extracted_values_from_script(mut self, value: String) -> Self {
        self.extracted_values_from_script = Some(value);
        self
    }

    pub fn is_critical(mut self, value: bool) -> Self {
        self.is_critical = Some(value);
        self
    }

    pub fn retry(mut self, value: crate::datadogV1::model::SyntheticsTestOptionsRetry) -> Self {
        self.retry = Some(value);
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

impl<'de> Deserialize<'de> for SyntheticsAPITestStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsAPITestStepVisitor;
        impl<'a> Visitor<'a> for SyntheticsAPITestStepVisitor {
            type Value = SyntheticsAPITestStep;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allow_failure: Option<bool> = None;
                let mut assertions: Option<Vec<crate::datadogV1::model::SyntheticsAssertion>> =
                    None;
                let mut exit_if_succeed: Option<bool> = None;
                let mut extracted_values: Option<
                    Vec<crate::datadogV1::model::SyntheticsParsingOptions>,
                > = None;
                let mut extracted_values_from_script: Option<String> = None;
                let mut is_critical: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut request: Option<crate::datadogV1::model::SyntheticsTestRequest> = None;
                let mut retry: Option<crate::datadogV1::model::SyntheticsTestOptionsRetry> = None;
                let mut subtype: Option<crate::datadogV1::model::SyntheticsAPITestStepSubtype> =
                    None;
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
                        "assertions" => {
                            assertions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exitIfSucceed" => {
                            if v.is_null() {
                                continue;
                            }
                            exit_if_succeed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "extractedValues" => {
                            if v.is_null() {
                                continue;
                            }
                            extracted_values =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "extractedValuesFromScript" => {
                            if v.is_null() {
                                continue;
                            }
                            extracted_values_from_script =
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
                        "request" => {
                            request = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retry" => {
                            if v.is_null() {
                                continue;
                            }
                            retry = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subtype" => {
                            subtype = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _subtype) = subtype {
                                match _subtype {
                                    crate::datadogV1::model::SyntheticsAPITestStepSubtype::UnparsedObject(_subtype) => {
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
                let assertions = assertions.ok_or_else(|| M::Error::missing_field("assertions"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let request = request.ok_or_else(|| M::Error::missing_field("request"))?;
                let subtype = subtype.ok_or_else(|| M::Error::missing_field("subtype"))?;

                let content = SyntheticsAPITestStep {
                    allow_failure,
                    assertions,
                    exit_if_succeed,
                    extracted_values,
                    extracted_values_from_script,
                    is_critical,
                    name,
                    request,
                    retry,
                    subtype,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsAPITestStepVisitor)
    }
}

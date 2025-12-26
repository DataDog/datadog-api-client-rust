// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The subtest step used in a Synthetics multi-step API test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsAPISubtestStep {
    /// Determines whether or not to continue with test if this step fails.
    #[serde(rename = "allowFailure")]
    pub allow_failure: Option<bool>,
    /// A boolean set to always execute this step even if the previous step failed or was skipped.
    #[serde(rename = "alwaysExecute")]
    pub always_execute: Option<bool>,
    /// Determines whether or not to exit the test if the step succeeds.
    #[serde(rename = "exitIfSucceed")]
    pub exit_if_succeed: Option<bool>,
    /// Generate variables using JavaScript.
    #[serde(rename = "extractedValuesFromScript")]
    pub extracted_values_from_script: Option<String>,
    /// ID of the step.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Determines whether or not to consider the entire test as failed if this step fails.
    /// Can be used only if `allowFailure` is `true`.
    #[serde(rename = "isCritical")]
    pub is_critical: Option<bool>,
    /// The name of the step.
    #[serde(rename = "name")]
    pub name: String,
    /// Object describing the retry strategy to apply to a Synthetic test.
    #[serde(rename = "retry")]
    pub retry: Option<crate::datadogV1::model::SyntheticsTestOptionsRetry>,
    /// Public ID of the test to be played as part of a `playSubTest` step type.
    #[serde(rename = "subtestPublicId")]
    pub subtest_public_id: String,
    /// The subtype of the Synthetic multi-step API subtest step.
    #[serde(rename = "subtype")]
    pub subtype: crate::datadogV1::model::SyntheticsAPISubtestStepSubtype,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsAPISubtestStep {
    pub fn new(
        name: String,
        subtest_public_id: String,
        subtype: crate::datadogV1::model::SyntheticsAPISubtestStepSubtype,
    ) -> SyntheticsAPISubtestStep {
        SyntheticsAPISubtestStep {
            allow_failure: None,
            always_execute: None,
            exit_if_succeed: None,
            extracted_values_from_script: None,
            id: None,
            is_critical: None,
            name,
            retry: None,
            subtest_public_id,
            subtype,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allow_failure(mut self, value: bool) -> Self {
        self.allow_failure = Some(value);
        self
    }

    pub fn always_execute(mut self, value: bool) -> Self {
        self.always_execute = Some(value);
        self
    }

    pub fn exit_if_succeed(mut self, value: bool) -> Self {
        self.exit_if_succeed = Some(value);
        self
    }

    pub fn extracted_values_from_script(mut self, value: String) -> Self {
        self.extracted_values_from_script = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
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

impl<'de> Deserialize<'de> for SyntheticsAPISubtestStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsAPISubtestStepVisitor;
        impl<'a> Visitor<'a> for SyntheticsAPISubtestStepVisitor {
            type Value = SyntheticsAPISubtestStep;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allow_failure: Option<bool> = None;
                let mut always_execute: Option<bool> = None;
                let mut exit_if_succeed: Option<bool> = None;
                let mut extracted_values_from_script: Option<String> = None;
                let mut id: Option<String> = None;
                let mut is_critical: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut retry: Option<crate::datadogV1::model::SyntheticsTestOptionsRetry> = None;
                let mut subtest_public_id: Option<String> = None;
                let mut subtype: Option<crate::datadogV1::model::SyntheticsAPISubtestStepSubtype> =
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
                        "alwaysExecute" => {
                            if v.is_null() {
                                continue;
                            }
                            always_execute =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exitIfSucceed" => {
                            if v.is_null() {
                                continue;
                            }
                            exit_if_succeed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "extractedValuesFromScript" => {
                            if v.is_null() {
                                continue;
                            }
                            extracted_values_from_script =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "retry" => {
                            if v.is_null() {
                                continue;
                            }
                            retry = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subtestPublicId" => {
                            subtest_public_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subtype" => {
                            subtype = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _subtype) = subtype {
                                match _subtype {
                                    crate::datadogV1::model::SyntheticsAPISubtestStepSubtype::UnparsedObject(_subtype) => {
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
                let subtest_public_id = subtest_public_id
                    .ok_or_else(|| M::Error::missing_field("subtest_public_id"))?;
                let subtype = subtype.ok_or_else(|| M::Error::missing_field("subtype"))?;

                let content = SyntheticsAPISubtestStep {
                    allow_failure,
                    always_execute,
                    exit_if_succeed,
                    extracted_values_from_script,
                    id,
                    is_critical,
                    name,
                    retry,
                    subtest_public_id,
                    subtype,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsAPISubtestStepVisitor)
    }
}

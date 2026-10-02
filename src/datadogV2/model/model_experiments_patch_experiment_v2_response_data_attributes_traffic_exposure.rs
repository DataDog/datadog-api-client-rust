// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Traffic exposure fraction or schedule configured for the experiment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsPatchExperimentV2ResponseDataAttributesTrafficExposure {
    /// STATIC exposure fraction. Draft experiments can change this value. After start only warehouse experiments without a Datadog flag can change a STATIC fraction through the public API.
    #[serde(rename = "fraction")]
    pub fraction: Option<f64>,
    /// Whether exposure uses a fixed fraction or a sequence of steps.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesTrafficExposureMode,
    /// Configured exposure plan rather than wall-clock history. At least two steps must have strictly increasing fractions and no gaps. Warehouse steps start at assignments_start_date and can use different durations. New Datadog plans have at most five steps and a first fraction above zero. Their nonfinal durations must be equal and exclude time paused. Datadog steps start with the experiment. Running warehouse experiments can replace step fractions, durations, and the exposure mode. After start, Datadog exposure plans cannot change through the public API. The final duration is null and its fraction holds until assignment ends.
    #[serde(rename = "steps")]
    pub steps: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesTrafficExposureStepsItems>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsPatchExperimentV2ResponseDataAttributesTrafficExposure {
    pub fn new(
        mode: crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesTrafficExposureMode,
    ) -> ExperimentsPatchExperimentV2ResponseDataAttributesTrafficExposure {
        ExperimentsPatchExperimentV2ResponseDataAttributesTrafficExposure {
            fraction: None,
            mode,
            steps: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn fraction(mut self, value: f64) -> Self {
        self.fraction = Some(value);
        self
    }

    pub fn steps(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesTrafficExposureStepsItems>,
    ) -> Self {
        self.steps = Some(value);
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

impl<'de> Deserialize<'de> for ExperimentsPatchExperimentV2ResponseDataAttributesTrafficExposure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPatchExperimentV2ResponseDataAttributesTrafficExposureVisitor;
        impl<'a> Visitor<'a> for ExperimentsPatchExperimentV2ResponseDataAttributesTrafficExposureVisitor {
            type Value = ExperimentsPatchExperimentV2ResponseDataAttributesTrafficExposure;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut fraction: Option<f64> = None;
                let mut mode: Option<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesTrafficExposureMode> = None;
                let mut steps: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesTrafficExposureStepsItems>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fraction" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            fraction = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mode" => {
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesTrafficExposureMode::UnparsedObject(_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "steps" => {
                            if v.is_null() {
                                continue;
                            }
                            steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;

                let content = ExperimentsPatchExperimentV2ResponseDataAttributesTrafficExposure {
                    fraction,
                    mode,
                    steps,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ExperimentsPatchExperimentV2ResponseDataAttributesTrafficExposureVisitor,
        )
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configured exposure plan rather than wall-clock history. At least two steps must have strictly increasing fractions and no gaps. Warehouse steps start at assignments_start_date and can use different durations. New Datadog plans have at most five steps and a first fraction above zero. Their nonfinal durations must be equal and exclude time paused. Datadog steps start with the experiment. Running warehouse experiments can replace step fractions, durations, and the exposure mode. After start, Datadog exposure plans cannot change through the public API. The final duration is null and its fraction holds until assignment ends.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsCreateExperimentV2RequestDataAttributesTrafficExposureStepsItems {
    /// Positive step duration in milliseconds. Datadog durations exclude pauses. Send null for the final step.
    #[serialize_always]
    #[serde(rename = "duration_ms")]
    pub duration_ms: Option<i64>,
    /// Fraction of traffic exposed during this step.
    #[serde(rename = "fraction")]
    pub fraction: f64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsCreateExperimentV2RequestDataAttributesTrafficExposureStepsItems {
    pub fn new(
        duration_ms: Option<i64>,
        fraction: f64,
    ) -> ExperimentsCreateExperimentV2RequestDataAttributesTrafficExposureStepsItems {
        ExperimentsCreateExperimentV2RequestDataAttributesTrafficExposureStepsItems {
            duration_ms,
            fraction,
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

impl<'de> Deserialize<'de>
    for ExperimentsCreateExperimentV2RequestDataAttributesTrafficExposureStepsItems
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsCreateExperimentV2RequestDataAttributesTrafficExposureStepsItemsVisitor;
        impl<'a> Visitor<'a>
            for ExperimentsCreateExperimentV2RequestDataAttributesTrafficExposureStepsItemsVisitor
        {
            type Value =
                ExperimentsCreateExperimentV2RequestDataAttributesTrafficExposureStepsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut duration_ms: Option<Option<i64>> = None;
                let mut fraction: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "duration_ms" => {
                            duration_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fraction" => {
                            fraction = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let duration_ms =
                    duration_ms.ok_or_else(|| M::Error::missing_field("duration_ms"))?;
                let fraction = fraction.ok_or_else(|| M::Error::missing_field("fraction"))?;

                let content =
                    ExperimentsCreateExperimentV2RequestDataAttributesTrafficExposureStepsItems {
                        duration_ms,
                        fraction,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ExperimentsCreateExperimentV2RequestDataAttributesTrafficExposureStepsItemsVisitor,
        )
    }
}

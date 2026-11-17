// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Steps to update. Steps that are not set are left unchanged.
/// A `tracking_plan` replaces the whole stored tracking plan, so include every event and property to keep.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsLaunchPatchSteps {
    /// Dashboard linked to the launch.
    #[serde(rename = "dashboard")]
    pub dashboard: Option<crate::datadogV2::model::ProductAnalyticsLaunchDashboardStep>,
    /// Experiment linked to the launch. The draft fields are cleared when `experiment_id` is set.
    #[serde(rename = "experiment")]
    pub experiment: Option<crate::datadogV2::model::ProductAnalyticsLaunchExperimentStep>,
    /// Funnel linked to the launch.
    #[serde(rename = "funnel")]
    pub funnel: Option<crate::datadogV2::model::ProductAnalyticsLaunchFunnelStep>,
    /// Tracking plan of a launch, listing the events and properties to instrument.
    #[serde(rename = "tracking_plan")]
    pub tracking_plan: Option<crate::datadogV2::model::ProductAnalyticsLaunchTrackingPlanRequest>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsLaunchPatchSteps {
    pub fn new() -> ProductAnalyticsLaunchPatchSteps {
        ProductAnalyticsLaunchPatchSteps {
            dashboard: None,
            experiment: None,
            funnel: None,
            tracking_plan: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dashboard(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsLaunchDashboardStep,
    ) -> Self {
        self.dashboard = Some(value);
        self
    }

    pub fn experiment(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsLaunchExperimentStep,
    ) -> Self {
        self.experiment = Some(value);
        self
    }

    pub fn funnel(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsLaunchFunnelStep,
    ) -> Self {
        self.funnel = Some(value);
        self
    }

    pub fn tracking_plan(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsLaunchTrackingPlanRequest,
    ) -> Self {
        self.tracking_plan = Some(value);
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

impl Default for ProductAnalyticsLaunchPatchSteps {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsLaunchPatchSteps {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsLaunchPatchStepsVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsLaunchPatchStepsVisitor {
            type Value = ProductAnalyticsLaunchPatchSteps;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dashboard: Option<
                    crate::datadogV2::model::ProductAnalyticsLaunchDashboardStep,
                > = None;
                let mut experiment: Option<
                    crate::datadogV2::model::ProductAnalyticsLaunchExperimentStep,
                > = None;
                let mut funnel: Option<crate::datadogV2::model::ProductAnalyticsLaunchFunnelStep> =
                    None;
                let mut tracking_plan: Option<
                    crate::datadogV2::model::ProductAnalyticsLaunchTrackingPlanRequest,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dashboard" => {
                            if v.is_null() {
                                continue;
                            }
                            dashboard = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experiment" => {
                            if v.is_null() {
                                continue;
                            }
                            experiment = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "funnel" => {
                            if v.is_null() {
                                continue;
                            }
                            funnel = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tracking_plan" => {
                            if v.is_null() {
                                continue;
                            }
                            tracking_plan =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProductAnalyticsLaunchPatchSteps {
                    dashboard,
                    experiment,
                    funnel,
                    tracking_plan,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsLaunchPatchStepsVisitor)
    }
}

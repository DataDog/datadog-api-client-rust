// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes to update on a launch. Fields that are not set are left unchanged, and `null` clears a nullable
/// field.
/// Setting a non-empty `feature_flag_id` clears the stored `feature_flag_key` when `feature_flag_key` is not set.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsLaunchPatchAttributes {
    /// UUID of the RUM application that the launch belongs to.
    #[serde(
        rename = "application_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub application_id: Option<Option<String>>,
    /// Latest inspection of the repository for the tracking plan instrumentation. A new analysis replaces the stored
    /// one.
    /// Events and properties that it does not list are not verified.
    #[serde(rename = "code_analysis")]
    pub code_analysis: Option<crate::datadogV2::model::ProductAnalyticsLaunchCodeAnalysis>,
    /// Context of the feature being launched.
    #[serde(rename = "context")]
    pub context: Option<String>,
    /// Description of the feature being launched.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// UUID of the Datadog feature flag that gates the launch.
    #[serde(
        rename = "feature_flag_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub feature_flag_id: Option<Option<String>>,
    /// Key of a feature flag managed outside Datadog. At most 256 characters, with no leading or trailing spaces.
    #[serde(
        rename = "feature_flag_key",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub feature_flag_key: Option<Option<String>>,
    /// KPIs that the launch is meant to move. Replaces the stored list, and `null` clears it.
    #[serde(rename = "kpis", default, with = "::serde_with::rust::double_option")]
    pub kpis: Option<Option<Vec<crate::datadogV2::model::ProductAnalyticsLaunchKPI>>>,
    /// Name of the launch.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Lifecycle phase of a launch. A new launch starts in `plan` or `setup`, and defaults to `plan`.
    /// Allowed changes: `plan` to `setup`, `setup` to `ready`, `ready` to `setup` or `launched`, and `launched` to
    /// `ready`.
    #[serde(rename = "phase")]
    pub phase: Option<crate::datadogV2::model::ProductAnalyticsLaunchPhase>,
    /// ID of the code repository that implements the feature.
    #[serde(
        rename = "repository_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub repository_id: Option<Option<String>>,
    /// Steps to update. Steps that are not set are left unchanged.
    /// A `tracking_plan` replaces the whole stored tracking plan, so include every event and property to keep.
    #[serde(rename = "steps")]
    pub steps: Option<crate::datadogV2::model::ProductAnalyticsLaunchPatchSteps>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsLaunchPatchAttributes {
    pub fn new() -> ProductAnalyticsLaunchPatchAttributes {
        ProductAnalyticsLaunchPatchAttributes {
            application_id: None,
            code_analysis: None,
            context: None,
            description: None,
            feature_flag_id: None,
            feature_flag_key: None,
            kpis: None,
            name: None,
            phase: None,
            repository_id: None,
            steps: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn application_id(mut self, value: Option<String>) -> Self {
        self.application_id = Some(value);
        self
    }

    pub fn code_analysis(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsLaunchCodeAnalysis,
    ) -> Self {
        self.code_analysis = Some(value);
        self
    }

    pub fn context(mut self, value: String) -> Self {
        self.context = Some(value);
        self
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn feature_flag_id(mut self, value: Option<String>) -> Self {
        self.feature_flag_id = Some(value);
        self
    }

    pub fn feature_flag_key(mut self, value: Option<String>) -> Self {
        self.feature_flag_key = Some(value);
        self
    }

    pub fn kpis(
        mut self,
        value: Option<Vec<crate::datadogV2::model::ProductAnalyticsLaunchKPI>>,
    ) -> Self {
        self.kpis = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn phase(mut self, value: crate::datadogV2::model::ProductAnalyticsLaunchPhase) -> Self {
        self.phase = Some(value);
        self
    }

    pub fn repository_id(mut self, value: Option<String>) -> Self {
        self.repository_id = Some(value);
        self
    }

    pub fn steps(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsLaunchPatchSteps,
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

impl Default for ProductAnalyticsLaunchPatchAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsLaunchPatchAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsLaunchPatchAttributesVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsLaunchPatchAttributesVisitor {
            type Value = ProductAnalyticsLaunchPatchAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<Option<String>> = None;
                let mut code_analysis: Option<
                    crate::datadogV2::model::ProductAnalyticsLaunchCodeAnalysis,
                > = None;
                let mut context: Option<String> = None;
                let mut description: Option<Option<String>> = None;
                let mut feature_flag_id: Option<Option<String>> = None;
                let mut feature_flag_key: Option<Option<String>> = None;
                let mut kpis: Option<
                    Option<Vec<crate::datadogV2::model::ProductAnalyticsLaunchKPI>>,
                > = None;
                let mut name: Option<String> = None;
                let mut phase: Option<crate::datadogV2::model::ProductAnalyticsLaunchPhase> = None;
                let mut repository_id: Option<Option<String>> = None;
                let mut steps: Option<crate::datadogV2::model::ProductAnalyticsLaunchPatchSteps> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "application_id" => {
                            application_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "code_analysis" => {
                            if v.is_null() {
                                continue;
                            }
                            code_analysis =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "context" => {
                            if v.is_null() {
                                continue;
                            }
                            context = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "feature_flag_id" => {
                            feature_flag_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "feature_flag_key" => {
                            feature_flag_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "kpis" => {
                            kpis = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "phase" => {
                            if v.is_null() {
                                continue;
                            }
                            phase = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _phase) = phase {
                                match _phase {
                                    crate::datadogV2::model::ProductAnalyticsLaunchPhase::UnparsedObject(_phase) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "repository_id" => {
                            repository_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = ProductAnalyticsLaunchPatchAttributes {
                    application_id,
                    code_analysis,
                    context,
                    description,
                    feature_flag_id,
                    feature_flag_key,
                    kpis,
                    name,
                    phase,
                    repository_id,
                    steps,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsLaunchPatchAttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a launch to create or replace.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsLaunchRequestAttributes {
    /// UUID of the RUM application that the launch belongs to.
    #[serde(
        rename = "application_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub application_id: Option<Option<String>>,
    /// Context of the feature being launched, used to build the tracking plan.
    #[serde(rename = "context")]
    pub context: String,
    /// Description of the feature being launched.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// UUID of the Datadog feature flag that gates the launch. Do not set it together with `feature_flag_key`.
    #[serde(
        rename = "feature_flag_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub feature_flag_id: Option<Option<String>>,
    /// Key of a feature flag managed outside Datadog, as evaluated in code. At most 256 characters, with no
    /// leading or trailing spaces. Do not set it together with `feature_flag_id`. `null` clears it. On replace,
    /// omitting it keeps the stored key unless `feature_flag_id` is set.
    #[serde(
        rename = "feature_flag_key",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub feature_flag_key: Option<Option<String>>,
    /// KPIs that the launch is meant to move.
    #[serde(rename = "kpis")]
    pub kpis: Option<Vec<crate::datadogV2::model::ProductAnalyticsLaunchKPI>>,
    /// Name of the launch.
    #[serde(rename = "name")]
    pub name: String,
    /// Lifecycle phase of a launch. A new launch starts in `plan` or `setup`, and defaults to `plan`.
    /// Allowed changes: `plan` to `setup`, `setup` to `ready`, `ready` to `setup` or `launched`, and `launched` to
    /// `ready`.
    #[serde(rename = "phase")]
    pub phase: Option<crate::datadogV2::model::ProductAnalyticsLaunchPhase>,
    /// ID of the code repository that implements the feature.
    #[serde(rename = "repository_id")]
    pub repository_id: Option<String>,
    /// Steps of the launch. The tracking plan is required.
    #[serde(rename = "steps")]
    pub steps: crate::datadogV2::model::ProductAnalyticsLaunchStepsRequest,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsLaunchRequestAttributes {
    pub fn new(
        context: String,
        name: String,
        steps: crate::datadogV2::model::ProductAnalyticsLaunchStepsRequest,
    ) -> ProductAnalyticsLaunchRequestAttributes {
        ProductAnalyticsLaunchRequestAttributes {
            application_id: None,
            context,
            description: None,
            feature_flag_id: None,
            feature_flag_key: None,
            kpis: None,
            name,
            phase: None,
            repository_id: None,
            steps,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn application_id(mut self, value: Option<String>) -> Self {
        self.application_id = Some(value);
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

    pub fn kpis(mut self, value: Vec<crate::datadogV2::model::ProductAnalyticsLaunchKPI>) -> Self {
        self.kpis = Some(value);
        self
    }

    pub fn phase(mut self, value: crate::datadogV2::model::ProductAnalyticsLaunchPhase) -> Self {
        self.phase = Some(value);
        self
    }

    pub fn repository_id(mut self, value: String) -> Self {
        self.repository_id = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsLaunchRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsLaunchRequestAttributesVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsLaunchRequestAttributesVisitor {
            type Value = ProductAnalyticsLaunchRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<Option<String>> = None;
                let mut context: Option<String> = None;
                let mut description: Option<Option<String>> = None;
                let mut feature_flag_id: Option<Option<String>> = None;
                let mut feature_flag_key: Option<Option<String>> = None;
                let mut kpis: Option<Vec<crate::datadogV2::model::ProductAnalyticsLaunchKPI>> =
                    None;
                let mut name: Option<String> = None;
                let mut phase: Option<crate::datadogV2::model::ProductAnalyticsLaunchPhase> = None;
                let mut repository_id: Option<String> = None;
                let mut steps: Option<crate::datadogV2::model::ProductAnalyticsLaunchStepsRequest> =
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
                        "context" => {
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
                            if v.is_null() {
                                continue;
                            }
                            kpis = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
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
                            if v.is_null() {
                                continue;
                            }
                            repository_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "steps" => {
                            steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let context = context.ok_or_else(|| M::Error::missing_field("context"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let steps = steps.ok_or_else(|| M::Error::missing_field("steps"))?;

                let content = ProductAnalyticsLaunchRequestAttributes {
                    application_id,
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

        deserializer.deserialize_any(ProductAnalyticsLaunchRequestAttributesVisitor)
    }
}

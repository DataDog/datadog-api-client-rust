// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a launch.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsLaunchAttributes {
    /// UUID of the RUM application that the launch belongs to.
    #[serde(
        rename = "application_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub application_id: Option<Option<String>>,
    /// Context of the feature being launched.
    #[serde(
        rename = "context",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub context: Option<Option<String>>,
    /// Time the launch was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// UUID of the user who created the launch.
    #[serde(rename = "created_by")]
    pub created_by: String,
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
    /// Key of a feature flag managed outside Datadog.
    #[serde(
        rename = "feature_flag_key",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub feature_flag_key: Option<Option<String>>,
    /// KPIs that the launch is meant to move.
    #[serde(rename = "kpis", default, with = "::serde_with::rust::double_option")]
    pub kpis: Option<Option<Vec<crate::datadogV2::model::ProductAnalyticsLaunchKPI>>>,
    /// Name of the launch.
    #[serde(rename = "name")]
    pub name: String,
    /// ID of the organization that owns the launch.
    #[serde(rename = "org_id")]
    pub org_id: i64,
    /// Lifecycle phase of a launch. A new launch starts in `plan` or `setup`, and defaults to `plan`.
    /// Allowed changes: `plan` to `setup`, `setup` to `ready`, `ready` to `setup` or `launched`, and `launched` to
    /// `ready`.
    #[serde(rename = "phase")]
    pub phase: crate::datadogV2::model::ProductAnalyticsLaunchPhase,
    /// ID of the code repository that implements the feature.
    #[serde(rename = "repository_id")]
    pub repository_id: Option<String>,
    /// Steps of the launch.
    #[serde(rename = "steps")]
    pub steps: crate::datadogV2::model::ProductAnalyticsLaunchSteps,
    /// Time the launch was last updated.
    #[serde(
        rename = "updated_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub updated_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// UUID of the user who last updated the launch.
    #[serde(
        rename = "updated_by",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub updated_by: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsLaunchAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: String,
        name: String,
        org_id: i64,
        phase: crate::datadogV2::model::ProductAnalyticsLaunchPhase,
        steps: crate::datadogV2::model::ProductAnalyticsLaunchSteps,
    ) -> ProductAnalyticsLaunchAttributes {
        ProductAnalyticsLaunchAttributes {
            application_id: None,
            context: None,
            created_at,
            created_by,
            description: None,
            feature_flag_id: None,
            feature_flag_key: None,
            kpis: None,
            name,
            org_id,
            phase,
            repository_id: None,
            steps,
            updated_at: None,
            updated_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn application_id(mut self, value: Option<String>) -> Self {
        self.application_id = Some(value);
        self
    }

    pub fn context(mut self, value: Option<String>) -> Self {
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

    pub fn repository_id(mut self, value: String) -> Self {
        self.repository_id = Some(value);
        self
    }

    pub fn updated_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn updated_by(mut self, value: Option<String>) -> Self {
        self.updated_by = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsLaunchAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsLaunchAttributesVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsLaunchAttributesVisitor {
            type Value = ProductAnalyticsLaunchAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<Option<String>> = None;
                let mut context: Option<Option<String>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut description: Option<Option<String>> = None;
                let mut feature_flag_id: Option<Option<String>> = None;
                let mut feature_flag_key: Option<Option<String>> = None;
                let mut kpis: Option<
                    Option<Vec<crate::datadogV2::model::ProductAnalyticsLaunchKPI>>,
                > = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut phase: Option<crate::datadogV2::model::ProductAnalyticsLaunchPhase> = None;
                let mut repository_id: Option<String> = None;
                let mut steps: Option<crate::datadogV2::model::ProductAnalyticsLaunchSteps> = None;
                let mut updated_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut updated_by: Option<Option<String>> = None;
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
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "phase" => {
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
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by" => {
                            updated_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let phase = phase.ok_or_else(|| M::Error::missing_field("phase"))?;
                let steps = steps.ok_or_else(|| M::Error::missing_field("steps"))?;

                let content = ProductAnalyticsLaunchAttributes {
                    application_id,
                    context,
                    created_at,
                    created_by,
                    description,
                    feature_flag_id,
                    feature_flag_key,
                    kpis,
                    name,
                    org_id,
                    phase,
                    repository_id,
                    steps,
                    updated_at,
                    updated_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsLaunchAttributesVisitor)
    }
}

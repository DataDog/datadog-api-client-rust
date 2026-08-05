// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a governance control.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GovernanceControlAttributes {
    /// The number of active detections for the control.
    #[serde(rename = "active_detections_count")]
    pub active_detections_count: i64,
    /// The value driver the control is grouped under, such as `security` or `cost`.
    #[serde(rename = "category")]
    pub category: String,
    /// The time the control configuration was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The UUID of the user who created the control configuration.
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// A human-readable description of what the control detects.
    #[serde(rename = "description")]
    pub description: String,
    /// How often detections are evaluated for the control.
    #[serde(rename = "detection_frequency")]
    pub detection_frequency: String,
    /// A free-form map of parameter names to their configured values.
    #[serde(rename = "detection_parameters")]
    pub detection_parameters: std::collections::BTreeMap<String, serde_json::Value>,
    /// The insight slugs associated with the control.
    #[serde(rename = "insights")]
    pub insights: Vec<String>,
    /// The time of the most recent detection for the control. `null` when there are no detections.
    #[serialize_always]
    #[serde(rename = "last_detection_at")]
    pub last_detection_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The number of mitigated detections for the control.
    #[serde(rename = "mitigated_detections_count")]
    pub mitigated_detections_count: i64,
    /// A free-form map of parameter names to their configured values.
    #[serde(rename = "mitigation_parameters")]
    pub mitigation_parameters: std::collections::BTreeMap<String, serde_json::Value>,
    /// The configured mitigation type for the control. Empty when not configured.
    #[serde(rename = "mitigation_type")]
    pub mitigation_type: String,
    /// The mitigations available for a control.
    #[serde(rename = "mitigations")]
    pub mitigations: Vec<crate::datadogV2::model::GovernanceControlMitigationDefinition>,
    /// Human-readable name of the control.
    #[serde(rename = "name")]
    pub name: String,
    /// The priority of the control, such as `High`.
    #[serde(rename = "priority")]
    pub priority: String,
    /// The product the control belongs to.
    #[serde(rename = "product")]
    pub product: String,
    /// The type of resource the control evaluates.
    #[serde(rename = "resource_type")]
    pub resource_type: String,
    /// The human-readable name of the resource type.
    #[serde(rename = "resource_type_display_name")]
    pub resource_type_display_name: String,
    /// An array of parameter definitions.
    #[serde(rename = "supported_detection_parameters")]
    pub supported_detection_parameters:
        Vec<crate::datadogV2::model::GovernanceControlParameterDefinition>,
    /// The control type, such as `Proactive` or `Detection`.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GovernanceControlAttributes {
    pub fn new(
        active_detections_count: i64,
        category: String,
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: String,
        description: String,
        detection_frequency: String,
        detection_parameters: std::collections::BTreeMap<String, serde_json::Value>,
        insights: Vec<String>,
        last_detection_at: Option<chrono::DateTime<chrono::Utc>>,
        mitigated_detections_count: i64,
        mitigation_parameters: std::collections::BTreeMap<String, serde_json::Value>,
        mitigation_type: String,
        mitigations: Vec<crate::datadogV2::model::GovernanceControlMitigationDefinition>,
        name: String,
        priority: String,
        product: String,
        resource_type: String,
        resource_type_display_name: String,
        supported_detection_parameters: Vec<
            crate::datadogV2::model::GovernanceControlParameterDefinition,
        >,
        type_: String,
    ) -> GovernanceControlAttributes {
        GovernanceControlAttributes {
            active_detections_count,
            category,
            created_at,
            created_by,
            description,
            detection_frequency,
            detection_parameters,
            insights,
            last_detection_at,
            mitigated_detections_count,
            mitigation_parameters,
            mitigation_type,
            mitigations,
            name,
            priority,
            product,
            resource_type,
            resource_type_display_name,
            supported_detection_parameters,
            type_,
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

impl<'de> Deserialize<'de> for GovernanceControlAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GovernanceControlAttributesVisitor;
        impl<'a> Visitor<'a> for GovernanceControlAttributesVisitor {
            type Value = GovernanceControlAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut active_detections_count: Option<i64> = None;
                let mut category: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut description: Option<String> = None;
                let mut detection_frequency: Option<String> = None;
                let mut detection_parameters: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut insights: Option<Vec<String>> = None;
                let mut last_detection_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut mitigated_detections_count: Option<i64> = None;
                let mut mitigation_parameters: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut mitigation_type: Option<String> = None;
                let mut mitigations: Option<
                    Vec<crate::datadogV2::model::GovernanceControlMitigationDefinition>,
                > = None;
                let mut name: Option<String> = None;
                let mut priority: Option<String> = None;
                let mut product: Option<String> = None;
                let mut resource_type: Option<String> = None;
                let mut resource_type_display_name: Option<String> = None;
                let mut supported_detection_parameters: Option<
                    Vec<crate::datadogV2::model::GovernanceControlParameterDefinition>,
                > = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "active_detections_count" => {
                            active_detections_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "category" => {
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "detection_frequency" => {
                            detection_frequency =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "detection_parameters" => {
                            detection_parameters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "insights" => {
                            insights = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_detection_at" => {
                            last_detection_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mitigated_detections_count" => {
                            mitigated_detections_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mitigation_parameters" => {
                            mitigation_parameters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mitigation_type" => {
                            mitigation_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mitigations" => {
                            mitigations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product" => {
                            product = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_type" => {
                            resource_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_type_display_name" => {
                            resource_type_display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "supported_detection_parameters" => {
                            supported_detection_parameters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let active_detections_count = active_detections_count
                    .ok_or_else(|| M::Error::missing_field("active_detections_count"))?;
                let category = category.ok_or_else(|| M::Error::missing_field("category"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let detection_frequency = detection_frequency
                    .ok_or_else(|| M::Error::missing_field("detection_frequency"))?;
                let detection_parameters = detection_parameters
                    .ok_or_else(|| M::Error::missing_field("detection_parameters"))?;
                let insights = insights.ok_or_else(|| M::Error::missing_field("insights"))?;
                let last_detection_at = last_detection_at
                    .ok_or_else(|| M::Error::missing_field("last_detection_at"))?;
                let mitigated_detections_count = mitigated_detections_count
                    .ok_or_else(|| M::Error::missing_field("mitigated_detections_count"))?;
                let mitigation_parameters = mitigation_parameters
                    .ok_or_else(|| M::Error::missing_field("mitigation_parameters"))?;
                let mitigation_type =
                    mitigation_type.ok_or_else(|| M::Error::missing_field("mitigation_type"))?;
                let mitigations =
                    mitigations.ok_or_else(|| M::Error::missing_field("mitigations"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let priority = priority.ok_or_else(|| M::Error::missing_field("priority"))?;
                let product = product.ok_or_else(|| M::Error::missing_field("product"))?;
                let resource_type =
                    resource_type.ok_or_else(|| M::Error::missing_field("resource_type"))?;
                let resource_type_display_name = resource_type_display_name
                    .ok_or_else(|| M::Error::missing_field("resource_type_display_name"))?;
                let supported_detection_parameters = supported_detection_parameters
                    .ok_or_else(|| M::Error::missing_field("supported_detection_parameters"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = GovernanceControlAttributes {
                    active_detections_count,
                    category,
                    created_at,
                    created_by,
                    description,
                    detection_frequency,
                    detection_parameters,
                    insights,
                    last_detection_at,
                    mitigated_detections_count,
                    mitigation_parameters,
                    mitigation_type,
                    mitigations,
                    name,
                    priority,
                    product,
                    resource_type,
                    resource_type_display_name,
                    supported_detection_parameters,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GovernanceControlAttributesVisitor)
    }
}

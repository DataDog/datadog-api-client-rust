// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a governance control detection.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GovernanceControlDetectionAttributes {
    /// The identifier of the team the detection is assigned to, if any.
    #[serde(rename = "assigned_team")]
    pub assigned_team: Option<String>,
    /// The identifier of the user the detection is assigned to, if any.
    #[serde(rename = "assigned_to")]
    pub assigned_to: Option<String>,
    /// How the detection's current assignment was determined. Possible values are `auto_resolved`, `manual`, `reassigned`, and `cleared`.
    #[serde(rename = "assignment_source")]
    pub assignment_source: crate::datadogV2::model::GovernanceControlDetectionAssignmentSource,
    /// The unique identifier of the control that produced this detection.
    #[serde(rename = "control_id")]
    pub control_id: String,
    /// The date and time when the detection was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The type of detection, which determines what condition was detected.
    #[serde(rename = "detection_type")]
    pub detection_type: String,
    /// The human-readable name of the detected resource.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// The date and time when the detection was marked as an exception, if applicable.
    #[serde(rename = "exception_at")]
    pub exception_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The identifier of the user who marked the detection as an exception, if applicable.
    #[serde(rename = "exception_by")]
    pub exception_by: Option<String>,
    /// Free-form metadata associated with the detection.
    #[serde(rename = "metadata")]
    pub metadata: Option<serde_json::Value>,
    /// The date and time after which the detection is scheduled to be mitigated, if applicable.
    #[serde(rename = "mitigate_after")]
    pub mitigate_after: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time when the detection was mitigated, if applicable.
    #[serde(rename = "mitigated_at")]
    pub mitigated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The priority of the detection, if set.
    #[serde(rename = "priority")]
    pub priority: i64,
    /// The identifier of the resource the detection applies to.
    #[serde(rename = "resource_id")]
    pub resource_id: String,
    /// A link to the detected resource in Datadog.
    #[serde(rename = "resource_url")]
    pub resource_url: String,
    /// The current state of the detection. Possible values are `active`, `exception`, `mitigated`, `inactive`, `obsolete`, `resolved_externally`, and `mitigation_in_progress`.
    #[serde(rename = "state")]
    pub state: crate::datadogV2::model::GovernanceControlDetectionState,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GovernanceControlDetectionAttributes {
    pub fn new(
        assignment_source: crate::datadogV2::model::GovernanceControlDetectionAssignmentSource,
        control_id: String,
        created_at: chrono::DateTime<chrono::Utc>,
        detection_type: String,
        display_name: String,
        priority: i64,
        resource_id: String,
        resource_url: String,
        state: crate::datadogV2::model::GovernanceControlDetectionState,
    ) -> GovernanceControlDetectionAttributes {
        GovernanceControlDetectionAttributes {
            assigned_team: None,
            assigned_to: None,
            assignment_source,
            control_id,
            created_at,
            detection_type,
            display_name,
            exception_at: None,
            exception_by: None,
            metadata: None,
            mitigate_after: None,
            mitigated_at: None,
            priority,
            resource_id,
            resource_url,
            state,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assigned_team(mut self, value: String) -> Self {
        self.assigned_team = Some(value);
        self
    }

    pub fn assigned_to(mut self, value: String) -> Self {
        self.assigned_to = Some(value);
        self
    }

    pub fn exception_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.exception_at = Some(value);
        self
    }

    pub fn exception_by(mut self, value: String) -> Self {
        self.exception_by = Some(value);
        self
    }

    pub fn metadata(mut self, value: serde_json::Value) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn mitigate_after(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.mitigate_after = Some(value);
        self
    }

    pub fn mitigated_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.mitigated_at = Some(value);
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

impl<'de> Deserialize<'de> for GovernanceControlDetectionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GovernanceControlDetectionAttributesVisitor;
        impl<'a> Visitor<'a> for GovernanceControlDetectionAttributesVisitor {
            type Value = GovernanceControlDetectionAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assigned_team: Option<String> = None;
                let mut assigned_to: Option<String> = None;
                let mut assignment_source: Option<
                    crate::datadogV2::model::GovernanceControlDetectionAssignmentSource,
                > = None;
                let mut control_id: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut detection_type: Option<String> = None;
                let mut display_name: Option<String> = None;
                let mut exception_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut exception_by: Option<String> = None;
                let mut metadata: Option<serde_json::Value> = None;
                let mut mitigate_after: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut mitigated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut priority: Option<i64> = None;
                let mut resource_id: Option<String> = None;
                let mut resource_url: Option<String> = None;
                let mut state: Option<crate::datadogV2::model::GovernanceControlDetectionState> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assigned_team" => {
                            if v.is_null() {
                                continue;
                            }
                            assigned_team =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "assigned_to" => {
                            if v.is_null() {
                                continue;
                            }
                            assigned_to =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "assignment_source" => {
                            assignment_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _assignment_source) = assignment_source {
                                match _assignment_source {
                                    crate::datadogV2::model::GovernanceControlDetectionAssignmentSource::UnparsedObject(_assignment_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "control_id" => {
                            control_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "detection_type" => {
                            detection_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_name" => {
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exception_at" => {
                            if v.is_null() {
                                continue;
                            }
                            exception_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exception_by" => {
                            if v.is_null() {
                                continue;
                            }
                            exception_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mitigate_after" => {
                            if v.is_null() {
                                continue;
                            }
                            mitigate_after =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mitigated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            mitigated_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_id" => {
                            resource_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_url" => {
                            resource_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV2::model::GovernanceControlDetectionState::UnparsedObject(_state) => {
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
                let assignment_source = assignment_source
                    .ok_or_else(|| M::Error::missing_field("assignment_source"))?;
                let control_id = control_id.ok_or_else(|| M::Error::missing_field("control_id"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let detection_type =
                    detection_type.ok_or_else(|| M::Error::missing_field("detection_type"))?;
                let display_name =
                    display_name.ok_or_else(|| M::Error::missing_field("display_name"))?;
                let priority = priority.ok_or_else(|| M::Error::missing_field("priority"))?;
                let resource_id =
                    resource_id.ok_or_else(|| M::Error::missing_field("resource_id"))?;
                let resource_url =
                    resource_url.ok_or_else(|| M::Error::missing_field("resource_url"))?;
                let state = state.ok_or_else(|| M::Error::missing_field("state"))?;

                let content = GovernanceControlDetectionAttributes {
                    assigned_team,
                    assigned_to,
                    assignment_source,
                    control_id,
                    created_at,
                    detection_type,
                    display_name,
                    exception_at,
                    exception_by,
                    metadata,
                    mitigate_after,
                    mitigated_at,
                    priority,
                    resource_id,
                    resource_url,
                    state,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GovernanceControlDetectionAttributesVisitor)
    }
}

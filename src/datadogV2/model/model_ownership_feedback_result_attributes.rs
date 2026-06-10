// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of an ownership feedback result.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OwnershipFeedbackResultAttributes {
    /// The feedback action to apply to an inference.
    #[serde(rename = "action")]
    pub action: crate::datadogV2::model::OwnershipFeedbackAction,
    /// The checksum of the inference after the feedback was applied.
    #[serde(rename = "checksum")]
    pub checksum: String,
    /// The lifecycle status of an ownership inference.
    #[serde(rename = "new_status")]
    pub new_status: crate::datadogV2::model::OwnershipInferenceStatus,
    /// The owner type for an ownership inference.
    #[serde(rename = "owner_type")]
    pub owner_type: crate::datadogV2::model::OwnershipOwnerType,
    /// The lifecycle status of an ownership inference.
    #[serde(rename = "previous_status")]
    pub previous_status: crate::datadogV2::model::OwnershipInferenceStatus,
    /// The primary contact reference for the inferred owner after the feedback was applied, formatted as `ref:handle/<owner_handle>`.
    #[serde(
        rename = "primary_contact_ref",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub primary_contact_ref: Option<Option<String>>,
    /// The time when the inference was updated by the feedback.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OwnershipFeedbackResultAttributes {
    pub fn new(
        action: crate::datadogV2::model::OwnershipFeedbackAction,
        checksum: String,
        new_status: crate::datadogV2::model::OwnershipInferenceStatus,
        owner_type: crate::datadogV2::model::OwnershipOwnerType,
        previous_status: crate::datadogV2::model::OwnershipInferenceStatus,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> OwnershipFeedbackResultAttributes {
        OwnershipFeedbackResultAttributes {
            action,
            checksum,
            new_status,
            owner_type,
            previous_status,
            primary_contact_ref: None,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn primary_contact_ref(mut self, value: Option<String>) -> Self {
        self.primary_contact_ref = Some(value);
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

impl<'de> Deserialize<'de> for OwnershipFeedbackResultAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OwnershipFeedbackResultAttributesVisitor;
        impl<'a> Visitor<'a> for OwnershipFeedbackResultAttributesVisitor {
            type Value = OwnershipFeedbackResultAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<crate::datadogV2::model::OwnershipFeedbackAction> = None;
                let mut checksum: Option<String> = None;
                let mut new_status: Option<crate::datadogV2::model::OwnershipInferenceStatus> =
                    None;
                let mut owner_type: Option<crate::datadogV2::model::OwnershipOwnerType> = None;
                let mut previous_status: Option<crate::datadogV2::model::OwnershipInferenceStatus> =
                    None;
                let mut primary_contact_ref: Option<Option<String>> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action" => {
                            action = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _action) = action {
                                match _action {
                                    crate::datadogV2::model::OwnershipFeedbackAction::UnparsedObject(_action) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "checksum" => {
                            checksum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "new_status" => {
                            new_status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _new_status) = new_status {
                                match _new_status {
                                    crate::datadogV2::model::OwnershipInferenceStatus::UnparsedObject(_new_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "owner_type" => {
                            owner_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _owner_type) = owner_type {
                                match _owner_type {
                                    crate::datadogV2::model::OwnershipOwnerType::UnparsedObject(
                                        _owner_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "previous_status" => {
                            previous_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _previous_status) = previous_status {
                                match _previous_status {
                                    crate::datadogV2::model::OwnershipInferenceStatus::UnparsedObject(_previous_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "primary_contact_ref" => {
                            primary_contact_ref =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let action = action.ok_or_else(|| M::Error::missing_field("action"))?;
                let checksum = checksum.ok_or_else(|| M::Error::missing_field("checksum"))?;
                let new_status = new_status.ok_or_else(|| M::Error::missing_field("new_status"))?;
                let owner_type = owner_type.ok_or_else(|| M::Error::missing_field("owner_type"))?;
                let previous_status =
                    previous_status.ok_or_else(|| M::Error::missing_field("previous_status"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = OwnershipFeedbackResultAttributes {
                    action,
                    checksum,
                    new_status,
                    owner_type,
                    previous_status,
                    primary_contact_ref,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OwnershipFeedbackResultAttributesVisitor)
    }
}

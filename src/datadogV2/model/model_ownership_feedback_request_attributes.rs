// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of an ownership feedback request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OwnershipFeedbackRequestAttributes {
    /// The feedback action to apply to an inference.
    #[serde(rename = "action")]
    pub action: crate::datadogV2::model::OwnershipFeedbackAction,
    /// The handle of the actor submitting the feedback.
    #[serde(rename = "actor_handle")]
    pub actor_handle: String,
    /// The type of actor submitting the feedback, for example `user` or `service`.
    #[serde(rename = "actor_type")]
    pub actor_type: String,
    /// The corrected owner handle. Required when `action` is `correct`.
    #[serde(
        rename = "corrected_owner_handle",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub corrected_owner_handle: Option<Option<String>>,
    /// The corrected owner type. Required when `action` is `correct`.
    #[serde(
        rename = "corrected_owner_type",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub corrected_owner_type: Option<Option<String>>,
    /// The checksum of the inference being acted upon. Must match the current inference checksum or the request returns a conflict.
    #[serde(rename = "inference_checksum")]
    pub inference_checksum: String,
    /// An optional free-form reason explaining the feedback.
    #[serde(rename = "reason", default, with = "::serde_with::rust::double_option")]
    pub reason: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OwnershipFeedbackRequestAttributes {
    pub fn new(
        action: crate::datadogV2::model::OwnershipFeedbackAction,
        actor_handle: String,
        actor_type: String,
        inference_checksum: String,
    ) -> OwnershipFeedbackRequestAttributes {
        OwnershipFeedbackRequestAttributes {
            action,
            actor_handle,
            actor_type,
            corrected_owner_handle: None,
            corrected_owner_type: None,
            inference_checksum,
            reason: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn corrected_owner_handle(mut self, value: Option<String>) -> Self {
        self.corrected_owner_handle = Some(value);
        self
    }

    pub fn corrected_owner_type(mut self, value: Option<String>) -> Self {
        self.corrected_owner_type = Some(value);
        self
    }

    pub fn reason(mut self, value: Option<String>) -> Self {
        self.reason = Some(value);
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

impl<'de> Deserialize<'de> for OwnershipFeedbackRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OwnershipFeedbackRequestAttributesVisitor;
        impl<'a> Visitor<'a> for OwnershipFeedbackRequestAttributesVisitor {
            type Value = OwnershipFeedbackRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<crate::datadogV2::model::OwnershipFeedbackAction> = None;
                let mut actor_handle: Option<String> = None;
                let mut actor_type: Option<String> = None;
                let mut corrected_owner_handle: Option<Option<String>> = None;
                let mut corrected_owner_type: Option<Option<String>> = None;
                let mut inference_checksum: Option<String> = None;
                let mut reason: Option<Option<String>> = None;
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
                        "actor_handle" => {
                            actor_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "actor_type" => {
                            actor_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "corrected_owner_handle" => {
                            corrected_owner_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "corrected_owner_type" => {
                            corrected_owner_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inference_checksum" => {
                            inference_checksum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reason" => {
                            reason = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let action = action.ok_or_else(|| M::Error::missing_field("action"))?;
                let actor_handle =
                    actor_handle.ok_or_else(|| M::Error::missing_field("actor_handle"))?;
                let actor_type = actor_type.ok_or_else(|| M::Error::missing_field("actor_type"))?;
                let inference_checksum = inference_checksum
                    .ok_or_else(|| M::Error::missing_field("inference_checksum"))?;

                let content = OwnershipFeedbackRequestAttributes {
                    action,
                    actor_handle,
                    actor_type,
                    corrected_owner_handle,
                    corrected_owner_type,
                    inference_checksum,
                    reason,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OwnershipFeedbackRequestAttributesVisitor)
    }
}

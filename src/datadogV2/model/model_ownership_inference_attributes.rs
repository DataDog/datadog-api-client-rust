// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a single ownership inference.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OwnershipInferenceAttributes {
    /// A checksum that uniquely identifies the current state of the inference. Required when submitting feedback.
    #[serde(rename = "checksum")]
    pub checksum: String,
    /// The confidence score of the inference, expressed as a numeric string with up to four decimal places.
    #[serde(rename = "confidence")]
    pub confidence: String,
    /// The time when the inference was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The list of evidence versions associated with an inference.
    #[serialize_always]
    #[serde(rename = "evidence_versions")]
    pub evidence_versions: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// A human-readable explanation of how the inference was produced.
    #[serde(rename = "explanation")]
    pub explanation: String,
    /// The owner type for an ownership inference.
    #[serde(rename = "owner_type")]
    pub owner_type: crate::datadogV2::model::OwnershipOwnerType,
    /// The primary contact reference for the inferred owner, formatted as `ref:handle/<owner_handle>`.
    #[serde(
        rename = "primary_contact_ref",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub primary_contact_ref: Option<Option<String>>,
    /// The list of sources backing an ownership inference. Empty when the inference status is not whitelisted to expose sources.
    #[serde(rename = "sources")]
    pub sources: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The lifecycle status of an ownership inference.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::OwnershipInferenceStatus,
    /// The time when the inference was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OwnershipInferenceAttributes {
    pub fn new(
        checksum: String,
        confidence: String,
        created_at: chrono::DateTime<chrono::Utc>,
        evidence_versions: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
        explanation: String,
        owner_type: crate::datadogV2::model::OwnershipOwnerType,
        sources: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
        status: crate::datadogV2::model::OwnershipInferenceStatus,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> OwnershipInferenceAttributes {
        OwnershipInferenceAttributes {
            checksum,
            confidence,
            created_at,
            evidence_versions,
            explanation,
            owner_type,
            primary_contact_ref: None,
            sources,
            status,
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

impl<'de> Deserialize<'de> for OwnershipInferenceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OwnershipInferenceAttributesVisitor;
        impl<'a> Visitor<'a> for OwnershipInferenceAttributesVisitor {
            type Value = OwnershipInferenceAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut checksum: Option<String> = None;
                let mut confidence: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut evidence_versions: Option<
                    Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
                > = None;
                let mut explanation: Option<String> = None;
                let mut owner_type: Option<crate::datadogV2::model::OwnershipOwnerType> = None;
                let mut primary_contact_ref: Option<Option<String>> = None;
                let mut sources: Option<
                    Vec<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut status: Option<crate::datadogV2::model::OwnershipInferenceStatus> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "checksum" => {
                            checksum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "confidence" => {
                            confidence = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evidence_versions" => {
                            evidence_versions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "explanation" => {
                            explanation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "primary_contact_ref" => {
                            primary_contact_ref =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sources" => {
                            sources = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::OwnershipInferenceStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                let checksum = checksum.ok_or_else(|| M::Error::missing_field("checksum"))?;
                let confidence = confidence.ok_or_else(|| M::Error::missing_field("confidence"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let evidence_versions = evidence_versions
                    .ok_or_else(|| M::Error::missing_field("evidence_versions"))?;
                let explanation =
                    explanation.ok_or_else(|| M::Error::missing_field("explanation"))?;
                let owner_type = owner_type.ok_or_else(|| M::Error::missing_field("owner_type"))?;
                let sources = sources.ok_or_else(|| M::Error::missing_field("sources"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = OwnershipInferenceAttributes {
                    checksum,
                    confidence,
                    created_at,
                    evidence_versions,
                    explanation,
                    owner_type,
                    primary_contact_ref,
                    sources,
                    status,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OwnershipInferenceAttributesVisitor)
    }
}

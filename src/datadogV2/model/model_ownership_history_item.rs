// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single ownership inference history entry.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OwnershipHistoryItem {
    /// A checksum identifying the state of the inference at this point in time.
    #[serde(rename = "checksum")]
    pub checksum: String,
    /// The confidence score of the inference, expressed as a numeric string with up to four decimal places.
    #[serde(rename = "confidence")]
    pub confidence: String,
    /// The time this history entry was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The list of evidence versions associated with an inference.
    #[serialize_always]
    #[serde(rename = "evidence_versions")]
    pub evidence_versions: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// A human-readable explanation of how the inference was produced.
    #[serde(rename = "explanation")]
    pub explanation: String,
    /// The time when this inference failed, if applicable.
    #[serde(
        rename = "failed_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub failed_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The reason why this inference failed, if applicable.
    #[serde(
        rename = "failure_reason",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub failure_reason: Option<Option<String>>,
    /// The unique identifier of the history entry.
    #[serde(rename = "id")]
    pub id: i64,
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
    /// The identifier of the resource that the inference applies to.
    #[serde(rename = "resource_id")]
    pub resource_id: String,
    /// The scheduled retry time for a failed inference, if applicable.
    #[serde(
        rename = "retry_schedule",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub retry_schedule: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The list of sources backing an ownership inference. Empty when the inference status is not whitelisted to expose sources.
    #[serde(rename = "sources")]
    pub sources: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The lifecycle status of an ownership inference.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::OwnershipInferenceStatus,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OwnershipHistoryItem {
    pub fn new(
        checksum: String,
        confidence: String,
        created_at: chrono::DateTime<chrono::Utc>,
        evidence_versions: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
        explanation: String,
        id: i64,
        owner_type: crate::datadogV2::model::OwnershipOwnerType,
        resource_id: String,
        sources: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
        status: crate::datadogV2::model::OwnershipInferenceStatus,
    ) -> OwnershipHistoryItem {
        OwnershipHistoryItem {
            checksum,
            confidence,
            created_at,
            evidence_versions,
            explanation,
            failed_at: None,
            failure_reason: None,
            id,
            owner_type,
            primary_contact_ref: None,
            resource_id,
            retry_schedule: None,
            sources,
            status,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn failed_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.failed_at = Some(value);
        self
    }

    pub fn failure_reason(mut self, value: Option<String>) -> Self {
        self.failure_reason = Some(value);
        self
    }

    pub fn primary_contact_ref(mut self, value: Option<String>) -> Self {
        self.primary_contact_ref = Some(value);
        self
    }

    pub fn retry_schedule(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.retry_schedule = Some(value);
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

impl<'de> Deserialize<'de> for OwnershipHistoryItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OwnershipHistoryItemVisitor;
        impl<'a> Visitor<'a> for OwnershipHistoryItemVisitor {
            type Value = OwnershipHistoryItem;

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
                let mut failed_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut failure_reason: Option<Option<String>> = None;
                let mut id: Option<i64> = None;
                let mut owner_type: Option<crate::datadogV2::model::OwnershipOwnerType> = None;
                let mut primary_contact_ref: Option<Option<String>> = None;
                let mut resource_id: Option<String> = None;
                let mut retry_schedule: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut sources: Option<
                    Vec<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut status: Option<crate::datadogV2::model::OwnershipInferenceStatus> = None;
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
                        "failed_at" => {
                            failed_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "failure_reason" => {
                            failure_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "resource_id" => {
                            resource_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retry_schedule" => {
                            retry_schedule =
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
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let owner_type = owner_type.ok_or_else(|| M::Error::missing_field("owner_type"))?;
                let resource_id =
                    resource_id.ok_or_else(|| M::Error::missing_field("resource_id"))?;
                let sources = sources.ok_or_else(|| M::Error::missing_field("sources"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = OwnershipHistoryItem {
                    checksum,
                    confidence,
                    created_at,
                    evidence_versions,
                    explanation,
                    failed_at,
                    failure_reason,
                    id,
                    owner_type,
                    primary_contact_ref,
                    resource_id,
                    retry_schedule,
                    sources,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OwnershipHistoryItemVisitor)
    }
}

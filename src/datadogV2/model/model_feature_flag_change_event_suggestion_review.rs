// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Review details of a change request that needed approval.
/// Present on review events, which `include_suggestion_events` adds.
/// Also present on a change that an approved change request applied, even when `include_suggestion_events` is `false`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FeatureFlagChangeEventSuggestionReview {
    /// Where the change request is in its lifecycle.
    #[serde(rename = "lifecycle_status")]
    pub lifecycle_status: crate::datadogV2::model::FeatureFlagChangeEventSuggestionLifecycleStatus,
    /// The comment left by the person who requested the change.
    #[serde(rename = "requester_comment")]
    pub requester_comment: Option<String>,
    /// The comment left by the reviewer.
    #[serde(rename = "reviewer_comment")]
    pub reviewer_comment: Option<String>,
    /// The review status of the change request.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::FeatureFlagChangeEventSuggestionStatus,
    /// The ID of the user who requested the change.
    #[serde(rename = "suggestion_created_by")]
    pub suggestion_created_by: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FeatureFlagChangeEventSuggestionReview {
    pub fn new(
        lifecycle_status: crate::datadogV2::model::FeatureFlagChangeEventSuggestionLifecycleStatus,
        status: crate::datadogV2::model::FeatureFlagChangeEventSuggestionStatus,
    ) -> FeatureFlagChangeEventSuggestionReview {
        FeatureFlagChangeEventSuggestionReview {
            lifecycle_status,
            requester_comment: None,
            reviewer_comment: None,
            status,
            suggestion_created_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn requester_comment(mut self, value: String) -> Self {
        self.requester_comment = Some(value);
        self
    }

    pub fn reviewer_comment(mut self, value: String) -> Self {
        self.reviewer_comment = Some(value);
        self
    }

    pub fn suggestion_created_by(mut self, value: String) -> Self {
        self.suggestion_created_by = Some(value);
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

impl<'de> Deserialize<'de> for FeatureFlagChangeEventSuggestionReview {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FeatureFlagChangeEventSuggestionReviewVisitor;
        impl<'a> Visitor<'a> for FeatureFlagChangeEventSuggestionReviewVisitor {
            type Value = FeatureFlagChangeEventSuggestionReview;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut lifecycle_status: Option<
                    crate::datadogV2::model::FeatureFlagChangeEventSuggestionLifecycleStatus,
                > = None;
                let mut requester_comment: Option<String> = None;
                let mut reviewer_comment: Option<String> = None;
                let mut status: Option<
                    crate::datadogV2::model::FeatureFlagChangeEventSuggestionStatus,
                > = None;
                let mut suggestion_created_by: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "lifecycle_status" => {
                            lifecycle_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _lifecycle_status) = lifecycle_status {
                                match _lifecycle_status {
                                    crate::datadogV2::model::FeatureFlagChangeEventSuggestionLifecycleStatus::UnparsedObject(_lifecycle_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "requester_comment" => {
                            if v.is_null() {
                                continue;
                            }
                            requester_comment =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reviewer_comment" => {
                            if v.is_null() {
                                continue;
                            }
                            reviewer_comment =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::FeatureFlagChangeEventSuggestionStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "suggestion_created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            suggestion_created_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let lifecycle_status =
                    lifecycle_status.ok_or_else(|| M::Error::missing_field("lifecycle_status"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = FeatureFlagChangeEventSuggestionReview {
                    lifecycle_status,
                    requester_comment,
                    reviewer_comment,
                    status,
                    suggestion_created_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FeatureFlagChangeEventSuggestionReviewVisitor)
    }
}

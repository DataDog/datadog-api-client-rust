// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// One or more feature flags that reference this saved filter require approval for the rule update.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SavedFilterApprovalConflictAttributes {
    /// Feature flags preventing this update.
    #[serde(rename = "approval_required_flags")]
    pub approval_required_flags: Vec<crate::datadogV2::model::SavedFilterFlagReference>,
    /// The reason the saved filter update was rejected.
    #[serde(rename = "message")]
    pub message: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SavedFilterApprovalConflictAttributes {
    pub fn new(
        approval_required_flags: Vec<crate::datadogV2::model::SavedFilterFlagReference>,
        message: String,
    ) -> SavedFilterApprovalConflictAttributes {
        SavedFilterApprovalConflictAttributes {
            approval_required_flags,
            message,
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

impl<'de> Deserialize<'de> for SavedFilterApprovalConflictAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SavedFilterApprovalConflictAttributesVisitor;
        impl<'a> Visitor<'a> for SavedFilterApprovalConflictAttributesVisitor {
            type Value = SavedFilterApprovalConflictAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut approval_required_flags: Option<
                    Vec<crate::datadogV2::model::SavedFilterFlagReference>,
                > = None;
                let mut message: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "approval_required_flags" => {
                            approval_required_flags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let approval_required_flags = approval_required_flags
                    .ok_or_else(|| M::Error::missing_field("approval_required_flags"))?;
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;

                let content = SavedFilterApprovalConflictAttributes {
                    approval_required_flags,
                    message,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SavedFilterApprovalConflictAttributesVisitor)
    }
}

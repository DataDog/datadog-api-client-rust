// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HamrOrgConnectionAttributesResponse {
    /// Status of the HAMR connection:
    /// - 0: UNSPECIFIED - Connection status not specified
    /// - 1: ONBOARDING - Initial setup of HAMR connection
    /// - 2: PASSIVE - Secondary organization in passive standby mode
    /// - 3: FAILOVER - Liminal status between PASSIVE and ACTIVE
    /// - 4: ACTIVE - Organization is an active failover
    /// - 5: RECOVERY - Recovery operation in progress
    #[serde(rename = "hamr_status")]
    pub hamr_status: crate::datadogV2::model::HamrOrgConnectionStatus,
    /// Indicates whether this organization is the primary organization in the HAMR relationship.
    /// If true, this is the primary organization. If false, this is the secondary/backup organization.
    #[serde(rename = "is_primary")]
    pub is_primary: bool,
    /// Timestamp of when this HAMR connection was last modified (RFC3339 format).
    #[serde(rename = "modified_at")]
    pub modified_at: String,
    /// Username or identifier of the user who last modified this HAMR connection.
    #[serde(rename = "modified_by")]
    pub modified_by: String,
    /// Datacenter location of the target organization (e.g., us1, eu1, us5).
    #[serde(rename = "target_org_datacenter")]
    pub target_org_datacenter: String,
    /// Name of the target organization in the HAMR relationship.
    #[serde(rename = "target_org_name")]
    pub target_org_name: String,
    /// UUID of the target organization in the HAMR relationship.
    #[serde(rename = "target_org_uuid")]
    pub target_org_uuid: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HamrOrgConnectionAttributesResponse {
    pub fn new(
        hamr_status: crate::datadogV2::model::HamrOrgConnectionStatus,
        is_primary: bool,
        modified_at: String,
        modified_by: String,
        target_org_datacenter: String,
        target_org_name: String,
        target_org_uuid: String,
    ) -> HamrOrgConnectionAttributesResponse {
        HamrOrgConnectionAttributesResponse {
            hamr_status,
            is_primary,
            modified_at,
            modified_by,
            target_org_datacenter,
            target_org_name,
            target_org_uuid,
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

impl<'de> Deserialize<'de> for HamrOrgConnectionAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HamrOrgConnectionAttributesResponseVisitor;
        impl<'a> Visitor<'a> for HamrOrgConnectionAttributesResponseVisitor {
            type Value = HamrOrgConnectionAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut hamr_status: Option<crate::datadogV2::model::HamrOrgConnectionStatus> =
                    None;
                let mut is_primary: Option<bool> = None;
                let mut modified_at: Option<String> = None;
                let mut modified_by: Option<String> = None;
                let mut target_org_datacenter: Option<String> = None;
                let mut target_org_name: Option<String> = None;
                let mut target_org_uuid: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hamr_status" => {
                            hamr_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _hamr_status) = hamr_status {
                                match _hamr_status {
                                    crate::datadogV2::model::HamrOrgConnectionStatus::UnparsedObject(_hamr_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "is_primary" => {
                            is_primary = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_by" => {
                            modified_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target_org_datacenter" => {
                            target_org_datacenter =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target_org_name" => {
                            target_org_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target_org_uuid" => {
                            target_org_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let hamr_status =
                    hamr_status.ok_or_else(|| M::Error::missing_field("hamr_status"))?;
                let is_primary = is_primary.ok_or_else(|| M::Error::missing_field("is_primary"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let modified_by =
                    modified_by.ok_or_else(|| M::Error::missing_field("modified_by"))?;
                let target_org_datacenter = target_org_datacenter
                    .ok_or_else(|| M::Error::missing_field("target_org_datacenter"))?;
                let target_org_name =
                    target_org_name.ok_or_else(|| M::Error::missing_field("target_org_name"))?;
                let target_org_uuid =
                    target_org_uuid.ok_or_else(|| M::Error::missing_field("target_org_uuid"))?;

                let content = HamrOrgConnectionAttributesResponse {
                    hamr_status,
                    is_primary,
                    modified_at,
                    modified_by,
                    target_org_datacenter,
                    target_org_name,
                    target_org_uuid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HamrOrgConnectionAttributesResponseVisitor)
    }
}

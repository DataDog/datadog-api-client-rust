// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Storage Management configuration data for the create or update request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpsertCloudInventorySyncConfigRequestData {
    /// Settings for the cloud provider specified in data.id. Include only the matching provider object (aws, gcp, or azure).
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::UpsertCloudInventorySyncConfigRequestAttributes,
    /// Cloud provider for this sync configuration (`aws`, `gcp`, or `azure`). For requests, must match the provider block supplied under `attributes`.
    #[serde(rename = "id")]
    pub id: crate::datadogV2::model::CloudInventoryCloudProviderId,
    /// Always cloud_provider.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::CloudInventoryCloudProviderRequestType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpsertCloudInventorySyncConfigRequestData {
    pub fn new(
        attributes: crate::datadogV2::model::UpsertCloudInventorySyncConfigRequestAttributes,
        id: crate::datadogV2::model::CloudInventoryCloudProviderId,
        type_: crate::datadogV2::model::CloudInventoryCloudProviderRequestType,
    ) -> UpsertCloudInventorySyncConfigRequestData {
        UpsertCloudInventorySyncConfigRequestData {
            attributes,
            id,
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

impl<'de> Deserialize<'de> for UpsertCloudInventorySyncConfigRequestData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpsertCloudInventorySyncConfigRequestDataVisitor;
        impl<'a> Visitor<'a> for UpsertCloudInventorySyncConfigRequestDataVisitor {
            type Value = UpsertCloudInventorySyncConfigRequestData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::UpsertCloudInventorySyncConfigRequestAttributes,
                > = None;
                let mut id: Option<crate::datadogV2::model::CloudInventoryCloudProviderId> = None;
                let mut type_: Option<
                    crate::datadogV2::model::CloudInventoryCloudProviderRequestType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _id) = id {
                                match _id {
                                    crate::datadogV2::model::CloudInventoryCloudProviderId::UnparsedObject(_id) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CloudInventoryCloudProviderRequestType::UnparsedObject(_type_) => {
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
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = UpsertCloudInventorySyncConfigRequestData {
                    attributes,
                    id,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpsertCloudInventorySyncConfigRequestDataVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an agentless host.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CsmAgentlessHostAttributes {
    /// The ID of the cloud account that the host belongs to.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The cloud provider of a host resource.
    #[serde(rename = "cloud_provider")]
    pub cloud_provider: crate::datadogV2::model::CsmCloudProvider,
    /// Whether CSM Misconfigurations is enabled for this host. `true` if enabled; `false` if disabled.
    #[serde(rename = "has_posture_management")]
    pub has_posture_management: bool,
    /// Whether CSM Vulnerabilities is enabled for this host. `true` if enabled; `false` if disabled.
    #[serde(rename = "has_vulnerability_scanning")]
    pub has_vulnerability_scanning: bool,
    /// The type of cloud resource for an agentless host.
    #[serde(rename = "resource_type")]
    pub resource_type: crate::datadogV2::model::CsmAgentlessHostResourceType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CsmAgentlessHostAttributes {
    pub fn new(
        account_id: String,
        cloud_provider: crate::datadogV2::model::CsmCloudProvider,
        has_posture_management: bool,
        has_vulnerability_scanning: bool,
        resource_type: crate::datadogV2::model::CsmAgentlessHostResourceType,
    ) -> CsmAgentlessHostAttributes {
        CsmAgentlessHostAttributes {
            account_id,
            cloud_provider,
            has_posture_management,
            has_vulnerability_scanning,
            resource_type,
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

impl<'de> Deserialize<'de> for CsmAgentlessHostAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CsmAgentlessHostAttributesVisitor;
        impl<'a> Visitor<'a> for CsmAgentlessHostAttributesVisitor {
            type Value = CsmAgentlessHostAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut cloud_provider: Option<crate::datadogV2::model::CsmCloudProvider> = None;
                let mut has_posture_management: Option<bool> = None;
                let mut has_vulnerability_scanning: Option<bool> = None;
                let mut resource_type: Option<
                    crate::datadogV2::model::CsmAgentlessHostResourceType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_provider" => {
                            cloud_provider =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _cloud_provider) = cloud_provider {
                                match _cloud_provider {
                                    crate::datadogV2::model::CsmCloudProvider::UnparsedObject(
                                        _cloud_provider,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "has_posture_management" => {
                            has_posture_management =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_vulnerability_scanning" => {
                            has_vulnerability_scanning =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_type" => {
                            resource_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _resource_type) = resource_type {
                                match _resource_type {
                                    crate::datadogV2::model::CsmAgentlessHostResourceType::UnparsedObject(_resource_type) => {
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
                let account_id = account_id.ok_or_else(|| M::Error::missing_field("account_id"))?;
                let cloud_provider =
                    cloud_provider.ok_or_else(|| M::Error::missing_field("cloud_provider"))?;
                let has_posture_management = has_posture_management
                    .ok_or_else(|| M::Error::missing_field("has_posture_management"))?;
                let has_vulnerability_scanning = has_vulnerability_scanning
                    .ok_or_else(|| M::Error::missing_field("has_vulnerability_scanning"))?;
                let resource_type =
                    resource_type.ok_or_else(|| M::Error::missing_field("resource_type"))?;

                let content = CsmAgentlessHostAttributes {
                    account_id,
                    cloud_provider,
                    has_posture_management,
                    has_vulnerability_scanning,
                    resource_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CsmAgentlessHostAttributesVisitor)
    }
}

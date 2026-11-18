// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Writable attributes used to update a Nebius Cloud integration account. Every field is optional; only the fields provided are changed. When `dataflows` is provided, only the dataflow ids included in the request are modified; dataflows omitted from the map keep their current configuration, as do the settings of an included dataflow that provides only `enabled`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NebiusCloudIntegrationAccountUpdateAttributes {
    /// Authentication for updating the Nebius Cloud integration account. Exactly one method is set.
    #[serde(rename = "authentication")]
    pub authentication:
        Option<crate::datadogV2::model::NebiusCloudIntegrationAccountAuthenticationUpdate>,
    /// Data Datadog collects from Nebius Cloud, keyed by dataflow id. Each dataflow turns on a distinct kind of collection: set `enabled` to stop or start it, and use `settings` to configure what it collects. Defaults listed on each dataflow apply when the account is created; on update, omitted fields keep their current values. Every dataflow except `nebius-cloud-cost-collection` runs as the service account in `authentication`, using its authorized key pair and the static API key; enabling one of them without those keys is rejected with a `422` response, and one enabled with keys that later expire is stored but collects no data.
    #[serde(rename = "dataflows")]
    pub dataflows: Option<crate::datadogV2::model::NebiusCloudIntegrationDataflowsRequest>,
    /// Human-readable name of the Nebius Cloud integration account.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Settings for updating the Nebius Cloud integration account. Only the fields provided are changed.
    #[serde(rename = "settings")]
    pub settings: Option<crate::datadogV2::model::NebiusCloudIntegrationAccountSettingsUpdate>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NebiusCloudIntegrationAccountUpdateAttributes {
    pub fn new() -> NebiusCloudIntegrationAccountUpdateAttributes {
        NebiusCloudIntegrationAccountUpdateAttributes {
            authentication: None,
            dataflows: None,
            name: None,
            settings: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn authentication(
        mut self,
        value: crate::datadogV2::model::NebiusCloudIntegrationAccountAuthenticationUpdate,
    ) -> Self {
        self.authentication = Some(value);
        self
    }

    pub fn dataflows(
        mut self,
        value: crate::datadogV2::model::NebiusCloudIntegrationDataflowsRequest,
    ) -> Self {
        self.dataflows = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn settings(
        mut self,
        value: crate::datadogV2::model::NebiusCloudIntegrationAccountSettingsUpdate,
    ) -> Self {
        self.settings = Some(value);
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

impl Default for NebiusCloudIntegrationAccountUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NebiusCloudIntegrationAccountUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NebiusCloudIntegrationAccountUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for NebiusCloudIntegrationAccountUpdateAttributesVisitor {
            type Value = NebiusCloudIntegrationAccountUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut authentication: Option<
                    crate::datadogV2::model::NebiusCloudIntegrationAccountAuthenticationUpdate,
                > = None;
                let mut dataflows: Option<
                    crate::datadogV2::model::NebiusCloudIntegrationDataflowsRequest,
                > = None;
                let mut name: Option<String> = None;
                let mut settings: Option<
                    crate::datadogV2::model::NebiusCloudIntegrationAccountSettingsUpdate,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "authentication" => {
                            if v.is_null() {
                                continue;
                            }
                            authentication =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _authentication) = authentication {
                                match _authentication {
                                    crate::datadogV2::model::NebiusCloudIntegrationAccountAuthenticationUpdate::UnparsedObject(_authentication) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "dataflows" => {
                            if v.is_null() {
                                continue;
                            }
                            dataflows = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "settings" => {
                            if v.is_null() {
                                continue;
                            }
                            settings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = NebiusCloudIntegrationAccountUpdateAttributes {
                    authentication,
                    dataflows,
                    name,
                    settings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NebiusCloudIntegrationAccountUpdateAttributesVisitor)
    }
}

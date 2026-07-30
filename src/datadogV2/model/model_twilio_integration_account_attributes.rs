// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a Twilio integration account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TwilioIntegrationAccountAttributes {
    /// Twilio interface (source-type) configuration.
    #[serde(rename = "interface")]
    pub interface: crate::datadogV2::model::TwilioInterface,
    /// Human-readable name of the account.
    #[serde(rename = "name")]
    pub name: String,
    /// Read-only permission information for the account, derived from its restriction policy.
    #[serde(rename = "permissions")]
    pub permissions: Option<crate::datadogV2::model::IntegrationAccountPermissions>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TwilioIntegrationAccountAttributes {
    pub fn new(
        interface: crate::datadogV2::model::TwilioInterface,
        name: String,
    ) -> TwilioIntegrationAccountAttributes {
        TwilioIntegrationAccountAttributes {
            interface,
            name,
            permissions: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn permissions(
        mut self,
        value: crate::datadogV2::model::IntegrationAccountPermissions,
    ) -> Self {
        self.permissions = Some(value);
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

impl<'de> Deserialize<'de> for TwilioIntegrationAccountAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TwilioIntegrationAccountAttributesVisitor;
        impl<'a> Visitor<'a> for TwilioIntegrationAccountAttributesVisitor {
            type Value = TwilioIntegrationAccountAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut interface: Option<crate::datadogV2::model::TwilioInterface> = None;
                let mut name: Option<String> = None;
                let mut permissions: Option<
                    crate::datadogV2::model::IntegrationAccountPermissions,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "interface" => {
                            interface = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "permissions" => {
                            if v.is_null() {
                                continue;
                            }
                            permissions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let interface = interface.ok_or_else(|| M::Error::missing_field("interface"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = TwilioIntegrationAccountAttributes {
                    interface,
                    name,
                    permissions,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TwilioIntegrationAccountAttributesVisitor)
    }
}

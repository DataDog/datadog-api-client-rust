// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Organization information for a global organization association.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GlobalOrg {
    /// The name of the organization.
    #[serde(rename = "name")]
    pub name: String,
    /// The public identifier of the organization.
    #[serde(
        rename = "public_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub public_id: Option<Option<String>>,
    /// The subdomain used to access the organization, if configured.
    #[serde(
        rename = "subdomain",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub subdomain: Option<Option<String>>,
    /// The UUID of the organization.
    #[serde(rename = "uuid")]
    pub uuid: uuid::Uuid,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GlobalOrg {
    pub fn new(name: String, uuid: uuid::Uuid) -> GlobalOrg {
        GlobalOrg {
            name,
            public_id: None,
            subdomain: None,
            uuid,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn public_id(mut self, value: Option<String>) -> Self {
        self.public_id = Some(value);
        self
    }

    pub fn subdomain(mut self, value: Option<String>) -> Self {
        self.subdomain = Some(value);
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

impl<'de> Deserialize<'de> for GlobalOrg {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GlobalOrgVisitor;
        impl<'a> Visitor<'a> for GlobalOrgVisitor {
            type Value = GlobalOrg;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut public_id: Option<Option<String>> = None;
                let mut subdomain: Option<Option<String>> = None;
                let mut uuid: Option<uuid::Uuid> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subdomain" => {
                            subdomain = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uuid" => {
                            uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let uuid = uuid.ok_or_else(|| M::Error::missing_field("uuid"))?;

                let content = GlobalOrg {
                    name,
                    public_id,
                    subdomain,
                    uuid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GlobalOrgVisitor)
    }
}

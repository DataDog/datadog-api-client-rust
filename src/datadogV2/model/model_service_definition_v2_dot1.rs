// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Service definition v2.1 for providing service metadata and integrations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceDefinitionV2Dot1 {
    /// Identifier for a group of related services serving a product feature, which the service is a part of.
    #[serde(rename = "application")]
    pub application: Option<String>,
    /// A list of contacts related to the services.
    #[serde(rename = "contacts")]
    pub contacts: Option<Vec<crate::datadogV2::model::ServiceDefinitionV2Dot1Contact>>,
    /// Unique identifier of the service. Must be unique across all services and is used to match with a service in Datadog.
    #[serde(rename = "dd-service")]
    pub dd_service: String,
    /// A short description of the service.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Extensions to v2.1 schema.
    #[serde(rename = "extensions")]
    pub extensions: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Third party integrations that Datadog supports.
    #[serde(rename = "integrations")]
    pub integrations: Option<crate::datadogV2::model::ServiceDefinitionV2Dot1Integrations>,
    /// The current life cycle phase of the service.
    #[serde(rename = "lifecycle")]
    pub lifecycle: Option<String>,
    /// A list of links related to the services.
    #[serde(rename = "links")]
    pub links: Option<Vec<crate::datadogV2::model::ServiceDefinitionV2Dot1Link>>,
    /// Schema version being used.
    #[serde(rename = "schema-version")]
    pub schema_version: crate::datadogV2::model::ServiceDefinitionV2Dot1Version,
    /// A set of custom tags.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Team that owns the service. It is used to locate a team defined in Datadog Teams if it exists.
    #[serde(rename = "team")]
    pub team: Option<String>,
    /// Importance of the service.
    #[serde(rename = "tier")]
    pub tier: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceDefinitionV2Dot1 {
    pub fn new(
        dd_service: String,
        schema_version: crate::datadogV2::model::ServiceDefinitionV2Dot1Version,
    ) -> ServiceDefinitionV2Dot1 {
        ServiceDefinitionV2Dot1 {
            application: None,
            contacts: None,
            dd_service,
            description: None,
            extensions: None,
            integrations: None,
            lifecycle: None,
            links: None,
            schema_version,
            tags: None,
            team: None,
            tier: None,
            _unparsed: false,
        }
    }

    pub fn application(&mut self, value: String) -> &mut Self {
        self.application = Some(value);
        self
    }

    pub fn contacts(
        &mut self,
        value: Vec<crate::datadogV2::model::ServiceDefinitionV2Dot1Contact>,
    ) -> &mut Self {
        self.contacts = Some(value);
        self
    }

    pub fn description(&mut self, value: String) -> &mut Self {
        self.description = Some(value);
        self
    }

    pub fn extensions(
        &mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> &mut Self {
        self.extensions = Some(value);
        self
    }

    pub fn integrations(
        &mut self,
        value: crate::datadogV2::model::ServiceDefinitionV2Dot1Integrations,
    ) -> &mut Self {
        self.integrations = Some(value);
        self
    }

    pub fn lifecycle(&mut self, value: String) -> &mut Self {
        self.lifecycle = Some(value);
        self
    }

    pub fn links(
        &mut self,
        value: Vec<crate::datadogV2::model::ServiceDefinitionV2Dot1Link>,
    ) -> &mut Self {
        self.links = Some(value);
        self
    }

    pub fn tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn team(&mut self, value: String) -> &mut Self {
        self.team = Some(value);
        self
    }

    pub fn tier(&mut self, value: String) -> &mut Self {
        self.tier = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ServiceDefinitionV2Dot1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceDefinitionV2Dot1Visitor;
        impl<'a> Visitor<'a> for ServiceDefinitionV2Dot1Visitor {
            type Value = ServiceDefinitionV2Dot1;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application: Option<String> = None;
                let mut contacts: Option<
                    Vec<crate::datadogV2::model::ServiceDefinitionV2Dot1Contact>,
                > = None;
                let mut dd_service: Option<String> = None;
                let mut description: Option<String> = None;
                let mut extensions: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut integrations: Option<
                    crate::datadogV2::model::ServiceDefinitionV2Dot1Integrations,
                > = None;
                let mut lifecycle: Option<String> = None;
                let mut links: Option<Vec<crate::datadogV2::model::ServiceDefinitionV2Dot1Link>> =
                    None;
                let mut schema_version: Option<
                    crate::datadogV2::model::ServiceDefinitionV2Dot1Version,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut team: Option<String> = None;
                let mut tier: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "application" => {
                            if v.is_null() {
                                continue;
                            }
                            application =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "contacts" => {
                            if v.is_null() {
                                continue;
                            }
                            contacts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dd-service" => {
                            dd_service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "extensions" => {
                            if v.is_null() {
                                continue;
                            }
                            extensions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integrations" => {
                            if v.is_null() {
                                continue;
                            }
                            integrations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lifecycle" => {
                            if v.is_null() {
                                continue;
                            }
                            lifecycle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "links" => {
                            if v.is_null() {
                                continue;
                            }
                            links = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schema-version" => {
                            schema_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _schema_version) = schema_version {
                                match _schema_version {
                                    crate::datadogV2::model::ServiceDefinitionV2Dot1Version::UnparsedObject(_schema_version) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team" => {
                            if v.is_null() {
                                continue;
                            }
                            team = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tier" => {
                            if v.is_null() {
                                continue;
                            }
                            tier = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let dd_service = dd_service.ok_or_else(|| M::Error::missing_field("dd_service"))?;
                let schema_version =
                    schema_version.ok_or_else(|| M::Error::missing_field("schema_version"))?;

                let content = ServiceDefinitionV2Dot1 {
                    application,
                    contacts,
                    dd_service,
                    description,
                    extensions,
                    integrations,
                    lifecycle,
                    links,
                    schema_version,
                    tags,
                    team,
                    tier,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceDefinitionV2Dot1Visitor)
    }
}

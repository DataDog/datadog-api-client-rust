// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Service definition V2 for providing service metadata and integrations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceDefinitionV2 {
    /// A list of contacts related to the services.
    #[serde(rename = "contacts")]
    pub contacts: Option<Vec<crate::datadogV2::model::ServiceDefinitionV2Contact>>,
    /// Unique identifier of the service. Must be unique across all services and is used to match with a service in Datadog.
    #[serde(rename = "dd-service")]
    pub dd_service: String,
    /// Experimental feature. A Team handle that matches a Team in the Datadog Teams product.
    #[serde(rename = "dd-team")]
    pub dd_team: Option<String>,
    /// A list of documentation related to the services.
    #[serde(rename = "docs")]
    pub docs: Option<Vec<crate::datadogV2::model::ServiceDefinitionV2Doc>>,
    /// Extensions to V2 schema.
    #[serde(rename = "extensions")]
    pub extensions: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Third party integrations that Datadog supports.
    #[serde(rename = "integrations")]
    pub integrations: Option<crate::datadogV2::model::ServiceDefinitionV2Integrations>,
    /// A list of links related to the services.
    #[serde(rename = "links")]
    pub links: Option<Vec<crate::datadogV2::model::ServiceDefinitionV2Link>>,
    /// A list of code repositories related to the services.
    #[serde(rename = "repos")]
    pub repos: Option<Vec<crate::datadogV2::model::ServiceDefinitionV2Repo>>,
    /// Schema version being used.
    #[serde(rename = "schema-version")]
    pub schema_version: crate::datadogV2::model::ServiceDefinitionV2Version,
    /// A set of custom tags.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Team that owns the service.
    #[serde(rename = "team")]
    pub team: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceDefinitionV2 {
    pub fn new(
        dd_service: String,
        schema_version: crate::datadogV2::model::ServiceDefinitionV2Version,
    ) -> ServiceDefinitionV2 {
        ServiceDefinitionV2 {
            contacts: None,
            dd_service,
            dd_team: None,
            docs: None,
            extensions: None,
            integrations: None,
            links: None,
            repos: None,
            schema_version,
            tags: None,
            team: None,
            _unparsed: false,
        }
    }

    pub fn contacts(
        mut self,
        value: Vec<crate::datadogV2::model::ServiceDefinitionV2Contact>,
    ) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn dd_team(mut self, value: String) -> Self {
        self.dd_team = Some(value);
        self
    }

    pub fn docs(mut self, value: Vec<crate::datadogV2::model::ServiceDefinitionV2Doc>) -> Self {
        self.docs = Some(value);
        self
    }

    pub fn extensions(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.extensions = Some(value);
        self
    }

    pub fn integrations(
        mut self,
        value: crate::datadogV2::model::ServiceDefinitionV2Integrations,
    ) -> Self {
        self.integrations = Some(value);
        self
    }

    pub fn links(mut self, value: Vec<crate::datadogV2::model::ServiceDefinitionV2Link>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn repos(mut self, value: Vec<crate::datadogV2::model::ServiceDefinitionV2Repo>) -> Self {
        self.repos = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn team(mut self, value: String) -> Self {
        self.team = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ServiceDefinitionV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceDefinitionV2Visitor;
        impl<'a> Visitor<'a> for ServiceDefinitionV2Visitor {
            type Value = ServiceDefinitionV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut contacts: Option<Vec<crate::datadogV2::model::ServiceDefinitionV2Contact>> =
                    None;
                let mut dd_service: Option<String> = None;
                let mut dd_team: Option<String> = None;
                let mut docs: Option<Vec<crate::datadogV2::model::ServiceDefinitionV2Doc>> = None;
                let mut extensions: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut integrations: Option<
                    crate::datadogV2::model::ServiceDefinitionV2Integrations,
                > = None;
                let mut links: Option<Vec<crate::datadogV2::model::ServiceDefinitionV2Link>> = None;
                let mut repos: Option<Vec<crate::datadogV2::model::ServiceDefinitionV2Repo>> = None;
                let mut schema_version: Option<
                    crate::datadogV2::model::ServiceDefinitionV2Version,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut team: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "contacts" => {
                            if v.is_null() {
                                continue;
                            }
                            contacts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dd-service" => {
                            dd_service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dd-team" => {
                            if v.is_null() {
                                continue;
                            }
                            dd_team = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "docs" => {
                            if v.is_null() {
                                continue;
                            }
                            docs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "links" => {
                            if v.is_null() {
                                continue;
                            }
                            links = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repos" => {
                            if v.is_null() {
                                continue;
                            }
                            repos = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schema-version" => {
                            schema_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _schema_version) = schema_version {
                                match _schema_version {
                                    crate::datadogV2::model::ServiceDefinitionV2Version::UnparsedObject(_schema_version) => {
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
                        &_ => {}
                    }
                }
                let dd_service = dd_service.ok_or_else(|| M::Error::missing_field("dd_service"))?;
                let schema_version =
                    schema_version.ok_or_else(|| M::Error::missing_field("schema_version"))?;

                let content = ServiceDefinitionV2 {
                    contacts,
                    dd_service,
                    dd_team,
                    docs,
                    extensions,
                    integrations,
                    links,
                    repos,
                    schema_version,
                    tags,
                    team,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceDefinitionV2Visitor)
    }
}

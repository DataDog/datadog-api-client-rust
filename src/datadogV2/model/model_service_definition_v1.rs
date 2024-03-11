// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Deprecated - Service definition V1 for providing additional service metadata and integrations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceDefinitionV1 {
    /// Contact information about the service.
    #[serde(rename = "contact")]
    pub contact: Option<crate::datadogV2::model::ServiceDefinitionV1Contact>,
    /// Extensions to V1 schema.
    #[serde(rename = "extensions")]
    pub extensions: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// A list of external links related to the services.
    #[serde(rename = "external-resources")]
    pub external_resources: Option<Vec<crate::datadogV2::model::ServiceDefinitionV1Resource>>,
    /// Basic information about a service.
    #[serde(rename = "info")]
    pub info: crate::datadogV2::model::ServiceDefinitionV1Info,
    /// Third party integrations that Datadog supports.
    #[serde(rename = "integrations")]
    pub integrations: Option<crate::datadogV2::model::ServiceDefinitionV1Integrations>,
    /// Org related information about the service.
    #[serde(rename = "org")]
    pub org: Option<crate::datadogV2::model::ServiceDefinitionV1Org>,
    /// Schema version being used.
    #[serde(rename = "schema-version")]
    pub schema_version: crate::datadogV2::model::ServiceDefinitionV1Version,
    /// A set of custom tags.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceDefinitionV1 {
    pub fn new(
        info: crate::datadogV2::model::ServiceDefinitionV1Info,
        schema_version: crate::datadogV2::model::ServiceDefinitionV1Version,
    ) -> ServiceDefinitionV1 {
        ServiceDefinitionV1 {
            contact: None,
            extensions: None,
            external_resources: None,
            info,
            integrations: None,
            org: None,
            schema_version,
            tags: None,
            _unparsed: false,
        }
    }

    pub fn contact(
        &mut self,
        value: crate::datadogV2::model::ServiceDefinitionV1Contact,
    ) -> &mut Self {
        self.contact = Some(value);
        self
    }

    pub fn extensions(
        &mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> &mut Self {
        self.extensions = Some(value);
        self
    }

    pub fn external_resources(
        &mut self,
        value: Vec<crate::datadogV2::model::ServiceDefinitionV1Resource>,
    ) -> &mut Self {
        self.external_resources = Some(value);
        self
    }

    pub fn integrations(
        &mut self,
        value: crate::datadogV2::model::ServiceDefinitionV1Integrations,
    ) -> &mut Self {
        self.integrations = Some(value);
        self
    }

    pub fn org(&mut self, value: crate::datadogV2::model::ServiceDefinitionV1Org) -> &mut Self {
        self.org = Some(value);
        self
    }

    pub fn tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ServiceDefinitionV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceDefinitionV1Visitor;
        impl<'a> Visitor<'a> for ServiceDefinitionV1Visitor {
            type Value = ServiceDefinitionV1;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut contact: Option<crate::datadogV2::model::ServiceDefinitionV1Contact> = None;
                let mut extensions: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut external_resources: Option<
                    Vec<crate::datadogV2::model::ServiceDefinitionV1Resource>,
                > = None;
                let mut info: Option<crate::datadogV2::model::ServiceDefinitionV1Info> = None;
                let mut integrations: Option<
                    crate::datadogV2::model::ServiceDefinitionV1Integrations,
                > = None;
                let mut org: Option<crate::datadogV2::model::ServiceDefinitionV1Org> = None;
                let mut schema_version: Option<
                    crate::datadogV2::model::ServiceDefinitionV1Version,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "contact" => {
                            if v.is_null() {
                                continue;
                            }
                            contact = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "extensions" => {
                            if v.is_null() {
                                continue;
                            }
                            extensions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "external-resources" => {
                            if v.is_null() {
                                continue;
                            }
                            external_resources =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "info" => {
                            info = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integrations" => {
                            if v.is_null() {
                                continue;
                            }
                            integrations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org" => {
                            if v.is_null() {
                                continue;
                            }
                            org = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schema-version" => {
                            schema_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _schema_version) = schema_version {
                                match _schema_version {
                                    crate::datadogV2::model::ServiceDefinitionV1Version::UnparsedObject(_schema_version) => {
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
                        &_ => {}
                    }
                }
                let info = info.ok_or_else(|| M::Error::missing_field("info"))?;
                let schema_version =
                    schema_version.ok_or_else(|| M::Error::missing_field("schema_version"))?;

                let content = ServiceDefinitionV1 {
                    contact,
                    extensions,
                    external_resources,
                    info,
                    integrations,
                    org,
                    schema_version,
                    tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceDefinitionV1Visitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response object for an organization creation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrganizationCreateResponse {
    /// Datadog API key.
    #[serde(rename = "api_key")]
    pub api_key: Option<crate::datadogV1::model::ApiKey>,
    /// An application key with its associated metadata.
    #[serde(rename = "application_key")]
    pub application_key: Option<crate::datadogV1::model::ApplicationKey>,
    /// Create, edit, and manage organizations.
    #[serde(rename = "org")]
    pub org: Option<crate::datadogV1::model::Organization>,
    /// Create, edit, and disable users.
    #[serde(rename = "user")]
    pub user: Option<crate::datadogV1::model::User>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrganizationCreateResponse {
    pub fn new() -> OrganizationCreateResponse {
        OrganizationCreateResponse {
            api_key: None,
            application_key: None,
            org: None,
            user: None,
            _unparsed: false,
        }
    }

    pub fn api_key(mut self, value: crate::datadogV1::model::ApiKey) -> Self {
        self.api_key = Some(value);
        self
    }

    pub fn application_key(mut self, value: crate::datadogV1::model::ApplicationKey) -> Self {
        self.application_key = Some(value);
        self
    }

    pub fn org(mut self, value: crate::datadogV1::model::Organization) -> Self {
        self.org = Some(value);
        self
    }

    pub fn user(mut self, value: crate::datadogV1::model::User) -> Self {
        self.user = Some(value);
        self
    }
}

impl Default for OrganizationCreateResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OrganizationCreateResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrganizationCreateResponseVisitor;
        impl<'a> Visitor<'a> for OrganizationCreateResponseVisitor {
            type Value = OrganizationCreateResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_key: Option<crate::datadogV1::model::ApiKey> = None;
                let mut application_key: Option<crate::datadogV1::model::ApplicationKey> = None;
                let mut org: Option<crate::datadogV1::model::Organization> = None;
                let mut user: Option<crate::datadogV1::model::User> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "api_key" => {
                            if v.is_null() {
                                continue;
                            }
                            api_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "application_key" => {
                            if v.is_null() {
                                continue;
                            }
                            application_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org" => {
                            if v.is_null() {
                                continue;
                            }
                            org = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user" => {
                            if v.is_null() {
                                continue;
                            }
                            user = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = OrganizationCreateResponse {
                    api_key,
                    application_key,
                    org,
                    user,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrganizationCreateResponseVisitor)
    }
}

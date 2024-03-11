// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Team membership attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserTeamAttributes {
    /// The mechanism responsible for provisioning the team relationship.
    /// Possible values: null for added by a user, "service_account" if added by a service account, and "saml_mapping" if provisioned via SAML mapping.
    #[serde(
        rename = "provisioned_by",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub provisioned_by: Option<Option<String>>,
    /// UUID of the User or Service Account who provisioned this team membership, or null if provisioned via SAML mapping.
    #[serde(
        rename = "provisioned_by_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub provisioned_by_id: Option<Option<String>>,
    /// The user's role within the team
    #[serde(rename = "role", default, with = "::serde_with::rust::double_option")]
    pub role: Option<Option<crate::datadogV2::model::UserTeamRole>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserTeamAttributes {
    pub fn new() -> UserTeamAttributes {
        UserTeamAttributes {
            provisioned_by: None,
            provisioned_by_id: None,
            role: None,
            _unparsed: false,
        }
    }

    pub fn provisioned_by(&mut self, value: Option<String>) -> &mut Self {
        self.provisioned_by = Some(value);
        self
    }

    pub fn provisioned_by_id(&mut self, value: Option<String>) -> &mut Self {
        self.provisioned_by_id = Some(value);
        self
    }

    pub fn role(&mut self, value: Option<crate::datadogV2::model::UserTeamRole>) -> &mut Self {
        self.role = Some(value);
        self
    }
}

impl Default for UserTeamAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UserTeamAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserTeamAttributesVisitor;
        impl<'a> Visitor<'a> for UserTeamAttributesVisitor {
            type Value = UserTeamAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut provisioned_by: Option<Option<String>> = None;
                let mut provisioned_by_id: Option<Option<String>> = None;
                let mut role: Option<Option<crate::datadogV2::model::UserTeamRole>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "provisioned_by" => {
                            provisioned_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provisioned_by_id" => {
                            provisioned_by_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "role" => {
                            role = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _role) = role {
                                match _role {
                                    Some(
                                        crate::datadogV2::model::UserTeamRole::UnparsedObject(
                                            _role,
                                        ),
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = UserTeamAttributes {
                    provisioned_by,
                    provisioned_by_id,
                    role,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserTeamAttributesVisitor)
    }
}

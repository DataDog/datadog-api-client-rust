// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Create, edit, and manage organizations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Organization {
    /// A JSON array of billing type.
    #[deprecated]
    #[serde(rename = "billing")]
    pub billing: Option<crate::datadogV1::model::OrganizationBilling>,
    /// Date of the organization creation.
    #[serde(rename = "created")]
    pub created: Option<String>,
    /// Description of the organization.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The name of the child organization, limited to 32 characters.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The `public_id` of the organization you are operating within.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// A JSON array of settings.
    #[serde(rename = "settings")]
    pub settings: Option<crate::datadogV1::model::OrganizationSettings>,
    /// Subscription definition.
    #[deprecated]
    #[serde(rename = "subscription")]
    pub subscription: Option<crate::datadogV1::model::OrganizationSubscription>,
    /// Only available for MSP customers. Allows child organizations to be created on a trial plan.
    #[serde(rename = "trial")]
    pub trial: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl Organization {
    pub fn new() -> Organization {
        #[allow(deprecated)]
        Organization {
            billing: None,
            created: None,
            description: None,
            name: None,
            public_id: None,
            settings: None,
            subscription: None,
            trial: None,
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn billing(mut self, value: crate::datadogV1::model::OrganizationBilling) -> Self {
        self.billing = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn created(mut self, value: String) -> Self {
        self.created = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn settings(mut self, value: crate::datadogV1::model::OrganizationSettings) -> Self {
        self.settings = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn subscription(
        mut self,
        value: crate::datadogV1::model::OrganizationSubscription,
    ) -> Self {
        self.subscription = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn trial(mut self, value: bool) -> Self {
        self.trial = Some(value);
        self
    }
}

impl Default for Organization {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for Organization {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrganizationVisitor;
        impl<'a> Visitor<'a> for OrganizationVisitor {
            type Value = Organization;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut billing: Option<crate::datadogV1::model::OrganizationBilling> = None;
                let mut created: Option<String> = None;
                let mut description: Option<String> = None;
                let mut name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut settings: Option<crate::datadogV1::model::OrganizationSettings> = None;
                let mut subscription: Option<crate::datadogV1::model::OrganizationSubscription> =
                    None;
                let mut trial: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "billing" => {
                            if v.is_null() {
                                continue;
                            }
                            billing = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created" => {
                            if v.is_null() {
                                continue;
                            }
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "settings" => {
                            if v.is_null() {
                                continue;
                            }
                            settings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subscription" => {
                            if v.is_null() {
                                continue;
                            }
                            subscription =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trial" => {
                            if v.is_null() {
                                continue;
                            }
                            trial = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                #[allow(deprecated)]
                let content = Organization {
                    billing,
                    created,
                    description,
                    name,
                    public_id,
                    settings,
                    subscription,
                    trial,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrganizationVisitor)
    }
}

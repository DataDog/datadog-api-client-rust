// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing an organization to create.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrganizationCreateBody {
    /// A JSON array of billing type.
    #[deprecated]
    #[serde(rename = "billing")]
    pub billing: Option<crate::datadogV1::model::OrganizationBilling>,
    /// The name of the new child-organization, limited to 32 characters.
    #[serde(rename = "name")]
    pub name: String,
    /// Subscription definition.
    #[deprecated]
    #[serde(rename = "subscription")]
    pub subscription: Option<crate::datadogV1::model::OrganizationSubscription>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrganizationCreateBody {
    pub fn new(name: String) -> OrganizationCreateBody {
        #[allow(deprecated)]
        OrganizationCreateBody {
            billing: None,
            name,
            subscription: None,
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn billing(mut self, value: crate::datadogV1::model::OrganizationBilling) -> Self {
        self.billing = Some(value);
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
}

impl<'de> Deserialize<'de> for OrganizationCreateBody {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrganizationCreateBodyVisitor;
        impl<'a> Visitor<'a> for OrganizationCreateBodyVisitor {
            type Value = OrganizationCreateBody;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut billing: Option<crate::datadogV1::model::OrganizationBilling> = None;
                let mut name: Option<String> = None;
                let mut subscription: Option<crate::datadogV1::model::OrganizationSubscription> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "billing" => {
                            if v.is_null() {
                                continue;
                            }
                            billing = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subscription" => {
                            if v.is_null() {
                                continue;
                            }
                            subscription =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = OrganizationCreateBody {
                    billing,
                    name,
                    subscription,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrganizationCreateBodyVisitor)
    }
}

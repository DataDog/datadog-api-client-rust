// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata about the organization that sent the email.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TransportWebhookLogOrgMetadata {
    /// Country code or name used for billing purposes.
    #[serde(rename = "billing_country")]
    pub billing_country: Option<String>,
    /// The Datadog billing plan for the organization (for example, "pro", "enterprise").
    #[serde(rename = "billing_plan")]
    pub billing_plan: Option<String>,
    /// Support or account tier assigned to the organization (for example, "tier-1").
    #[serde(rename = "customer_tier")]
    pub customer_tier: Option<String>,
    /// Primary email domain associated with the organization (for example, "example.com").
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    /// Industry classification of the organization (for example, "technology", "finance").
    #[serde(rename = "industry")]
    pub industry: Option<String>,
    /// Whether the organization is enrolled in the Datadog bug bounty program.
    #[serde(rename = "is_bugbounty")]
    pub is_bugbounty: Option<String>,
    /// Whether the organization operates as a Managed Service Provider managing child orgs.
    #[serde(rename = "is_msp")]
    pub is_msp: Option<String>,
    /// Display name of the organization as configured in Datadog account settings.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Globally unique identifier for the Datadog organization (UUID v1 format).
    #[serde(rename = "org_uuid")]
    pub org_uuid: Option<String>,
    /// Identifier of the immediate parent organization, if this is a child org.
    #[serde(rename = "parent_org_id")]
    pub parent_org_id: Option<String>,
    /// Whether the organization has a premium support plan with Datadog.
    #[serde(rename = "premium_support")]
    pub premium_support: Option<String>,
    /// Identifier of the top-level parent organization in a multi-org account hierarchy.
    #[serde(rename = "root_org_id")]
    pub root_org_id: Option<String>,
    /// Display name of the top-level parent organization in a multi-org account hierarchy.
    #[serde(rename = "root_org_name")]
    pub root_org_name: Option<String>,
    /// Country code or name used for shipping or regional assignment.
    #[serde(rename = "shipping_country")]
    pub shipping_country: Option<String>,
    /// Website URL provided during organization registration.
    #[serde(rename = "website")]
    pub website: Option<String>,
    /// ISO 8601 timestamp of when the Datadog organization was created.
    #[serde(rename = "when_created")]
    pub when_created: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TransportWebhookLogOrgMetadata {
    pub fn new() -> TransportWebhookLogOrgMetadata {
        TransportWebhookLogOrgMetadata {
            billing_country: None,
            billing_plan: None,
            customer_tier: None,
            domain: None,
            industry: None,
            is_bugbounty: None,
            is_msp: None,
            name: None,
            org_uuid: None,
            parent_org_id: None,
            premium_support: None,
            root_org_id: None,
            root_org_name: None,
            shipping_country: None,
            website: None,
            when_created: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn billing_country(mut self, value: String) -> Self {
        self.billing_country = Some(value);
        self
    }

    pub fn billing_plan(mut self, value: String) -> Self {
        self.billing_plan = Some(value);
        self
    }

    pub fn customer_tier(mut self, value: String) -> Self {
        self.customer_tier = Some(value);
        self
    }

    pub fn domain(mut self, value: String) -> Self {
        self.domain = Some(value);
        self
    }

    pub fn industry(mut self, value: String) -> Self {
        self.industry = Some(value);
        self
    }

    pub fn is_bugbounty(mut self, value: String) -> Self {
        self.is_bugbounty = Some(value);
        self
    }

    pub fn is_msp(mut self, value: String) -> Self {
        self.is_msp = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn org_uuid(mut self, value: String) -> Self {
        self.org_uuid = Some(value);
        self
    }

    pub fn parent_org_id(mut self, value: String) -> Self {
        self.parent_org_id = Some(value);
        self
    }

    pub fn premium_support(mut self, value: String) -> Self {
        self.premium_support = Some(value);
        self
    }

    pub fn root_org_id(mut self, value: String) -> Self {
        self.root_org_id = Some(value);
        self
    }

    pub fn root_org_name(mut self, value: String) -> Self {
        self.root_org_name = Some(value);
        self
    }

    pub fn shipping_country(mut self, value: String) -> Self {
        self.shipping_country = Some(value);
        self
    }

    pub fn website(mut self, value: String) -> Self {
        self.website = Some(value);
        self
    }

    pub fn when_created(mut self, value: String) -> Self {
        self.when_created = Some(value);
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

impl Default for TransportWebhookLogOrgMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TransportWebhookLogOrgMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TransportWebhookLogOrgMetadataVisitor;
        impl<'a> Visitor<'a> for TransportWebhookLogOrgMetadataVisitor {
            type Value = TransportWebhookLogOrgMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut billing_country: Option<String> = None;
                let mut billing_plan: Option<String> = None;
                let mut customer_tier: Option<String> = None;
                let mut domain: Option<String> = None;
                let mut industry: Option<String> = None;
                let mut is_bugbounty: Option<String> = None;
                let mut is_msp: Option<String> = None;
                let mut name: Option<String> = None;
                let mut org_uuid: Option<String> = None;
                let mut parent_org_id: Option<String> = None;
                let mut premium_support: Option<String> = None;
                let mut root_org_id: Option<String> = None;
                let mut root_org_name: Option<String> = None;
                let mut shipping_country: Option<String> = None;
                let mut website: Option<String> = None;
                let mut when_created: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "billing_country" => {
                            if v.is_null() {
                                continue;
                            }
                            billing_country =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "billing_plan" => {
                            if v.is_null() {
                                continue;
                            }
                            billing_plan =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "customer_tier" => {
                            if v.is_null() {
                                continue;
                            }
                            customer_tier =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "domain" => {
                            if v.is_null() {
                                continue;
                            }
                            domain = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "industry" => {
                            if v.is_null() {
                                continue;
                            }
                            industry = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_bugbounty" => {
                            if v.is_null() {
                                continue;
                            }
                            is_bugbounty =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_msp" => {
                            if v.is_null() {
                                continue;
                            }
                            is_msp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            org_uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parent_org_id" => {
                            if v.is_null() {
                                continue;
                            }
                            parent_org_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "premium_support" => {
                            if v.is_null() {
                                continue;
                            }
                            premium_support =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "root_org_id" => {
                            if v.is_null() {
                                continue;
                            }
                            root_org_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "root_org_name" => {
                            if v.is_null() {
                                continue;
                            }
                            root_org_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "shipping_country" => {
                            if v.is_null() {
                                continue;
                            }
                            shipping_country =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "website" => {
                            if v.is_null() {
                                continue;
                            }
                            website = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "when_created" => {
                            if v.is_null() {
                                continue;
                            }
                            when_created =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TransportWebhookLogOrgMetadata {
                    billing_country,
                    billing_plan,
                    customer_tier,
                    domain,
                    industry,
                    is_bugbounty,
                    is_msp,
                    name,
                    org_uuid,
                    parent_org_id,
                    premium_support,
                    root_org_id,
                    root_org_name,
                    shipping_country,
                    website,
                    when_created,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TransportWebhookLogOrgMetadataVisitor)
    }
}

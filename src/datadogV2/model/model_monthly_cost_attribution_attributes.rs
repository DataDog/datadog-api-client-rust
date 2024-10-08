// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Cost Attribution by Tag for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonthlyCostAttributionAttributes {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]`.
    #[serde(rename = "month")]
    pub month: Option<chrono::DateTime<chrono::Utc>>,
    /// The name of the organization.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// The source of the cost attribution tag configuration and the selected tags in the format `<source_org_name>:::<selected tag 1>///<selected tag 2>///<selected tag 3>`.
    #[serde(rename = "tag_config_source")]
    pub tag_config_source: Option<String>,
    /// Tag keys and values.
    /// A `null` value here means that the requested tag breakdown cannot be applied because it does not match the [tags
    /// configured for usage attribution](<https://docs.datadoghq.com/account_management/billing/usage_attribution/#getting-started>).
    /// In this scenario the API returns the total cost, not broken down by tags.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<std::collections::BTreeMap<String, Vec<String>>>>,
    /// Shows the most recent hour in the current months for all organizations for which all costs were calculated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    /// Fields in Cost Attribution by tag(s). Example: `infra_host_on_demand_cost`, `infra_host_committed_cost`, `infra_host_total_cost`, `infra_host_percentage_in_org`, `infra_host_percentage_in_account`.
    #[serde(rename = "values")]
    pub values: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonthlyCostAttributionAttributes {
    pub fn new() -> MonthlyCostAttributionAttributes {
        MonthlyCostAttributionAttributes {
            month: None,
            org_name: None,
            public_id: None,
            tag_config_source: None,
            tags: None,
            updated_at: None,
            values: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn month(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.month = Some(value);
        self
    }

    pub fn org_name(mut self, value: String) -> Self {
        self.org_name = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }

    pub fn tag_config_source(mut self, value: String) -> Self {
        self.tag_config_source = Some(value);
        self
    }

    pub fn tags(mut self, value: Option<std::collections::BTreeMap<String, Vec<String>>>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn updated_at(mut self, value: String) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn values(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.values = Some(value);
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

impl Default for MonthlyCostAttributionAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonthlyCostAttributionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonthlyCostAttributionAttributesVisitor;
        impl<'a> Visitor<'a> for MonthlyCostAttributionAttributesVisitor {
            type Value = MonthlyCostAttributionAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut month: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut tag_config_source: Option<String> = None;
                let mut tags: Option<Option<std::collections::BTreeMap<String, Vec<String>>>> =
                    None;
                let mut updated_at: Option<String> = None;
                let mut values: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "month" => {
                            if v.is_null() {
                                continue;
                            }
                            month = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_name" => {
                            if v.is_null() {
                                continue;
                            }
                            org_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_config_source" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_config_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "values" => {
                            if v.is_null() {
                                continue;
                            }
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = MonthlyCostAttributionAttributes {
                    month,
                    org_name,
                    public_id,
                    tag_config_source,
                    tags,
                    updated_at,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonthlyCostAttributionAttributesVisitor)
    }
}

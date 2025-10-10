// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ArbitraryRuleResponseDataAttributes` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ArbitraryRuleResponseDataAttributes {
    /// The `attributes` `costs_to_allocate`.
    #[serde(rename = "costs_to_allocate")]
    pub costs_to_allocate:
        Vec<crate::datadogV2::model::ArbitraryRuleResponseDataAttributesCostsToAllocateItems>,
    /// The `attributes` `created`.
    #[serde(rename = "created")]
    pub created: chrono::DateTime<chrono::Utc>,
    /// The `attributes` `enabled`.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The `attributes` `last_modified_user_uuid`.
    #[serde(rename = "last_modified_user_uuid")]
    pub last_modified_user_uuid: String,
    /// The `attributes` `order_id`.
    #[serde(rename = "order_id")]
    pub order_id: i64,
    /// The `attributes` `processing_status`.
    #[serde(rename = "processing_status")]
    pub processing_status: Option<String>,
    /// The `attributes` `provider`.
    #[serde(rename = "provider")]
    pub provider: Vec<String>,
    /// The `attributes` `rejected`.
    #[serde(rename = "rejected")]
    pub rejected: Option<bool>,
    /// The `attributes` `rule_name`.
    #[serde(rename = "rule_name")]
    pub rule_name: String,
    /// The definition of `ArbitraryRuleResponseDataAttributesStrategy` object.
    #[serde(rename = "strategy")]
    pub strategy: crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategy,
    /// The `attributes` `type`.
    #[serde(rename = "type")]
    pub type_: String,
    /// The `attributes` `updated`.
    #[serde(rename = "updated")]
    pub updated: chrono::DateTime<chrono::Utc>,
    /// The `attributes` `version`.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ArbitraryRuleResponseDataAttributes {
    pub fn new(
        costs_to_allocate: Vec<
            crate::datadogV2::model::ArbitraryRuleResponseDataAttributesCostsToAllocateItems,
        >,
        created: chrono::DateTime<chrono::Utc>,
        enabled: bool,
        last_modified_user_uuid: String,
        order_id: i64,
        provider: Vec<String>,
        rule_name: String,
        strategy: crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategy,
        type_: String,
        updated: chrono::DateTime<chrono::Utc>,
        version: i32,
    ) -> ArbitraryRuleResponseDataAttributes {
        ArbitraryRuleResponseDataAttributes {
            costs_to_allocate,
            created,
            enabled,
            last_modified_user_uuid,
            order_id,
            processing_status: None,
            provider,
            rejected: None,
            rule_name,
            strategy,
            type_,
            updated,
            version,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn processing_status(mut self, value: String) -> Self {
        self.processing_status = Some(value);
        self
    }

    pub fn rejected(mut self, value: bool) -> Self {
        self.rejected = Some(value);
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

impl<'de> Deserialize<'de> for ArbitraryRuleResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArbitraryRuleResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for ArbitraryRuleResponseDataAttributesVisitor {
            type Value = ArbitraryRuleResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut costs_to_allocate: Option<Vec<crate::datadogV2::model::ArbitraryRuleResponseDataAttributesCostsToAllocateItems>> = None;
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut enabled: Option<bool> = None;
                let mut last_modified_user_uuid: Option<String> = None;
                let mut order_id: Option<i64> = None;
                let mut processing_status: Option<String> = None;
                let mut provider: Option<Vec<String>> = None;
                let mut rejected: Option<bool> = None;
                let mut rule_name: Option<String> = None;
                let mut strategy: Option<
                    crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategy,
                > = None;
                let mut type_: Option<String> = None;
                let mut updated: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut version: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "costs_to_allocate" => {
                            costs_to_allocate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created" => {
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_modified_user_uuid" => {
                            last_modified_user_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "order_id" => {
                            order_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "processing_status" => {
                            if v.is_null() {
                                continue;
                            }
                            processing_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider" => {
                            provider = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rejected" => {
                            if v.is_null() {
                                continue;
                            }
                            rejected = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_name" => {
                            rule_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "strategy" => {
                            strategy = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated" => {
                            updated = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let costs_to_allocate = costs_to_allocate
                    .ok_or_else(|| M::Error::missing_field("costs_to_allocate"))?;
                let created = created.ok_or_else(|| M::Error::missing_field("created"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let last_modified_user_uuid = last_modified_user_uuid
                    .ok_or_else(|| M::Error::missing_field("last_modified_user_uuid"))?;
                let order_id = order_id.ok_or_else(|| M::Error::missing_field("order_id"))?;
                let provider = provider.ok_or_else(|| M::Error::missing_field("provider"))?;
                let rule_name = rule_name.ok_or_else(|| M::Error::missing_field("rule_name"))?;
                let strategy = strategy.ok_or_else(|| M::Error::missing_field("strategy"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let updated = updated.ok_or_else(|| M::Error::missing_field("updated"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = ArbitraryRuleResponseDataAttributes {
                    costs_to_allocate,
                    created,
                    enabled,
                    last_modified_user_uuid,
                    order_id,
                    processing_status,
                    provider,
                    rejected,
                    rule_name,
                    strategy,
                    type_,
                    updated,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ArbitraryRuleResponseDataAttributesVisitor)
    }
}

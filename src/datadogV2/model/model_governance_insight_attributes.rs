// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a governance insight. Exactly one of `metric_query`, `event_query`,
/// `usage_query`, `audit_query`, or `percentage_query` is populated, depending on the data
/// source the insight is computed from; the rest are `null`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GovernanceInsightAttributes {
    /// An audit log query used to compute an insight value.
    #[serde(rename = "audit_query")]
    pub audit_query: Option<crate::datadogV2::model::GovernanceInsightAuditQuery>,
    /// A human-readable description of what the insight measures.
    #[serde(rename = "description")]
    pub description: String,
    /// Human-readable name of the insight.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// An event query used to compute an insight value.
    #[serde(rename = "event_query")]
    pub event_query: Option<crate::datadogV2::model::GovernanceInsightEventQuery>,
    /// A metric query used to compute an insight value.
    #[serde(rename = "metric_query")]
    pub metric_query: Option<crate::datadogV2::model::GovernanceInsightMetricQuery>,
    /// A percentage query that computes an insight value as a ratio of two metric queries.
    #[serde(rename = "percentage_query")]
    pub percentage_query: Option<crate::datadogV2::model::GovernanceInsightPercentageQuery>,
    /// The product the insight belongs to.
    #[serde(rename = "product")]
    pub product: String,
    /// Query execution context for running insight queries directly.
    #[serde(rename = "query_config")]
    pub query_config: Option<crate::datadogV2::model::GovernanceInsightQueryConfig>,
    /// The sub-product the insight belongs to, if any.
    #[serde(rename = "sub_product")]
    pub sub_product: String,
    /// The time range the insight value is computed over, if applicable.
    #[serde(rename = "time_range")]
    pub time_range: String,
    /// The unit that the insight's value is measured in.
    #[serde(rename = "unit_name")]
    pub unit_name: String,
    /// A usage query used to compute an insight value.
    #[serde(rename = "usage_query")]
    pub usage_query: Option<crate::datadogV2::model::GovernanceInsightUsageQuery>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GovernanceInsightAttributes {
    pub fn new(
        description: String,
        display_name: String,
        product: String,
        sub_product: String,
        time_range: String,
        unit_name: String,
    ) -> GovernanceInsightAttributes {
        GovernanceInsightAttributes {
            audit_query: None,
            description,
            display_name,
            event_query: None,
            metric_query: None,
            percentage_query: None,
            product,
            query_config: None,
            sub_product,
            time_range,
            unit_name,
            usage_query: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn audit_query(
        mut self,
        value: crate::datadogV2::model::GovernanceInsightAuditQuery,
    ) -> Self {
        self.audit_query = Some(value);
        self
    }

    pub fn event_query(
        mut self,
        value: crate::datadogV2::model::GovernanceInsightEventQuery,
    ) -> Self {
        self.event_query = Some(value);
        self
    }

    pub fn metric_query(
        mut self,
        value: crate::datadogV2::model::GovernanceInsightMetricQuery,
    ) -> Self {
        self.metric_query = Some(value);
        self
    }

    pub fn percentage_query(
        mut self,
        value: crate::datadogV2::model::GovernanceInsightPercentageQuery,
    ) -> Self {
        self.percentage_query = Some(value);
        self
    }

    pub fn query_config(
        mut self,
        value: crate::datadogV2::model::GovernanceInsightQueryConfig,
    ) -> Self {
        self.query_config = Some(value);
        self
    }

    pub fn usage_query(
        mut self,
        value: crate::datadogV2::model::GovernanceInsightUsageQuery,
    ) -> Self {
        self.usage_query = Some(value);
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

impl<'de> Deserialize<'de> for GovernanceInsightAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GovernanceInsightAttributesVisitor;
        impl<'a> Visitor<'a> for GovernanceInsightAttributesVisitor {
            type Value = GovernanceInsightAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut audit_query: Option<crate::datadogV2::model::GovernanceInsightAuditQuery> =
                    None;
                let mut description: Option<String> = None;
                let mut display_name: Option<String> = None;
                let mut event_query: Option<crate::datadogV2::model::GovernanceInsightEventQuery> =
                    None;
                let mut metric_query: Option<
                    crate::datadogV2::model::GovernanceInsightMetricQuery,
                > = None;
                let mut percentage_query: Option<
                    crate::datadogV2::model::GovernanceInsightPercentageQuery,
                > = None;
                let mut product: Option<String> = None;
                let mut query_config: Option<
                    crate::datadogV2::model::GovernanceInsightQueryConfig,
                > = None;
                let mut sub_product: Option<String> = None;
                let mut time_range: Option<String> = None;
                let mut unit_name: Option<String> = None;
                let mut usage_query: Option<crate::datadogV2::model::GovernanceInsightUsageQuery> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "audit_query" => {
                            if v.is_null() {
                                continue;
                            }
                            audit_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_name" => {
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_query" => {
                            if v.is_null() {
                                continue;
                            }
                            event_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_query" => {
                            if v.is_null() {
                                continue;
                            }
                            metric_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "percentage_query" => {
                            if v.is_null() {
                                continue;
                            }
                            percentage_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product" => {
                            product = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_config" => {
                            if v.is_null() {
                                continue;
                            }
                            query_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sub_product" => {
                            sub_product =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time_range" => {
                            time_range = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unit_name" => {
                            unit_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "usage_query" => {
                            if v.is_null() {
                                continue;
                            }
                            usage_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let display_name =
                    display_name.ok_or_else(|| M::Error::missing_field("display_name"))?;
                let product = product.ok_or_else(|| M::Error::missing_field("product"))?;
                let sub_product =
                    sub_product.ok_or_else(|| M::Error::missing_field("sub_product"))?;
                let time_range = time_range.ok_or_else(|| M::Error::missing_field("time_range"))?;
                let unit_name = unit_name.ok_or_else(|| M::Error::missing_field("unit_name"))?;

                let content = GovernanceInsightAttributes {
                    audit_query,
                    description,
                    display_name,
                    event_query,
                    metric_query,
                    percentage_query,
                    product,
                    query_config,
                    sub_product,
                    time_range,
                    unit_name,
                    usage_query,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GovernanceInsightAttributesVisitor)
    }
}

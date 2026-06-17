// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a governance insight.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GovernanceInsightAttributes {
    /// An audit log query used to compute an insight value.
    #[serde(rename = "audit_query")]
    pub audit_query: crate::datadogV2::model::GovernanceInsightAuditQuery,
    /// The best practice associated with an insight. Populated with the first active best practice
    /// matched to the insight; `null` when no best practice is attached.
    #[serde(rename = "best_practice")]
    pub best_practice: crate::datadogV2::model::GovernanceBestPracticeDefinition,
    /// A relative link to the product surface where the insight can be acted upon.
    #[serde(rename = "deep_link")]
    pub deep_link: String,
    /// A human-readable description of what the insight measures.
    #[serde(rename = "description")]
    pub description: String,
    /// Human-readable name of the insight.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// An event query used to compute an insight value.
    #[serde(rename = "event_query")]
    pub event_query: crate::datadogV2::model::GovernanceInsightEventQuery,
    /// A metric query used to compute an insight value.
    #[serde(rename = "metric_query")]
    pub metric_query: crate::datadogV2::model::GovernanceInsightMetricQuery,
    /// The value of the insight over the previous comparison window. `null` when values were
    /// not requested or could not be computed.
    #[serialize_always]
    #[serde(rename = "old_value")]
    pub old_value: Option<f64>,
    /// A percentage query that computes an insight value as a ratio of two metric queries.
    #[serde(rename = "percentage_query")]
    pub percentage_query: crate::datadogV2::model::GovernanceInsightPercentageQuery,
    /// The product the insight belongs to.
    #[serde(rename = "product")]
    pub product: String,
    /// Query execution context that allows the frontend to execute insight queries directly.
    #[serde(rename = "query_config")]
    pub query_config: Option<crate::datadogV2::model::GovernanceInsightQueryConfig>,
    /// The relative order in which the insight should be displayed.
    #[serde(rename = "sort_order")]
    pub sort_order: Option<i64>,
    /// The state of the insight. A `critical` insight receives extra UI treatment to draw
    /// attention to it.
    #[serde(rename = "state")]
    pub state: String,
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
    pub usage_query: crate::datadogV2::model::GovernanceInsightUsageQuery,
    /// The current value of the insight. `null` when values were not requested or could not be computed.
    #[serialize_always]
    #[serde(rename = "value")]
    pub value: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GovernanceInsightAttributes {
    pub fn new(
        audit_query: crate::datadogV2::model::GovernanceInsightAuditQuery,
        best_practice: crate::datadogV2::model::GovernanceBestPracticeDefinition,
        deep_link: String,
        description: String,
        display_name: String,
        event_query: crate::datadogV2::model::GovernanceInsightEventQuery,
        metric_query: crate::datadogV2::model::GovernanceInsightMetricQuery,
        old_value: Option<f64>,
        percentage_query: crate::datadogV2::model::GovernanceInsightPercentageQuery,
        product: String,
        state: String,
        sub_product: String,
        time_range: String,
        unit_name: String,
        usage_query: crate::datadogV2::model::GovernanceInsightUsageQuery,
        value: Option<f64>,
    ) -> GovernanceInsightAttributes {
        GovernanceInsightAttributes {
            audit_query,
            best_practice,
            deep_link,
            description,
            display_name,
            event_query,
            metric_query,
            old_value,
            percentage_query,
            product,
            query_config: None,
            sort_order: None,
            state,
            sub_product,
            time_range,
            unit_name,
            usage_query,
            value,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn query_config(
        mut self,
        value: crate::datadogV2::model::GovernanceInsightQueryConfig,
    ) -> Self {
        self.query_config = Some(value);
        self
    }

    pub fn sort_order(mut self, value: i64) -> Self {
        self.sort_order = Some(value);
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
                let mut best_practice: Option<
                    crate::datadogV2::model::GovernanceBestPracticeDefinition,
                > = None;
                let mut deep_link: Option<String> = None;
                let mut description: Option<String> = None;
                let mut display_name: Option<String> = None;
                let mut event_query: Option<crate::datadogV2::model::GovernanceInsightEventQuery> =
                    None;
                let mut metric_query: Option<
                    crate::datadogV2::model::GovernanceInsightMetricQuery,
                > = None;
                let mut old_value: Option<Option<f64>> = None;
                let mut percentage_query: Option<
                    crate::datadogV2::model::GovernanceInsightPercentageQuery,
                > = None;
                let mut product: Option<String> = None;
                let mut query_config: Option<
                    crate::datadogV2::model::GovernanceInsightQueryConfig,
                > = None;
                let mut sort_order: Option<i64> = None;
                let mut state: Option<String> = None;
                let mut sub_product: Option<String> = None;
                let mut time_range: Option<String> = None;
                let mut unit_name: Option<String> = None;
                let mut usage_query: Option<crate::datadogV2::model::GovernanceInsightUsageQuery> =
                    None;
                let mut value: Option<Option<f64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "audit_query" => {
                            audit_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "best_practice" => {
                            best_practice =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deep_link" => {
                            deep_link = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                            event_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_query" => {
                            metric_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "old_value" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            old_value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "percentage_query" => {
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
                        "sort_order" => {
                            if v.is_null() {
                                continue;
                            }
                            sort_order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                            usage_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let audit_query =
                    audit_query.ok_or_else(|| M::Error::missing_field("audit_query"))?;
                let best_practice =
                    best_practice.ok_or_else(|| M::Error::missing_field("best_practice"))?;
                let deep_link = deep_link.ok_or_else(|| M::Error::missing_field("deep_link"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let display_name =
                    display_name.ok_or_else(|| M::Error::missing_field("display_name"))?;
                let event_query =
                    event_query.ok_or_else(|| M::Error::missing_field("event_query"))?;
                let metric_query =
                    metric_query.ok_or_else(|| M::Error::missing_field("metric_query"))?;
                let old_value = old_value.ok_or_else(|| M::Error::missing_field("old_value"))?;
                let percentage_query =
                    percentage_query.ok_or_else(|| M::Error::missing_field("percentage_query"))?;
                let product = product.ok_or_else(|| M::Error::missing_field("product"))?;
                let state = state.ok_or_else(|| M::Error::missing_field("state"))?;
                let sub_product =
                    sub_product.ok_or_else(|| M::Error::missing_field("sub_product"))?;
                let time_range = time_range.ok_or_else(|| M::Error::missing_field("time_range"))?;
                let unit_name = unit_name.ok_or_else(|| M::Error::missing_field("unit_name"))?;
                let usage_query =
                    usage_query.ok_or_else(|| M::Error::missing_field("usage_query"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = GovernanceInsightAttributes {
                    audit_query,
                    best_practice,
                    deep_link,
                    description,
                    display_name,
                    event_query,
                    metric_query,
                    old_value,
                    percentage_query,
                    product,
                    query_config,
                    sort_order,
                    state,
                    sub_product,
                    time_range,
                    unit_name,
                    usage_query,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GovernanceInsightAttributesVisitor)
    }
}

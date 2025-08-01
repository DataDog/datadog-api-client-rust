// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response with properties for each aggregated usage type.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageBillableSummaryBody {
    /// The total account usage.
    #[serde(rename = "account_billable_usage")]
    pub account_billable_usage: Option<i64>,
    /// The total account committed usage.
    #[serde(rename = "account_committed_usage")]
    pub account_committed_usage: Option<i64>,
    /// The total account on-demand usage.
    #[serde(rename = "account_on_demand_usage")]
    pub account_on_demand_usage: Option<i64>,
    /// Elapsed usage hours for some billable product.
    #[serde(rename = "elapsed_usage_hours")]
    pub elapsed_usage_hours: Option<i64>,
    /// The first billable hour for the org.
    #[serde(rename = "first_billable_usage_hour")]
    pub first_billable_usage_hour: Option<chrono::DateTime<chrono::Utc>>,
    /// The last billable hour for the org.
    #[serde(rename = "last_billable_usage_hour")]
    pub last_billable_usage_hour: Option<chrono::DateTime<chrono::Utc>>,
    /// The number of units used within the billable timeframe.
    #[serde(rename = "org_billable_usage")]
    pub org_billable_usage: Option<i64>,
    /// The percentage of account usage the org represents.
    #[serde(rename = "percentage_in_account")]
    pub percentage_in_account: Option<f64>,
    /// Units pertaining to the usage.
    #[serde(rename = "usage_unit")]
    pub usage_unit: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageBillableSummaryBody {
    pub fn new() -> UsageBillableSummaryBody {
        UsageBillableSummaryBody {
            account_billable_usage: None,
            account_committed_usage: None,
            account_on_demand_usage: None,
            elapsed_usage_hours: None,
            first_billable_usage_hour: None,
            last_billable_usage_hour: None,
            org_billable_usage: None,
            percentage_in_account: None,
            usage_unit: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_billable_usage(mut self, value: i64) -> Self {
        self.account_billable_usage = Some(value);
        self
    }

    pub fn account_committed_usage(mut self, value: i64) -> Self {
        self.account_committed_usage = Some(value);
        self
    }

    pub fn account_on_demand_usage(mut self, value: i64) -> Self {
        self.account_on_demand_usage = Some(value);
        self
    }

    pub fn elapsed_usage_hours(mut self, value: i64) -> Self {
        self.elapsed_usage_hours = Some(value);
        self
    }

    pub fn first_billable_usage_hour(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.first_billable_usage_hour = Some(value);
        self
    }

    pub fn last_billable_usage_hour(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.last_billable_usage_hour = Some(value);
        self
    }

    pub fn org_billable_usage(mut self, value: i64) -> Self {
        self.org_billable_usage = Some(value);
        self
    }

    pub fn percentage_in_account(mut self, value: f64) -> Self {
        self.percentage_in_account = Some(value);
        self
    }

    pub fn usage_unit(mut self, value: String) -> Self {
        self.usage_unit = Some(value);
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

impl Default for UsageBillableSummaryBody {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageBillableSummaryBody {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageBillableSummaryBodyVisitor;
        impl<'a> Visitor<'a> for UsageBillableSummaryBodyVisitor {
            type Value = UsageBillableSummaryBody;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_billable_usage: Option<i64> = None;
                let mut account_committed_usage: Option<i64> = None;
                let mut account_on_demand_usage: Option<i64> = None;
                let mut elapsed_usage_hours: Option<i64> = None;
                let mut first_billable_usage_hour: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut last_billable_usage_hour: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut org_billable_usage: Option<i64> = None;
                let mut percentage_in_account: Option<f64> = None;
                let mut usage_unit: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_billable_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            account_billable_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "account_committed_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            account_committed_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "account_on_demand_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            account_on_demand_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "elapsed_usage_hours" => {
                            if v.is_null() {
                                continue;
                            }
                            elapsed_usage_hours =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "first_billable_usage_hour" => {
                            if v.is_null() {
                                continue;
                            }
                            first_billable_usage_hour =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_billable_usage_hour" => {
                            if v.is_null() {
                                continue;
                            }
                            last_billable_usage_hour =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_billable_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            org_billable_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "percentage_in_account" => {
                            if v.is_null() {
                                continue;
                            }
                            percentage_in_account =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "usage_unit" => {
                            if v.is_null() {
                                continue;
                            }
                            usage_unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UsageBillableSummaryBody {
                    account_billable_usage,
                    account_committed_usage,
                    account_on_demand_usage,
                    elapsed_usage_hours,
                    first_billable_usage_hour,
                    last_billable_usage_hour,
                    org_billable_usage,
                    percentage_in_account,
                    usage_unit,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageBillableSummaryBodyVisitor)
    }
}

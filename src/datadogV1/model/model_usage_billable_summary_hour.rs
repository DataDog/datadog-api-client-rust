// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response with monthly summary of data billed by Datadog.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageBillableSummaryHour {
    /// The billing plan.
    #[serde(rename = "billing_plan")]
    pub billing_plan: Option<String>,
    /// Shows the last date of usage.
    #[serde(rename = "end_date")]
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    /// The number of organizations.
    #[serde(rename = "num_orgs")]
    pub num_orgs: Option<i64>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Shows usage aggregation for a billing period.
    #[serde(rename = "ratio_in_month")]
    pub ratio_in_month: Option<f64>,
    /// The region of the organization.
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// Shows the first date of usage.
    #[serde(rename = "start_date")]
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    /// Response with aggregated usage types.
    #[serde(rename = "usage")]
    pub usage: Option<crate::datadogV1::model::UsageBillableSummaryKeys>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageBillableSummaryHour {
    pub fn new() -> UsageBillableSummaryHour {
        UsageBillableSummaryHour {
            billing_plan: None,
            end_date: None,
            num_orgs: None,
            org_name: None,
            public_id: None,
            ratio_in_month: None,
            region: None,
            start_date: None,
            usage: None,
            _unparsed: false,
        }
    }

    pub fn billing_plan(mut self, value: String) -> Self {
        self.billing_plan = Some(value);
        self
    }

    pub fn end_date(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn num_orgs(mut self, value: i64) -> Self {
        self.num_orgs = Some(value);
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

    pub fn ratio_in_month(mut self, value: f64) -> Self {
        self.ratio_in_month = Some(value);
        self
    }

    pub fn region(mut self, value: String) -> Self {
        self.region = Some(value);
        self
    }

    pub fn start_date(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn usage(mut self, value: crate::datadogV1::model::UsageBillableSummaryKeys) -> Self {
        self.usage = Some(value);
        self
    }
}

impl Default for UsageBillableSummaryHour {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageBillableSummaryHour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageBillableSummaryHourVisitor;
        impl<'a> Visitor<'a> for UsageBillableSummaryHourVisitor {
            type Value = UsageBillableSummaryHour;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut billing_plan: Option<String> = None;
                let mut end_date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut num_orgs: Option<i64> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut ratio_in_month: Option<f64> = None;
                let mut region: Option<String> = None;
                let mut start_date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut usage: Option<crate::datadogV1::model::UsageBillableSummaryKeys> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "billing_plan" => {
                            if v.is_null() {
                                continue;
                            }
                            billing_plan =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end_date" => {
                            if v.is_null() {
                                continue;
                            }
                            end_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "num_orgs" => {
                            if v.is_null() {
                                continue;
                            }
                            num_orgs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "ratio_in_month" => {
                            if v.is_null() {
                                continue;
                            }
                            ratio_in_month =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region" => {
                            if v.is_null() {
                                continue;
                            }
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_date" => {
                            if v.is_null() {
                                continue;
                            }
                            start_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "usage" => {
                            if v.is_null() {
                                continue;
                            }
                            usage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageBillableSummaryHour {
                    billing_plan,
                    end_date,
                    num_orgs,
                    org_name,
                    public_id,
                    ratio_in_month,
                    region,
                    start_date,
                    usage,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageBillableSummaryHourVisitor)
    }
}

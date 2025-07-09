// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The usage for one set of tags for one hour.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HourlyUsageAttributionBody {
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<chrono::DateTime<chrono::Utc>>,
    /// The name of the organization.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// The region of the Datadog instance that the organization belongs to.
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// The source of the usage attribution tag configuration and the selected tags in the format of `<source_org_name>:::<selected tag 1>///<selected tag 2>///<selected tag 3>`.
    #[serde(rename = "tag_config_source")]
    pub tag_config_source: Option<String>,
    /// Tag keys and values.
    ///
    /// A `null` value here means that the requested tag breakdown cannot be applied because it does not match the [tags
    /// configured for usage attribution](<https://docs.datadoghq.com/account_management/billing/usage_attribution/#getting-started>).
    /// In this scenario the API returns the total usage, not broken down by tags.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<std::collections::BTreeMap<String, Vec<String>>>>,
    /// Total product usage for the given tags within the hour.
    #[serde(rename = "total_usage_sum")]
    pub total_usage_sum: Option<f64>,
    /// Shows the most recent hour in the current month for all organizations where usages are calculated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    /// Supported products for hourly usage attribution requests.
    /// The following values have been **deprecated**: `estimated_indexed_spans_usage`, `estimated_ingested_spans_usage`.
    #[serde(rename = "usage_type")]
    pub usage_type: Option<crate::datadogV1::model::HourlyUsageAttributionUsageType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HourlyUsageAttributionBody {
    pub fn new() -> HourlyUsageAttributionBody {
        HourlyUsageAttributionBody {
            hour: None,
            org_name: None,
            public_id: None,
            region: None,
            tag_config_source: None,
            tags: None,
            total_usage_sum: None,
            updated_at: None,
            usage_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn hour(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.hour = Some(value);
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

    pub fn region(mut self, value: String) -> Self {
        self.region = Some(value);
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

    pub fn total_usage_sum(mut self, value: f64) -> Self {
        self.total_usage_sum = Some(value);
        self
    }

    pub fn updated_at(mut self, value: String) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn usage_type(
        mut self,
        value: crate::datadogV1::model::HourlyUsageAttributionUsageType,
    ) -> Self {
        self.usage_type = Some(value);
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

impl Default for HourlyUsageAttributionBody {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HourlyUsageAttributionBody {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HourlyUsageAttributionBodyVisitor;
        impl<'a> Visitor<'a> for HourlyUsageAttributionBodyVisitor {
            type Value = HourlyUsageAttributionBody;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut hour: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut region: Option<String> = None;
                let mut tag_config_source: Option<String> = None;
                let mut tags: Option<Option<std::collections::BTreeMap<String, Vec<String>>>> =
                    None;
                let mut total_usage_sum: Option<f64> = None;
                let mut updated_at: Option<String> = None;
                let mut usage_type: Option<
                    crate::datadogV1::model::HourlyUsageAttributionUsageType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hour" => {
                            if v.is_null() {
                                continue;
                            }
                            hour = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "region" => {
                            if v.is_null() {
                                continue;
                            }
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "total_usage_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            total_usage_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "usage_type" => {
                            if v.is_null() {
                                continue;
                            }
                            usage_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _usage_type) = usage_type {
                                match _usage_type {
                                    crate::datadogV1::model::HourlyUsageAttributionUsageType::UnparsedObject(_usage_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = HourlyUsageAttributionBody {
                    hour,
                    org_name,
                    public_id,
                    region,
                    tag_config_source,
                    tags,
                    total_usage_sum,
                    updated_at,
                    usage_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HourlyUsageAttributionBodyVisitor)
    }
}

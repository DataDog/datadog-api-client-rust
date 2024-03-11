// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Usage Summary by tag for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageAttributionBody {
    /// Datetime in ISO-8601 format, UTC, precise to month: [YYYY-MM].
    #[serde(rename = "month")]
    pub month: Option<String>,
    /// The name of the organization.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// The source of the usage attribution tag configuration and the selected tags in the format `<source_org_name>:::<selected tag 1>///<selected tag 2>///<selected tag 3>`.
    #[serde(rename = "tag_config_source")]
    pub tag_config_source: Option<String>,
    /// Tag keys and values.
    ///
    /// A `null` value here means that the requested tag breakdown cannot be applied because it does not match the [tags
    /// configured for usage attribution](<https://docs.datadoghq.com/account_management/billing/usage_attribution/#getting-started>).
    /// In this scenario the API returns the total usage, not broken down by tags.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<std::collections::BTreeMap<String, Option<Vec<String>>>>>,
    /// Shows the the most recent hour in the current months for all organizations for which all usages were calculated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    /// Fields in Usage Summary by tag(s).
    #[serde(rename = "values")]
    pub values: Option<crate::datadogV1::model::UsageAttributionValues>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageAttributionBody {
    pub fn new() -> UsageAttributionBody {
        UsageAttributionBody {
            month: None,
            org_name: None,
            public_id: None,
            tag_config_source: None,
            tags: None,
            updated_at: None,
            values: None,
            _unparsed: false,
        }
    }

    pub fn month(&mut self, value: String) -> &mut Self {
        self.month = Some(value);
        self
    }

    pub fn org_name(&mut self, value: String) -> &mut Self {
        self.org_name = Some(value);
        self
    }

    pub fn public_id(&mut self, value: String) -> &mut Self {
        self.public_id = Some(value);
        self
    }

    pub fn tag_config_source(&mut self, value: String) -> &mut Self {
        self.tag_config_source = Some(value);
        self
    }

    pub fn tags(
        &mut self,
        value: Option<std::collections::BTreeMap<String, Option<Vec<String>>>>,
    ) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn updated_at(&mut self, value: String) -> &mut Self {
        self.updated_at = Some(value);
        self
    }

    pub fn values(&mut self, value: crate::datadogV1::model::UsageAttributionValues) -> &mut Self {
        self.values = Some(value);
        self
    }
}

impl Default for UsageAttributionBody {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageAttributionBody {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageAttributionBodyVisitor;
        impl<'a> Visitor<'a> for UsageAttributionBodyVisitor {
            type Value = UsageAttributionBody;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut month: Option<String> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut tag_config_source: Option<String> = None;
                let mut tags: Option<
                    Option<std::collections::BTreeMap<String, Option<Vec<String>>>>,
                > = None;
                let mut updated_at: Option<String> = None;
                let mut values: Option<crate::datadogV1::model::UsageAttributionValues> = None;
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
                        &_ => {}
                    }
                }

                let content = UsageAttributionBody {
                    month,
                    org_name,
                    public_id,
                    tag_config_source,
                    tags,
                    updated_at,
                    values,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageAttributionBodyVisitor)
    }
}

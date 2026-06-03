// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The lists of field names returned by `GET /api/v1/usage/summary` at each
/// of its three response levels. Each list contains every key the data endpoint
/// emits—both typed fields declared in the OpenAPI spec and untyped keys
/// exposed through `additionalProperties`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageSummaryAvailableFieldsAttributes {
    /// Sorted list of every key returned inside each `UsageSummaryDate`
    /// entry of `usage[]` (typed fields and `additionalProperties` keys
    /// combined).
    #[serde(rename = "date_fields")]
    pub date_fields: Option<Vec<String>>,
    /// Sorted list of every key returned inside each `UsageSummaryDateOrg`
    /// entry of `usage[].orgs[]` (typed fields and `additionalProperties`
    /// keys combined).
    #[serde(rename = "date_org_fields")]
    pub date_org_fields: Option<Vec<String>>,
    /// Sorted list of every key returned as a direct property of
    /// `UsageSummaryResponse` (typed fields and `additionalProperties`
    /// keys combined).
    #[serde(rename = "response_fields")]
    pub response_fields: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageSummaryAvailableFieldsAttributes {
    pub fn new() -> UsageSummaryAvailableFieldsAttributes {
        UsageSummaryAvailableFieldsAttributes {
            date_fields: None,
            date_org_fields: None,
            response_fields: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn date_fields(mut self, value: Vec<String>) -> Self {
        self.date_fields = Some(value);
        self
    }

    pub fn date_org_fields(mut self, value: Vec<String>) -> Self {
        self.date_org_fields = Some(value);
        self
    }

    pub fn response_fields(mut self, value: Vec<String>) -> Self {
        self.response_fields = Some(value);
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

impl Default for UsageSummaryAvailableFieldsAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageSummaryAvailableFieldsAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageSummaryAvailableFieldsAttributesVisitor;
        impl<'a> Visitor<'a> for UsageSummaryAvailableFieldsAttributesVisitor {
            type Value = UsageSummaryAvailableFieldsAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut date_fields: Option<Vec<String>> = None;
                let mut date_org_fields: Option<Vec<String>> = None;
                let mut response_fields: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "date_fields" => {
                            if v.is_null() {
                                continue;
                            }
                            date_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "date_org_fields" => {
                            if v.is_null() {
                                continue;
                            }
                            date_org_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "response_fields" => {
                            if v.is_null() {
                                continue;
                            }
                            response_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UsageSummaryAvailableFieldsAttributes {
                    date_fields,
                    date_org_fields,
                    response_fields,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageSummaryAvailableFieldsAttributesVisitor)
    }
}

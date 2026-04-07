// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an anonymize users response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnonymizeUsersResponseAttributes {
    /// List of errors encountered during anonymization, one entry per failed user.
    #[serde(rename = "anonymize_errors")]
    pub anonymize_errors: Vec<crate::datadogV2::model::AnonymizeUserError>,
    /// List of user IDs (UUIDs) that were successfully anonymized.
    #[serde(rename = "anonymized_user_ids")]
    pub anonymized_user_ids: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AnonymizeUsersResponseAttributes {
    pub fn new(
        anonymize_errors: Vec<crate::datadogV2::model::AnonymizeUserError>,
        anonymized_user_ids: Vec<String>,
    ) -> AnonymizeUsersResponseAttributes {
        AnonymizeUsersResponseAttributes {
            anonymize_errors,
            anonymized_user_ids,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for AnonymizeUsersResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnonymizeUsersResponseAttributesVisitor;
        impl<'a> Visitor<'a> for AnonymizeUsersResponseAttributesVisitor {
            type Value = AnonymizeUsersResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut anonymize_errors: Option<Vec<crate::datadogV2::model::AnonymizeUserError>> =
                    None;
                let mut anonymized_user_ids: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "anonymize_errors" => {
                            anonymize_errors =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "anonymized_user_ids" => {
                            anonymized_user_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let anonymize_errors =
                    anonymize_errors.ok_or_else(|| M::Error::missing_field("anonymize_errors"))?;
                let anonymized_user_ids = anonymized_user_ids
                    .ok_or_else(|| M::Error::missing_field("anonymized_user_ids"))?;

                let content = AnonymizeUsersResponseAttributes {
                    anonymize_errors,
                    anonymized_user_ids,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AnonymizeUsersResponseAttributesVisitor)
    }
}

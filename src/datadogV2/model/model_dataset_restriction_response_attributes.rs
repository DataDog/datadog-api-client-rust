// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The current configuration of a dataset restriction, including restriction mode,
/// ownership mode, and exempt principals.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatasetRestrictionResponseAttributes {
    /// Controls how dataset ownership is determined. `disabled` turns off ownership-based access
    /// entirely. `team_tag_based` assigns dataset ownership based on the team tags applied to the
    /// data, allowing team members to see their own team's datasets.
    #[serde(rename = "ownership_mode")]
    pub ownership_mode: Option<crate::datadogV2::model::DatasetRestrictionOwnershipMode>,
    /// Internal key used by the restriction enforcement system to identify this restriction rule.
    #[serde(rename = "restriction_key")]
    pub restriction_key: Option<String>,
    /// Controls the default data visibility for the product type. `standard` makes data visible
    /// to all users with appropriate product access. `default_hide` hides data by default and
    /// requires explicit grants for each dataset.
    #[serde(rename = "restriction_mode")]
    pub restriction_mode: crate::datadogV2::model::DatasetRestrictionRestrictionMode,
    /// Principals (users or roles) that are exempt from this restriction and retain
    /// full data access regardless of the restriction mode.
    #[serde(rename = "unrestricted_principals")]
    pub unrestricted_principals: Option<Vec<crate::datadogV2::model::DatasetRestrictionPrincipal>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatasetRestrictionResponseAttributes {
    pub fn new(
        restriction_mode: crate::datadogV2::model::DatasetRestrictionRestrictionMode,
    ) -> DatasetRestrictionResponseAttributes {
        DatasetRestrictionResponseAttributes {
            ownership_mode: None,
            restriction_key: None,
            restriction_mode,
            unrestricted_principals: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn ownership_mode(
        mut self,
        value: crate::datadogV2::model::DatasetRestrictionOwnershipMode,
    ) -> Self {
        self.ownership_mode = Some(value);
        self
    }

    pub fn restriction_key(mut self, value: String) -> Self {
        self.restriction_key = Some(value);
        self
    }

    pub fn unrestricted_principals(
        mut self,
        value: Vec<crate::datadogV2::model::DatasetRestrictionPrincipal>,
    ) -> Self {
        self.unrestricted_principals = Some(value);
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

impl<'de> Deserialize<'de> for DatasetRestrictionResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatasetRestrictionResponseAttributesVisitor;
        impl<'a> Visitor<'a> for DatasetRestrictionResponseAttributesVisitor {
            type Value = DatasetRestrictionResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ownership_mode: Option<
                    crate::datadogV2::model::DatasetRestrictionOwnershipMode,
                > = None;
                let mut restriction_key: Option<String> = None;
                let mut restriction_mode: Option<
                    crate::datadogV2::model::DatasetRestrictionRestrictionMode,
                > = None;
                let mut unrestricted_principals: Option<
                    Vec<crate::datadogV2::model::DatasetRestrictionPrincipal>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ownership_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            ownership_mode =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _ownership_mode) = ownership_mode {
                                match _ownership_mode {
                                    crate::datadogV2::model::DatasetRestrictionOwnershipMode::UnparsedObject(_ownership_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "restriction_key" => {
                            if v.is_null() {
                                continue;
                            }
                            restriction_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "restriction_mode" => {
                            restriction_mode =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _restriction_mode) = restriction_mode {
                                match _restriction_mode {
                                    crate::datadogV2::model::DatasetRestrictionRestrictionMode::UnparsedObject(_restriction_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "unrestricted_principals" => {
                            if v.is_null() {
                                continue;
                            }
                            unrestricted_principals =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let restriction_mode =
                    restriction_mode.ok_or_else(|| M::Error::missing_field("restriction_mode"))?;

                let content = DatasetRestrictionResponseAttributes {
                    ownership_mode,
                    restriction_key,
                    restriction_mode,
                    unrestricted_principals,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatasetRestrictionResponseAttributesVisitor)
    }
}

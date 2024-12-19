// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Vulnerability remediation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Remediation {
    /// Whether the vulnerability can be resolved when recompiling the package or not.
    #[serde(rename = "auto_solvable")]
    pub auto_solvable: bool,
    /// Avoided advisories.
    #[serde(rename = "avoided_advisories")]
    pub avoided_advisories: Vec<crate::datadogV2::model::Advisory>,
    /// Remediation fixed advisories.
    #[serde(rename = "fixed_advisories")]
    pub fixed_advisories: Vec<crate::datadogV2::model::Advisory>,
    /// Library name remediating the vulnerability.
    #[serde(rename = "library_name")]
    pub library_name: String,
    /// Library version remediating the vulnerability.
    #[serde(rename = "library_version")]
    pub library_version: String,
    /// New advisories.
    #[serde(rename = "new_advisories")]
    pub new_advisories: Vec<crate::datadogV2::model::Advisory>,
    /// Remaining advisories.
    #[serde(rename = "remaining_advisories")]
    pub remaining_advisories: Vec<crate::datadogV2::model::Advisory>,
    /// Remediation type.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl Remediation {
    pub fn new(
        auto_solvable: bool,
        avoided_advisories: Vec<crate::datadogV2::model::Advisory>,
        fixed_advisories: Vec<crate::datadogV2::model::Advisory>,
        library_name: String,
        library_version: String,
        new_advisories: Vec<crate::datadogV2::model::Advisory>,
        remaining_advisories: Vec<crate::datadogV2::model::Advisory>,
        type_: String,
    ) -> Remediation {
        Remediation {
            auto_solvable,
            avoided_advisories,
            fixed_advisories,
            library_name,
            library_version,
            new_advisories,
            remaining_advisories,
            type_,
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

impl<'de> Deserialize<'de> for Remediation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RemediationVisitor;
        impl<'a> Visitor<'a> for RemediationVisitor {
            type Value = Remediation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auto_solvable: Option<bool> = None;
                let mut avoided_advisories: Option<Vec<crate::datadogV2::model::Advisory>> = None;
                let mut fixed_advisories: Option<Vec<crate::datadogV2::model::Advisory>> = None;
                let mut library_name: Option<String> = None;
                let mut library_version: Option<String> = None;
                let mut new_advisories: Option<Vec<crate::datadogV2::model::Advisory>> = None;
                let mut remaining_advisories: Option<Vec<crate::datadogV2::model::Advisory>> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auto_solvable" => {
                            auto_solvable =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avoided_advisories" => {
                            avoided_advisories =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fixed_advisories" => {
                            fixed_advisories =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "library_name" => {
                            library_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "library_version" => {
                            library_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "new_advisories" => {
                            new_advisories =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "remaining_advisories" => {
                            remaining_advisories =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let auto_solvable =
                    auto_solvable.ok_or_else(|| M::Error::missing_field("auto_solvable"))?;
                let avoided_advisories = avoided_advisories
                    .ok_or_else(|| M::Error::missing_field("avoided_advisories"))?;
                let fixed_advisories =
                    fixed_advisories.ok_or_else(|| M::Error::missing_field("fixed_advisories"))?;
                let library_name =
                    library_name.ok_or_else(|| M::Error::missing_field("library_name"))?;
                let library_version =
                    library_version.ok_or_else(|| M::Error::missing_field("library_version"))?;
                let new_advisories =
                    new_advisories.ok_or_else(|| M::Error::missing_field("new_advisories"))?;
                let remaining_advisories = remaining_advisories
                    .ok_or_else(|| M::Error::missing_field("remaining_advisories"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = Remediation {
                    auto_solvable,
                    avoided_advisories,
                    fixed_advisories,
                    library_name,
                    library_version,
                    new_advisories,
                    remaining_advisories,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RemediationVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for the flaky tests search request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FlakyTestsSearchRequestAttributes {
    /// Search filter settings.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::FlakyTestsSearchFilter>,
    /// Whether to include the status change history for each flaky test in the response.
    /// When set to true, each test will include a `history` array with chronological status changes.
    /// Defaults to false.
    #[serde(rename = "include_history")]
    pub include_history: Option<bool>,
    /// Pagination attributes for listing flaky tests.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::FlakyTestsSearchPageOptions>,
    /// Parameter for sorting flaky test results. The default sort is by ascending Fully Qualified Name (FQN). The FQN is the concatenation of the test module, suite, and name.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::FlakyTestsSearchSort>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FlakyTestsSearchRequestAttributes {
    pub fn new() -> FlakyTestsSearchRequestAttributes {
        FlakyTestsSearchRequestAttributes {
            filter: None,
            include_history: None,
            page: None,
            sort: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn filter(mut self, value: crate::datadogV2::model::FlakyTestsSearchFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn include_history(mut self, value: bool) -> Self {
        self.include_history = Some(value);
        self
    }

    pub fn page(mut self, value: crate::datadogV2::model::FlakyTestsSearchPageOptions) -> Self {
        self.page = Some(value);
        self
    }

    pub fn sort(mut self, value: crate::datadogV2::model::FlakyTestsSearchSort) -> Self {
        self.sort = Some(value);
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

impl Default for FlakyTestsSearchRequestAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FlakyTestsSearchRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FlakyTestsSearchRequestAttributesVisitor;
        impl<'a> Visitor<'a> for FlakyTestsSearchRequestAttributesVisitor {
            type Value = FlakyTestsSearchRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter: Option<crate::datadogV2::model::FlakyTestsSearchFilter> = None;
                let mut include_history: Option<bool> = None;
                let mut page: Option<crate::datadogV2::model::FlakyTestsSearchPageOptions> = None;
                let mut sort: Option<crate::datadogV2::model::FlakyTestsSearchSort> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include_history" => {
                            if v.is_null() {
                                continue;
                            }
                            include_history =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "page" => {
                            if v.is_null() {
                                continue;
                            }
                            page = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _sort) = sort {
                                match _sort {
                                    crate::datadogV2::model::FlakyTestsSearchSort::UnparsedObject(_sort) => {
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

                let content = FlakyTestsSearchRequestAttributes {
                    filter,
                    include_history,
                    page,
                    sort,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FlakyTestsSearchRequestAttributesVisitor)
    }
}

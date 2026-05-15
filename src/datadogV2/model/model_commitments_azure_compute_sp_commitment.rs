// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Azure Compute Savings Plan commitment details.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CommitmentsAzureComputeSPCommitment {
    /// The display name of the Azure Savings Plan.
    #[serde(rename = "benefit_name")]
    pub benefit_name: String,
    /// The unique identifier of the Savings Plan.
    #[serde(rename = "commitment_id")]
    pub commitment_id: String,
    /// The hourly committed spend for the Savings Plan.
    #[serde(rename = "committed_spend_per_hour")]
    pub committed_spend_per_hour: Option<f64>,
    /// The expiration date of the commitment.
    #[serde(rename = "expiration_date")]
    pub expiration_date: Option<String>,
    /// The start date of the commitment.
    #[serde(rename = "start_date")]
    pub start_date: Option<String>,
    /// The term length in years.
    #[serde(rename = "term_length")]
    pub term_length: Option<f64>,
    /// The utilization percentage of the commitment.
    #[serde(rename = "utilization")]
    pub utilization: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CommitmentsAzureComputeSPCommitment {
    pub fn new(benefit_name: String, commitment_id: String) -> CommitmentsAzureComputeSPCommitment {
        CommitmentsAzureComputeSPCommitment {
            benefit_name,
            commitment_id,
            committed_spend_per_hour: None,
            expiration_date: None,
            start_date: None,
            term_length: None,
            utilization: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn committed_spend_per_hour(mut self, value: f64) -> Self {
        self.committed_spend_per_hour = Some(value);
        self
    }

    pub fn expiration_date(mut self, value: String) -> Self {
        self.expiration_date = Some(value);
        self
    }

    pub fn start_date(mut self, value: String) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn term_length(mut self, value: f64) -> Self {
        self.term_length = Some(value);
        self
    }

    pub fn utilization(mut self, value: f64) -> Self {
        self.utilization = Some(value);
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

impl<'de> Deserialize<'de> for CommitmentsAzureComputeSPCommitment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitmentsAzureComputeSPCommitmentVisitor;
        impl<'a> Visitor<'a> for CommitmentsAzureComputeSPCommitmentVisitor {
            type Value = CommitmentsAzureComputeSPCommitment;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut benefit_name: Option<String> = None;
                let mut commitment_id: Option<String> = None;
                let mut committed_spend_per_hour: Option<f64> = None;
                let mut expiration_date: Option<String> = None;
                let mut start_date: Option<String> = None;
                let mut term_length: Option<f64> = None;
                let mut utilization: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "benefit_name" => {
                            benefit_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "commitment_id" => {
                            commitment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "committed_spend_per_hour" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            committed_spend_per_hour =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expiration_date" => {
                            if v.is_null() {
                                continue;
                            }
                            expiration_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_date" => {
                            if v.is_null() {
                                continue;
                            }
                            start_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "term_length" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            term_length =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "utilization" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            utilization =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let benefit_name =
                    benefit_name.ok_or_else(|| M::Error::missing_field("benefit_name"))?;
                let commitment_id =
                    commitment_id.ok_or_else(|| M::Error::missing_field("commitment_id"))?;

                let content = CommitmentsAzureComputeSPCommitment {
                    benefit_name,
                    commitment_id,
                    committed_spend_per_hour,
                    expiration_date,
                    start_date,
                    term_length,
                    utilization,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CommitmentsAzureComputeSPCommitmentVisitor)
    }
}

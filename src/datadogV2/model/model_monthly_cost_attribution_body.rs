// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Cost data.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonthlyCostAttributionBody {
    /// Cost Attribution by Tag for a given organization.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::MonthlyCostAttributionAttributes>,
    /// Unique ID of the response.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Type of cost attribution data.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::CostAttributionType>,
}

impl MonthlyCostAttributionBody {
    pub fn new() -> MonthlyCostAttributionBody {
        MonthlyCostAttributionBody {
            attributes: None,
            id: None,
            type_: None,
        }
    }

    pub fn with_attributes(
        &mut self,
        value: crate::datadogV2::model::MonthlyCostAttributionAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn with_id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn with_type_(&mut self, value: crate::datadogV2::model::CostAttributionType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
impl Default for MonthlyCostAttributionBody {
    fn default() -> Self {
        Self::new()
    }
}

/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// RestrictionQueryUpdatePayload : Update a restriction query.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RestrictionQueryUpdatePayload {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::RestrictionQueryUpdateData>>,
}

impl RestrictionQueryUpdatePayload {
    /// Update a restriction query.
    pub fn new() -> RestrictionQueryUpdatePayload {
        RestrictionQueryUpdatePayload {
            data: None,
        }
    }
}



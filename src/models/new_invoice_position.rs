/*
 * Billing Backend
 *
 * # API for managing billing of customers  The purpose of this API is to manage customers, articles, recurring billing, and payments.  The caller has to be authenticated with the system and provide a JWT token in the Authorization header of the HTTP request. When using the API you typically get this token by authenticating first with OAuth 2.0.  When you are trying this API using the Swagger interface, you need to click the `Authorize` button and then again the Authorize button in the pop-up that gets opened.
 *
 * The version of the OpenAPI document: 0.1.1
 * Contact: tech@p7m.de
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NewInvoicePosition {
    #[serde(rename = "positionNumber")]
    pub position_number: i32,
    #[serde(rename = "productCode")]
    pub product_code: String,
    #[serde(rename = "description")]
    pub description: Vec<String>,
    #[serde(rename = "quantityThousandth")]
    pub quantity_thousandth: i32,
    #[serde(rename = "unitPrice")]
    pub unit_price: i64,
    #[serde(rename = "allPrice")]
    pub all_price: i64,
    #[serde(rename = "vatRatePerMil")]
    pub vat_rate_per_mil: i32,
    #[serde(rename = "serviceStart")]
    pub service_start: String,
    #[serde(rename = "serviceEnd")]
    pub service_end: String,
}

impl NewInvoicePosition {
    pub fn new(position_number: i32, product_code: String, description: Vec<String>, quantity_thousandth: i32, unit_price: i64, all_price: i64, vat_rate_per_mil: i32, service_start: String, service_end: String) -> NewInvoicePosition {
        NewInvoicePosition {
            position_number,
            product_code,
            description,
            quantity_thousandth,
            unit_price,
            all_price,
            vat_rate_per_mil,
            service_start,
            service_end,
        }
    }
}



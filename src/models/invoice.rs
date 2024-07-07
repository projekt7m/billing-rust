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
pub struct Invoice {
    #[serde(rename = "invoiceId")]
    pub invoice_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "numberGroup")]
    pub number_group: String,
    #[serde(rename = "invoiceNumber")]
    pub invoice_number: i32,
    #[serde(rename = "invoiceDate")]
    pub invoice_date: String,
    #[serde(rename = "dueDate")]
    pub due_date: String,
    #[serde(rename = "customerId")]
    pub customer_id: String,
    #[serde(rename = "visibleCustomerId")]
    pub visible_customer_id: i32,
    #[serde(rename = "salutation")]
    pub salutation: crate::models::Salutation,
    #[serde(rename = "companyName")]
    pub company_name: String,
    #[serde(rename = "givenName")]
    pub given_name: String,
    #[serde(rename = "familyName")]
    pub family_name: String,
    #[serde(rename = "deliveryInstructions")]
    pub delivery_instructions: String,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "addressExtra")]
    pub address_extra: String,
    #[serde(rename = "zip")]
    pub zip: String,
    #[serde(rename = "town")]
    pub town: String,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "netTotal")]
    pub net_total: i64,
    #[serde(rename = "grossTotal")]
    pub gross_total: i64,
    #[serde(rename = "currencyId")]
    pub currency_id: String,
    #[serde(rename = "bankAccounts")]
    pub bank_accounts: Vec<String>,
    #[serde(rename = "lastChange")]
    pub last_change: String,
}

impl Invoice {
    pub fn new(invoice_id: String, tenant_id: String, number_group: String, invoice_number: i32, invoice_date: String, due_date: String, customer_id: String, visible_customer_id: i32, salutation: crate::models::Salutation, company_name: String, given_name: String, family_name: String, delivery_instructions: String, address: String, address_extra: String, zip: String, town: String, country: String, net_total: i64, gross_total: i64, currency_id: String, bank_accounts: Vec<String>, last_change: String) -> Invoice {
        Invoice {
            invoice_id,
            tenant_id,
            number_group,
            invoice_number,
            invoice_date,
            due_date,
            customer_id,
            visible_customer_id,
            salutation,
            company_name,
            given_name,
            family_name,
            delivery_instructions,
            address,
            address_extra,
            zip,
            town,
            country,
            net_total,
            gross_total,
            currency_id,
            bank_accounts,
            last_change,
        }
    }
}


# \InvoiceApi

All URIs are relative to *https://billing.api.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_currencies**](InvoiceApi.md#get_currencies) | **GET** /currencies | Get a list of all (available) currencies
[**get_invoices**](InvoiceApi.md#get_invoices) | **GET** /invoices | Get a list of all invoices
[**get_invoices_id**](InvoiceApi.md#get_invoices_id) | **GET** /invoices/{invoice_id} | Get a single invoice by its ID
[**post_invoices**](InvoiceApi.md#post_invoices) | **POST** /invoices | Create a new invoice
[**post_invoices_invoice_id**](InvoiceApi.md#post_invoices_invoice_id) | **POST** /invoices/{invoice_id} | Create a new invoice position



## get_currencies

> crate::models::CurrencyData get_currencies()
Get a list of all (available) currencies

Get a list of all (available) currencies

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CurrencyData**](CurrencyData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoices

> crate::models::InvoiceData get_invoices()
Get a list of all invoices

Get a list of all invoices

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InvoiceData**](InvoiceData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoices_id

> crate::models::DetailedInvoice get_invoices_id(invoice_id)
Get a single invoice by its ID

Get a single invoice by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **String** | ID of the invoice | [required] |

### Return type

[**crate::models::DetailedInvoice**](DetailedInvoice.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_invoices

> crate::models::Invoice post_invoices(new_invoice)
Create a new invoice

Create a new invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_invoice** | [**NewInvoice**](NewInvoice.md) | Invoice to create | [required] |

### Return type

[**crate::models::Invoice**](Invoice.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_invoices_invoice_id

> crate::models::NewInvoicePosition post_invoices_invoice_id(invoice_id, new_invoice_position)
Create a new invoice position

Create a new invoice position

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **String** | ID of the invoice | [required] |
**new_invoice_position** | [**NewInvoicePosition**](NewInvoicePosition.md) | Invoice position to create | [required] |

### Return type

[**crate::models::NewInvoicePosition**](NewInvoicePosition.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


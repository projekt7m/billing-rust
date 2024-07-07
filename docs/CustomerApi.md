# \CustomerApi

All URIs are relative to *https://billing.api.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_customers**](CustomerApi.md#get_customers) | **GET** /customers | Get a list of all customers
[**get_customers_id**](CustomerApi.md#get_customers_id) | **GET** /customers/{customer_id} | Request a single customer by its ID
[**post_customers**](CustomerApi.md#post_customers) | **POST** /customers | Create a new customer
[**pub_customers_id**](CustomerApi.md#pub_customers_id) | **PUT** /customers/{customer_id} | Update an existing customer



## get_customers

> crate::models::CustomerData get_customers()
Get a list of all customers

Get a list of all customers

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CustomerData**](CustomerData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_customers_id

> crate::models::Customer get_customers_id(customer_id)
Request a single customer by its ID

Request a single customer by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the customer | [required] |

### Return type

[**crate::models::Customer**](Customer.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_customers

> crate::models::Customer post_customers(new_customer)
Create a new customer

Create a new customer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_customer** | [**NewCustomer**](NewCustomer.md) | Customer to create | [required] |

### Return type

[**crate::models::Customer**](Customer.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pub_customers_id

> crate::models::Customer pub_customers_id(customer_id, new_customer)
Update an existing customer

Update an existing customer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | ID of the customer | [required] |
**new_customer** | [**NewCustomer**](NewCustomer.md) | Customer to update | [required] |

### Return type

[**crate::models::Customer**](Customer.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


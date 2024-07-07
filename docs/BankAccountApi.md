# \BankAccountApi

All URIs are relative to *https://billing.api.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_bank_accounts**](BankAccountApi.md#get_bank_accounts) | **GET** /bankaccounts | Get a list of all bank accounts
[**post_bank_accounts**](BankAccountApi.md#post_bank_accounts) | **POST** /bankaccounts | Create a new bank account



## get_bank_accounts

> crate::models::BankAccountData get_bank_accounts()
Get a list of all bank accounts

Get a list of all bank accounts

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BankAccountData**](BankAccountData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_bank_accounts

> crate::models::BankAccount post_bank_accounts(new_bank_account)
Create a new bank account

Create a new bank account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_bank_account** | [**NewBankAccount**](NewBankAccount.md) | Bank account to create | [required] |

### Return type

[**crate::models::BankAccount**](BankAccount.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


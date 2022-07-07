# \ProxyApi

All URIs are relative to *http://localhost:8888*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_proxy_ping_get**](ProxyApi.md#api_proxy_ping_get) | **GET** /api/proxy/ping | 
[**api_proxy_uuid_playercount_post**](ProxyApi.md#api_proxy_uuid_playercount_post) | **POST** /api/proxy/{uuid}/playercount | 



## api_proxy_ping_get

> crate::models::PingInfo api_proxy_ping_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PingInfo**](PingInfo.md)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_proxy_uuid_playercount_post

> api_proxy_uuid_playercount_post(uuid, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**body** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \ServerApi

All URIs are relative to *http://localhost:8888*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_onlinecount_get**](ServerApi.md#api_onlinecount_get) | **GET** /api/onlinecount | 
[**api_servers_broadcast_post**](ServerApi.md#api_servers_broadcast_post) | **POST** /api/servers/broadcast | 
[**api_servers_get**](ServerApi.md#api_servers_get) | **GET** /api/servers | 
[**api_servers_label_delete**](ServerApi.md#api_servers_label_delete) | **DELETE** /api/servers/{label} | 
[**api_servers_label_get**](ServerApi.md#api_servers_label_get) | **GET** /api/servers/{label} | 
[**api_servers_post**](ServerApi.md#api_servers_post) | **POST** /api/servers | 
[**api_servers_uuid_playercount_post**](ServerApi.md#api_servers_uuid_playercount_post) | **POST** /api/servers/{uuid}/playercount | 
[**api_servers_uuid_setdescription_post**](ServerApi.md#api_servers_uuid_setdescription_post) | **POST** /api/servers/{uuid}/setdescription | 
[**api_servers_uuid_setstate_post**](ServerApi.md#api_servers_uuid_setstate_post) | **POST** /api/servers/{uuid}/setstate | 



## api_onlinecount_get

> i32 api_onlinecount_get()


### Parameters

This endpoint does not need any parameter.

### Return type

**i32**

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_servers_broadcast_post

> api_servers_broadcast_post(broadcast)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**broadcast** | [**Broadcast**](Broadcast.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_servers_get

> Vec<crate::models::Server> api_servers_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Server>**](Server.md)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_servers_label_delete

> api_servers_label_delete(label)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**label** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_servers_label_get

> crate::models::Server api_servers_label_get(label)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**label** | **String** |  | [required] |

### Return type

[**crate::models::Server**](Server.md)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_servers_post

> String api_servers_post(create_server)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_server** | [**CreateServer**](CreateServer.md) |  | [required] |

### Return type

**String**

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_servers_uuid_playercount_post

> api_servers_uuid_playercount_post(uuid, body)


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


## api_servers_uuid_setdescription_post

> api_servers_uuid_setdescription_post(uuid, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**body** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_servers_uuid_setstate_post

> api_servers_uuid_setstate_post(uuid, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**body** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


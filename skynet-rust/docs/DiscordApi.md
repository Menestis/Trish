# \DiscordApi

All URIs are relative to *http://localhost:8888*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_discord_link_code_post**](DiscordApi.md#api_discord_link_code_post) | **POST** /api/discord/link/{code} | 
[**api_discord_link_discord_delete**](DiscordApi.md#api_discord_link_discord_delete) | **DELETE** /api/discord/link/{discord} | 
[**api_discord_link_uuid_get**](DiscordApi.md#api_discord_link_uuid_get) | **GET** /api/discord/link/{uuid} | 
[**api_discord_webhook_name_post**](DiscordApi.md#api_discord_webhook_name_post) | **POST** /api/discord/webhook/{name} | 



## api_discord_link_code_post

> api_discord_link_code_post(code, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |
**body** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_discord_link_discord_delete

> api_discord_link_discord_delete(discord)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**discord** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_discord_link_uuid_get

> String api_discord_link_uuid_get(uuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |

### Return type

**String**

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_discord_webhook_name_post

> api_discord_webhook_name_post(name, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**body** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


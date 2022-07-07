# \SessionApi

All URIs are relative to *http://localhost:8888*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_players_uuid_session_delete**](SessionApi.md#api_players_uuid_session_delete) | **DELETE** /api/players/{uuid}/session | 
[**api_sessions_session_clientbrand_post**](SessionApi.md#api_sessions_session_clientbrand_post) | **POST** /api/sessions/{session}/clientbrand | 
[**api_sessions_session_modsinfo_post**](SessionApi.md#api_sessions_session_modsinfo_post) | **POST** /api/sessions/{session}/modsinfo | 



## api_players_uuid_session_delete

> api_players_uuid_session_delete(uuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_sessions_session_clientbrand_post

> api_sessions_session_clientbrand_post(session, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_sessions_session_modsinfo_post

> api_sessions_session_modsinfo_post(session, mod_info)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session** | **String** |  | [required] |
**mod_info** | Option<[**Vec<crate::models::ModInfo>**](ModInfo.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


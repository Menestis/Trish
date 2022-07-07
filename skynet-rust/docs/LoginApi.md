# \LoginApi

All URIs are relative to *http://localhost:8888*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_players_ip_proxy_prelogin_get**](LoginApi.md#api_players_ip_proxy_prelogin_get) | **GET** /api/players/{ip}/proxy/prelogin | 
[**api_players_uuid_login_post**](LoginApi.md#api_players_uuid_login_post) | **POST** /api/players/{uuid}/login | 
[**api_players_uuid_proxy_login_post**](LoginApi.md#api_players_uuid_proxy_login_post) | **POST** /api/players/{uuid}/proxy/login | 
[**api_sessions_session_clientbrand_post**](LoginApi.md#api_sessions_session_clientbrand_post) | **POST** /api/sessions/{session}/clientbrand | 
[**api_sessions_session_modsinfo_post**](LoginApi.md#api_sessions_session_modsinfo_post) | **POST** /api/sessions/{session}/modsinfo | 



## api_players_ip_proxy_prelogin_get

> crate::models::ProxyPreLoginResponse api_players_ip_proxy_prelogin_get(ip)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip** | **String** |  | [required] |

### Return type

[**crate::models::ProxyPreLoginResponse**](ProxyPreLoginResponse.md)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_uuid_login_post

> crate::models::ServerLoginPlayerInfo api_players_uuid_login_post(uuid, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**body** | **String** |  | [required] |

### Return type

[**crate::models::ServerLoginPlayerInfo**](ServerLoginPlayerInfo.md)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_uuid_proxy_login_post

> crate::models::ProxyLoginResponse api_players_uuid_proxy_login_post(uuid, proxy_login_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**proxy_login_request** | Option<[**ProxyLoginRequest**](ProxyLoginRequest.md)> |  |  |

### Return type

[**crate::models::ProxyLoginResponse**](ProxyLoginResponse.md)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

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


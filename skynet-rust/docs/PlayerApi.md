# \PlayerApi

All URIs are relative to *http://localhost:8888*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_discord_link_code_post**](PlayerApi.md#api_discord_link_code_post) | **POST** /api/discord/link/{code} | 
[**api_discord_link_discord_delete**](PlayerApi.md#api_discord_link_discord_delete) | **DELETE** /api/discord/link/{discord} | 
[**api_discord_link_uuid_get**](PlayerApi.md#api_discord_link_uuid_get) | **GET** /api/discord/link/{uuid} | 
[**api_players_get**](PlayerApi.md#api_players_get) | **GET** /api/players | 
[**api_players_player_get**](PlayerApi.md#api_players_player_get) | **GET** /api/players/{player} | 
[**api_players_uuid_ban_post**](PlayerApi.md#api_players_uuid_ban_post) | **POST** /api/players/{uuid}/ban | 
[**api_players_uuid_disconnect_post**](PlayerApi.md#api_players_uuid_disconnect_post) | **POST** /api/players/{uuid}/disconnect | 
[**api_players_uuid_groups_update_post**](PlayerApi.md#api_players_uuid_groups_update_post) | **POST** /api/players/{uuid}/groups/update | 
[**api_players_uuid_inventory_transaction_post**](PlayerApi.md#api_players_uuid_inventory_transaction_post) | **POST** /api/players/{uuid}/inventory/transaction | 
[**api_players_uuid_move_post**](PlayerApi.md#api_players_uuid_move_post) | **POST** /api/players/{uuid}/move | 
[**api_players_uuid_mute_post**](PlayerApi.md#api_players_uuid_mute_post) | **POST** /api/players/{uuid}/mute | 
[**api_players_uuid_stats_post**](PlayerApi.md#api_players_uuid_stats_post) | **POST** /api/players/{uuid}/stats | 
[**api_players_uuid_transaction_post**](PlayerApi.md#api_players_uuid_transaction_post) | **POST** /api/players/{uuid}/transaction | 



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


## api_players_get

> Vec<crate::models::OnlinePlayerInfo> api_players_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::OnlinePlayerInfo>**](OnlinePlayerInfo.md)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_player_get

> crate::models::PlayerInfo api_players_player_get(player)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player** | **String** |  | [required] |

### Return type

[**crate::models::PlayerInfo**](PlayerInfo.md)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_uuid_ban_post

> api_players_uuid_ban_post(uuid, player_ban)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**player_ban** | [**PlayerBan**](PlayerBan.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_uuid_disconnect_post

> api_players_uuid_disconnect_post(uuid)


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


## api_players_uuid_groups_update_post

> api_players_uuid_groups_update_post(uuid, request_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**request_body** | [**Vec<String>**](String.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_uuid_inventory_transaction_post

> bool api_players_uuid_inventory_transaction_post(uuid, request_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**request_body** | [**::std::collections::HashMap<String, i32>**](i32.md) |  | [required] |

### Return type

**bool**

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_uuid_move_post

> String api_players_uuid_move_post(uuid, player_move)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**player_move** | [**PlayerMove**](PlayerMove.md) |  | [required] |

### Return type

**String**

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_uuid_mute_post

> api_players_uuid_mute_post(uuid, player_mute)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**player_mute** | [**PlayerMute**](PlayerMute.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_uuid_stats_post

> api_players_uuid_stats_post(uuid, player_stats)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**player_stats** | [**PlayerStats**](PlayerStats.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_uuid_transaction_post

> bool api_players_uuid_transaction_post(uuid, player_transaction)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**player_transaction** | [**PlayerTransaction**](PlayerTransaction.md) |  | [required] |

### Return type

**bool**

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


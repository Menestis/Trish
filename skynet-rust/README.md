# Rust API client for skynet-api

This is the skynet infrastructure api


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.1
- Package version: 1.0.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `skynet-api` and add the following to `Cargo.toml` under `[dependencies]`:

```
skynet-api = { path = "./skynet-api" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost:8888*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AdminApi* | [**api_shutdown_post**](docs/AdminApi.md#api_shutdown_post) | **POST** /api/shutdown | 
*DiscordApi* | [**api_discord_link_code_post**](docs/DiscordApi.md#api_discord_link_code_post) | **POST** /api/discord/link/{code} | 
*DiscordApi* | [**api_discord_link_discord_delete**](docs/DiscordApi.md#api_discord_link_discord_delete) | **DELETE** /api/discord/link/{discord} | 
*DiscordApi* | [**api_discord_link_uuid_get**](docs/DiscordApi.md#api_discord_link_uuid_get) | **GET** /api/discord/link/{uuid} | 
*DiscordApi* | [**api_discord_webhook_name_post**](docs/DiscordApi.md#api_discord_webhook_name_post) | **POST** /api/discord/webhook/{name} | 
*LoginApi* | [**api_players_ip_proxy_prelogin_get**](docs/LoginApi.md#api_players_ip_proxy_prelogin_get) | **GET** /api/players/{ip}/proxy/prelogin | 
*LoginApi* | [**api_players_uuid_login_post**](docs/LoginApi.md#api_players_uuid_login_post) | **POST** /api/players/{uuid}/login | 
*LoginApi* | [**api_players_uuid_proxy_login_post**](docs/LoginApi.md#api_players_uuid_proxy_login_post) | **POST** /api/players/{uuid}/proxy/login | 
*LoginApi* | [**api_sessions_session_clientbrand_post**](docs/LoginApi.md#api_sessions_session_clientbrand_post) | **POST** /api/sessions/{session}/clientbrand | 
*LoginApi* | [**api_sessions_session_modsinfo_post**](docs/LoginApi.md#api_sessions_session_modsinfo_post) | **POST** /api/sessions/{session}/modsinfo | 
*PlayerApi* | [**api_discord_link_code_post**](docs/PlayerApi.md#api_discord_link_code_post) | **POST** /api/discord/link/{code} | 
*PlayerApi* | [**api_discord_link_discord_delete**](docs/PlayerApi.md#api_discord_link_discord_delete) | **DELETE** /api/discord/link/{discord} | 
*PlayerApi* | [**api_discord_link_uuid_get**](docs/PlayerApi.md#api_discord_link_uuid_get) | **GET** /api/discord/link/{uuid} | 
*PlayerApi* | [**api_players_get**](docs/PlayerApi.md#api_players_get) | **GET** /api/players | 
*PlayerApi* | [**api_players_player_get**](docs/PlayerApi.md#api_players_player_get) | **GET** /api/players/{player} | 
*PlayerApi* | [**api_players_uuid_ban_post**](docs/PlayerApi.md#api_players_uuid_ban_post) | **POST** /api/players/{uuid}/ban | 
*PlayerApi* | [**api_players_uuid_disconnect_post**](docs/PlayerApi.md#api_players_uuid_disconnect_post) | **POST** /api/players/{uuid}/disconnect | 
*PlayerApi* | [**api_players_uuid_groups_update_post**](docs/PlayerApi.md#api_players_uuid_groups_update_post) | **POST** /api/players/{uuid}/groups/update | 
*PlayerApi* | [**api_players_uuid_inventory_transaction_post**](docs/PlayerApi.md#api_players_uuid_inventory_transaction_post) | **POST** /api/players/{uuid}/inventory/transaction | 
*PlayerApi* | [**api_players_uuid_move_post**](docs/PlayerApi.md#api_players_uuid_move_post) | **POST** /api/players/{uuid}/move | 
*PlayerApi* | [**api_players_uuid_mute_post**](docs/PlayerApi.md#api_players_uuid_mute_post) | **POST** /api/players/{uuid}/mute | 
*PlayerApi* | [**api_players_uuid_stats_post**](docs/PlayerApi.md#api_players_uuid_stats_post) | **POST** /api/players/{uuid}/stats | 
*PlayerApi* | [**api_players_uuid_transaction_post**](docs/PlayerApi.md#api_players_uuid_transaction_post) | **POST** /api/players/{uuid}/transaction | 
*ProxyApi* | [**api_proxy_ping_get**](docs/ProxyApi.md#api_proxy_ping_get) | **GET** /api/proxy/ping | 
*ProxyApi* | [**api_proxy_uuid_playercount_post**](docs/ProxyApi.md#api_proxy_uuid_playercount_post) | **POST** /api/proxy/{uuid}/playercount | 
*RegistrationApi* | [**api_servers_label_register_get**](docs/RegistrationApi.md#api_servers_label_register_get) | **GET** /api/servers/{label}/register | 
*ServerApi* | [**api_onlinecount_get**](docs/ServerApi.md#api_onlinecount_get) | **GET** /api/onlinecount | 
*ServerApi* | [**api_servers_broadcast_post**](docs/ServerApi.md#api_servers_broadcast_post) | **POST** /api/servers/broadcast | 
*ServerApi* | [**api_servers_get**](docs/ServerApi.md#api_servers_get) | **GET** /api/servers | 
*ServerApi* | [**api_servers_label_delete**](docs/ServerApi.md#api_servers_label_delete) | **DELETE** /api/servers/{label} | 
*ServerApi* | [**api_servers_label_get**](docs/ServerApi.md#api_servers_label_get) | **GET** /api/servers/{label} | 
*ServerApi* | [**api_servers_post**](docs/ServerApi.md#api_servers_post) | **POST** /api/servers | 
*ServerApi* | [**api_servers_uuid_playercount_post**](docs/ServerApi.md#api_servers_uuid_playercount_post) | **POST** /api/servers/{uuid}/playercount | 
*ServerApi* | [**api_servers_uuid_setdescription_post**](docs/ServerApi.md#api_servers_uuid_setdescription_post) | **POST** /api/servers/{uuid}/setdescription | 
*ServerApi* | [**api_servers_uuid_setstate_post**](docs/ServerApi.md#api_servers_uuid_setstate_post) | **POST** /api/servers/{uuid}/setstate | 
*SessionApi* | [**api_players_uuid_session_delete**](docs/SessionApi.md#api_players_uuid_session_delete) | **DELETE** /api/players/{uuid}/session | 
*SessionApi* | [**api_sessions_session_clientbrand_post**](docs/SessionApi.md#api_sessions_session_clientbrand_post) | **POST** /api/sessions/{session}/clientbrand | 
*SessionApi* | [**api_sessions_session_modsinfo_post**](docs/SessionApi.md#api_sessions_session_modsinfo_post) | **POST** /api/sessions/{session}/modsinfo | 
*StatsApi* | [**api_leaderboards_name_post**](docs/StatsApi.md#api_leaderboards_name_post) | **POST** /api/leaderboards/{name} | 
*StatsApi* | [**api_leaderboards_post**](docs/StatsApi.md#api_leaderboards_post) | **POST** /api/leaderboards | 
*StatusApi* | [**status_get**](docs/StatusApi.md#status_get) | **GET** /status | 


## Documentation For Models

 - [Ban](docs/Ban.md)
 - [Broadcast](docs/Broadcast.md)
 - [CreateServer](docs/CreateServer.md)
 - [Leaderboard](docs/Leaderboard.md)
 - [MessageComponent](docs/MessageComponent.md)
 - [MessageComponentModifiers](docs/MessageComponentModifiers.md)
 - [ModInfo](docs/ModInfo.md)
 - [Mute](docs/Mute.md)
 - [OnlinePlayerInfo](docs/OnlinePlayerInfo.md)
 - [PingInfo](docs/PingInfo.md)
 - [PlayerBan](docs/PlayerBan.md)
 - [PlayerInfo](docs/PlayerInfo.md)
 - [PlayerMove](docs/PlayerMove.md)
 - [PlayerMute](docs/PlayerMute.md)
 - [PlayerStats](docs/PlayerStats.md)
 - [PlayerTransaction](docs/PlayerTransaction.md)
 - [ProxyLoginPlayerInfo](docs/ProxyLoginPlayerInfo.md)
 - [ProxyLoginRequest](docs/ProxyLoginRequest.md)
 - [ProxyLoginResponse](docs/ProxyLoginResponse.md)
 - [ProxyPreLoginResponse](docs/ProxyPreLoginResponse.md)
 - [Server](docs/Server.md)
 - [ServerLoginPlayerInfo](docs/ServerLoginPlayerInfo.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

contact@guillaume-etheve.fr


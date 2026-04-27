# API 文档总览

本目录整理当前项目 `crates/web-server` 中已经暴露的全部 HTTP API。

## 文档目录

- `common.md`: 通用约定，包括鉴权、统一响应格式、错误码
- `admin.md`: 后台 RBAC 相关接口
- `app.md`: App 用户、App 角色、App 权限配置接口
- `ws.md`: WebSocket 连接与小铃铛通知推送协议

## 路由前缀

- 后台管理接口前缀：`/api/admin`
- App 管理接口前缀：`/api/admin/app`
- App 用户接口前缀：`/api/app`
- WebSocket 连接路径：`/api/ws`

## 当前接口清单

### Admin

1. `POST /api/admin/users`
2. `GET /api/admin/users`
3. `PATCH /api/admin/users/{user_id}`
4. `DELETE /api/admin/users/{user_id}`
5. `POST /api/admin/roles`
6. `GET /api/admin/roles`
7. `DELETE /api/admin/roles/{role_id}`
8. `GET /api/admin/permissions`
9. `GET /api/admin/roles/{role_id}/permissions`
10. `PUT /api/admin/roles/{role_id}/permissions`
11. `POST /api/admin/menus`
12. `GET /api/admin/menus`
13. `GET /api/admin/users/{user_id}/roles`
14. `PUT /api/admin/users/{user_id}/roles`
15. `GET /api/admin/me/permissions`
16. `GET /api/admin/me/menus`

### App

1. `POST /api/admin/app/users`
2. `GET /api/admin/app/users`
3. `PATCH /api/admin/app/users/{user_id}`
4. `DELETE /api/admin/app/users/{user_id}`
5. `GET /api/admin/app/users/{user_id}/roles`
6. `PUT /api/admin/app/users/{user_id}/roles`
7. `POST /api/admin/app/roles`
8. `GET /api/admin/app/roles`
9. `DELETE /api/admin/app/roles/{role_id}`
10. `GET /api/admin/app/permissions`
11. `GET /api/admin/app/roles/{role_id}/permissions`
12. `PUT /api/admin/app/roles/{role_id}/permissions`
13. `GET /api/app/me/permissions`

### WebSocket

1. `GET /api/ws` WebSocket upgrade

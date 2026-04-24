# API 文档总览

本目录整理当前项目 `crates/web-server` 中已经暴露的全部 HTTP API。

## 文档目录

- `common.md`: 通用约定，包括鉴权、统一响应格式、错误码
- `admin.md`: 后台 RBAC 相关接口

## 路由前缀

- 后台管理接口前缀：`/api/admin`

## 当前接口清单

### Admin

1. `POST /api/admin/admin-users`
2. `GET /api/admin/admin-users`
3. `PATCH /api/admin/admin-users/{user_id}`
4. `DELETE /api/admin/admin-users/{user_id}`
5. `POST /api/admin/roles`
6. `GET /api/admin/roles`
7. `DELETE /api/admin/roles/{role_id}`
8. `POST /api/admin/permissions`
9. `GET /api/admin/permissions`
10. `POST /api/admin/menus`
11. `GET /api/admin/menus`
12. `POST /api/admin/user-roles`
13. `GET /api/admin/users/{user_id}/roles`
14. `POST /api/admin/role-permissions`
15. `GET /api/admin/me/permissions`
16. `GET /api/admin/me/menus`

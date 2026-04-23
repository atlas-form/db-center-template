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
3. `POST /api/admin/roles`
4. `GET /api/admin/roles`
5. `POST /api/admin/permissions`
6. `GET /api/admin/permissions`
7. `POST /api/admin/menus`
8. `GET /api/admin/menus`
9. `POST /api/admin/user-roles`
10. `GET /api/admin/users/{user_id}/roles`
11. `POST /api/admin/role-permissions`
12. `GET /api/admin/me/permissions`
13. `GET /api/admin/me/menus`

# App API

后台管理接口前缀：`/api/admin/app`

App 用户接口前缀：`/api/app`

说明：本组接口用于后台管理员管理 App 用户、App 角色、App 角色权限，以及 App 用户查询自己的权限码。这里的 App 权限来自 `app_permissions` 表，与后台菜单权限 `admin_permissions` 是两套权限体系。

## 后台权限码

访问 `/api/admin/app/*` 接口时，当前登录用户必须是有效后台用户，并拥有对应的后台菜单级权限。

| 后台权限码 | 用途 |
| --- | --- |
| `accounts:app_users` | App Users 菜单；管理 App 用户及 App 用户角色 |
| `access_control:app_roles` | App Roles 菜单；管理 App 角色 |
| `access_control:app_role_permissions` | App Role Permissions 菜单；查看 App 权限树、配置 App 角色权限 |

## App 权限码

当前初始化脚本 `scripts/init_app_permissions.sh` 会写入以下 App 权限：

| App 权限码 | 类型 | 用途 |
| --- | --- | --- |
| `profile` | `group` | 个人中心 |
| `profile:view` | `action` | 查看个人资料 |
| `profile:update` | `action` | 更新个人资料 |

## 统一返回说明

所有 HTTP 接口都返回 `CommonResponse<T>`，成功时结构类似：

```json
{
  "code": 0,
  "message": "ok",
  "data": {}
}
```

## 1. 创建 App 用户

- 方法：`POST`
- 路径：`/api/admin/app/users`
- 后台权限：`accounts:app_users`

请求体：

```json
{
  "identifier": "zhangsan",
  "remark": "测试 App 用户"
}
```

字段说明：

| 字段 | 类型 | 必填 | 约束 |
| --- | --- | --- | --- |
| `identifier` | `string` | 是 | 长度 `1..=128`，用于调用 auth 服务定位用户 |
| `remark` | `string \| null` | 否 | 有值时长度 `1..=255` |

成功响应 `data`：

```json
{
  "user_id": "1b1f4e1d-5b4f-4d25-ae07-520f587f8d13",
  "display_id": "zhangsan",
  "display_name": "张三",
  "remark": "测试 App 用户",
  "status": "enabled",
  "roles": []
}
```

补充说明：

- 接口会先通过 `identifier` 查询 auth 用户，再写入 `app_users`

## 2. 查询 App 用户列表

- 方法：`GET`
- 路径：`/api/admin/app/users`
- 后台权限：`accounts:app_users`

成功响应 `data`：

```json
[
  {
    "user_id": "1b1f4e1d-5b4f-4d25-ae07-520f587f8d13",
    "display_id": "zhangsan",
    "display_name": "张三",
    "remark": "测试 App 用户",
    "status": "enabled",
    "roles": [
      {
        "id": 1,
        "name": "普通用户",
        "code": "user"
      }
    ]
  }
]
```

## 3. 更新 App 用户

- 方法：`PATCH`
- 路径：`/api/admin/app/users/{user_id}`
- 后台权限：`accounts:app_users`

请求体：

```json
{
  "remark": "新的备注",
  "status": "disabled"
}
```

字段说明：

| 字段 | 类型 | 必填 | 约束 |
| --- | --- | --- | --- |
| `remark` | `string \| null` | 否 | 有值时长度 `1..=255` |
| `status` | `enabled \| disabled` | 是 | App 用户状态 |

成功响应 `data`：同“创建 App 用户”。

## 4. 删除 App 用户

- 方法：`DELETE`
- 路径：`/api/admin/app/users/{user_id}`
- 后台权限：`accounts:app_users`

成功响应 `data`：

```json
null
```

补充说明：

- 删除 App 用户时会同步删除该用户的角色绑定

## 5. 创建 App 角色

- 方法：`POST`
- 路径：`/api/admin/app/roles`
- 后台权限：`access_control:app_roles`

请求体：

```json
{
  "name": "普通用户",
  "code": "user"
}
```

字段说明：

| 字段 | 类型 | 必填 | 约束 |
| --- | --- | --- | --- |
| `name` | `string` | 是 | 长度 `1..=64` |
| `code` | `string` | 是 | 长度 `1..=64` |

成功响应 `data`：

```json
{
  "id": 1,
  "name": "普通用户",
  "code": "user"
}
```

## 6. 查询 App 角色列表

- 方法：`GET`
- 路径：`/api/admin/app/roles`
- 后台权限：`access_control:app_roles`

成功响应 `data`：

```json
[
  {
    "id": 1,
    "name": "普通用户",
    "code": "user"
  }
]
```

## 7. 删除 App 角色

- 方法：`DELETE`
- 路径：`/api/admin/app/roles/{role_id}`
- 后台权限：`access_control:app_roles`

成功响应 `data`：

```json
null
```

补充说明：

- 删除角色时会同步删除该角色的用户绑定和权限绑定

## 8. 查询 App 权限树

- 方法：`GET`
- 路径：`/api/admin/app/permissions`
- 后台权限：`access_control:app_role_permissions`

成功响应 `data`：

```json
[
  {
    "id": 1,
    "name": "个人中心",
    "kind": "group",
    "children": [
      {
        "id": 2,
        "name": "查看个人资料",
        "kind": "action",
        "children": []
      },
      {
        "id": 3,
        "name": "更新个人资料",
        "kind": "action",
        "children": []
      }
    ]
  }
]
```

## 9. 查询 App 角色权限配置

- 方法：`GET`
- 路径：`/api/admin/app/roles/{role_id}/permissions`
- 后台权限：`access_control:app_role_permissions`

成功响应 `data`：

```json
[
  {
    "id": 1,
    "name": "个人中心",
    "kind": "group",
    "checked": true,
    "children": [
      {
        "id": 2,
        "name": "查看个人资料",
        "kind": "action",
        "checked": true,
        "children": []
      }
    ]
  }
]
```

## 10. 更新 App 角色权限配置

- 方法：`PUT`
- 路径：`/api/admin/app/roles/{role_id}/permissions`
- 后台权限：`access_control:app_role_permissions`

请求体：

```json
{
  "permission_ids": [1, 2, 3]
}
```

字段说明：

| 字段 | 类型 | 必填 | 约束 |
| --- | --- | --- | --- |
| `permission_ids` | `integer[]` | 是 | 前端权限树中勾选的 App 权限 ID |

成功响应 `data`：同“查询 App 角色权限配置”。

补充说明：

- 更新不是增量修改
- 服务端会先删除该角色下所有 `app_role_permissions` 记录，再插入本次提交的 `permission_ids`

## 11. 查询 App 用户角色配置

- 方法：`GET`
- 路径：`/api/admin/app/users/{user_id}/roles`
- 后台权限：`accounts:app_users`

成功响应 `data`：

```json
[
  {
    "id": 1,
    "name": "普通用户",
    "code": "user",
    "checked": true
  }
]
```

## 12. 更新 App 用户角色配置

- 方法：`PUT`
- 路径：`/api/admin/app/users/{user_id}/roles`
- 后台权限：`accounts:app_users`

请求体：

```json
{
  "role_ids": [1, 2]
}
```

字段说明：

| 字段 | 类型 | 必填 | 约束 |
| --- | --- | --- | --- |
| `role_ids` | `integer[]` | 是 | 前端角色配置中勾选的 App 角色 ID |

成功响应 `data`：同“查询 App 用户角色配置”。

补充说明：

- 更新不是增量修改
- 服务端会先删除该 `user_id` 下所有 `app_user_roles` 记录，再插入本次提交的 `role_ids`

## 13. 查询当前 App 用户权限

- 方法：`GET`
- 路径：`/api/app/me/permissions`
- 鉴权：有效 App 用户 JWT

成功响应 `data`：

```json
{
  "user_id": "1b1f4e1d-5b4f-4d25-ae07-520f587f8d13",
  "role_codes": [
    "user"
  ],
  "permission_codes": [
    "profile",
    "profile:view",
    "profile:update"
  ]
}
```

补充说明：

- 当前用户必须存在于 `app_users`，且状态为 `enabled`
- 返回权限码来自该用户绑定的 App 角色

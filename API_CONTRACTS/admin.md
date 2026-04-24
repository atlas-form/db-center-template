# Admin API

接口前缀：`/api/admin`

说明：本组接口用于 RBAC 管理，包括后台用户、角色、权限、菜单、用户角色绑定，以及当前用户可见权限和菜单查询。

## 权限码总览

| 权限码 | 用途 |
| --- | --- |
| `admin:user:create` | 创建后台用户 |
| `admin:user:list` | 查看后台用户列表 |
| `admin:user:update` | 更新后台用户 |
| `admin:user:delete` | 删除后台用户 |
| `admin:role:create` | 创建角色 |
| `admin:role:list` | 查看角色列表 |
| `admin:role:delete` | 删除角色 |
| `admin:permission:create` | 创建权限 |
| `admin:permission:list` | 查看权限列表 |
| `admin:menu:create` | 创建菜单 |
| `admin:menu:list` | 查看菜单列表 |
| `admin:user_role:assign` | 给用户分配角色 |
| `admin:user_role:list` | 查看用户角色 |
| `admin:role_permission:grant` | 给角色授予权限 |

`GET /api/admin/me/permissions` 和 `GET /api/admin/me/menus` 不要求单独权限码，但要求当前用户本身是有效后台用户。

## 统一返回说明

所有接口都返回 `CommonResponse<T>`，成功时结构类似：

```json
{
  "code": 0,
  "message": "ok",
  "data": {}
}
```

## 1. 创建后台用户

- 方法：`POST`
- 路径：`/api/admin/admin-users`
- 权限：`admin:user:create`

请求体：

```json
{
  "identifier": "zhangsan",
  "remark": "测试环境管理员"
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
  "remark": "测试环境管理员",
  "status": "enabled",
  "roles": []
}
```

补充说明：

- 接口会先通过 `identifier` 查询 auth 用户，再写入后台用户表

## 2. 查询后台用户列表

- 方法：`GET`
- 路径：`/api/admin/admin-users`
- 权限：`admin:user:list`

成功响应 `data`：

```json
[
  {
    "user_id": "1b1f4e1d-5b4f-4d25-ae07-520f587f8d13",
    "display_id": "zhangsan",
    "display_name": "张三",
    "remark": "测试环境管理员",
    "status": "enabled",
    "roles": [
      {
        "id": 1,
        "name": "系统管理员",
        "code": "system_admin"
      }
    ]
  }
]
```

补充说明：

- 带有 `root` 角色的用户不会出现在列表中

## 3. 更新后台用户

- 方法：`PATCH`
- 路径：`/api/admin/admin-users/{user_id}`
- 权限：`admin:user:update`

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
| `status` | `enabled \| disabled` | 是 | 后台用户状态 |

成功响应 `data`：

```json
{
  "user_id": "1b1f4e1d-5b4f-4d25-ae07-520f587f8d13",
  "display_id": "zhangsan",
  "display_name": "张三",
  "remark": "新的备注",
  "status": "disabled",
  "roles": [
    {
      "id": 1,
      "name": "系统管理员",
      "code": "system_admin"
    }
  ]
}
```

补充说明：

- 仅允许修改 `remark` 和 `status`
- 非 `root` 用户不能更新带有 `root` 角色的后台用户

## 4. 删除后台用户

- 方法：`DELETE`
- 路径：`/api/admin/admin-users/{user_id}`
- 权限：`admin:user:delete`

成功响应 `data`：

```json
null
```

补充说明：

- 删除后台用户时会同步删除该用户的角色绑定
- 非 `root` 用户不能删除带有 `root` 角色的后台用户

## 5. 创建角色

- 方法：`POST`
- 路径：`/api/admin/roles`
- 权限：`admin:role:create`

请求体：

```json
{
  "name": "系统管理员",
  "code": "system_admin"
}
```

字段说明：

| 字段 | 类型 | 必填 | 约束 |
| --- | --- | --- | --- |
| `name` | `string` | 是 | 长度 `1..=64` |
| `code` | `string` | 是 | 长度 `1..=64`，且不能为保留值 `root` |

成功响应 `data`：

```json
{
  "id": 1,
  "name": "系统管理员",
  "code": "system_admin"
}
```

## 6. 查询角色列表

- 方法：`GET`
- 路径：`/api/admin/roles`
- 权限：`admin:role:list`

补充说明：

- `root` 角色不会出现在列表中

成功响应 `data`：

```json
[
  {
    "id": 1,
    "name": "系统管理员",
    "code": "system_admin"
  }
]
```

## 7. 删除角色

- 方法：`DELETE`
- 路径：`/api/admin/roles/{role_id}`
- 权限：`admin:role:delete`

成功响应 `data`：

```json
null
```

补充说明：

- `root` 角色是保留角色，不能删除
- 删除角色时会同步删除该角色的用户绑定和权限绑定

## 8. 创建权限

- 方法：`POST`
- 路径：`/api/admin/permissions`
- 权限：`admin:permission:create`

请求体：

```json
{
  "code": "device:create",
  "name": "创建设备",
  "parent_code": "device",
  "sort": 10,
  "kind": "action"
}
```

字段说明：

| 字段 | 类型 | 必填 | 约束 |
| --- | --- | --- | --- |
| `code` | `string` | 是 | 长度 `1..=128` |
| `name` | `string` | 是 | 长度 `1..=128` |
| `parent_code` | `string \| null` | 否 | 有值时长度 `1..=128` |
| `sort` | `integer` | 是 | 排序值 |
| `kind` | `string` | 是 | `group` 或 `action` |

成功响应 `data`：

```json
{
  "id": 1,
  "code": "device:create",
  "name": "创建设备",
  "parent_code": "device",
  "sort": 10,
  "kind": "action"
}
```

## 9. 查询权限列表

- 方法：`GET`
- 路径：`/api/admin/permissions`
- 权限：`admin:permission:list`

成功响应 `data`：

```json
[
  {
    "id": 1,
    "code": "device:create",
    "name": "创建设备",
    "parent_code": "device",
    "sort": 10,
    "kind": "action"
  }
]
```

## 10. 创建菜单

- 方法：`POST`
- 路径：`/api/admin/menus`
- 权限：`admin:menu:create`

请求体：

```json
{
  "name": "设备管理",
  "parent_id": null,
  "permission_code": "device:list"
}
```

字段说明：

| 字段 | 类型 | 必填 | 约束 |
| --- | --- | --- | --- |
| `name` | `string` | 是 | 长度 `1..=64` |
| `parent_id` | `integer \| null` | 否 | 父菜单 ID |
| `permission_code` | `string \| null` | 否 | 有值时长度 `1..=128` |

成功响应 `data`：

```json
{
  "id": 1,
  "name": "设备管理",
  "parent_id": null,
  "permission_code": "device:list"
}
```

## 11. 查询菜单列表

- 方法：`GET`
- 路径：`/api/admin/menus`
- 权限：`admin:menu:list`

成功响应 `data`：

```json
[
  {
    "id": 1,
    "name": "设备管理",
    "parent_id": null,
    "permission_code": "device:list"
  }
]
```

## 12. 给用户分配角色

- 方法：`POST`
- 路径：`/api/admin/user-roles`
- 权限：`admin:user_role:assign`

请求体：

```json
{
  "user_id": "1b1f4e1d-5b4f-4d25-ae07-520f587f8d13",
  "role_id": 1
}
```

字段说明：

| 字段 | 类型 | 必填 | 约束 |
| --- | --- | --- | --- |
| `user_id` | `string` | 是 | 长度 `1..=128`，业务层会按 UUID 解析 |
| `role_id` | `integer` | 是 | 角色 ID |

成功响应 `data`：

```json
{
  "user_id": "1b1f4e1d-5b4f-4d25-ae07-520f587f8d13",
  "role_id": 1
}
```

补充说明：

- 非 `root` 用户不能给别人分配 `root` 角色

## 13. 查询某个用户的角色列表

- 方法：`GET`
- 路径：`/api/admin/users/{user_id}/roles`
- 权限：`admin:user_role:list`

路径参数：

| 参数 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| `user_id` | `string` | 是 | 目标用户 ID，业务层会按 UUID 解析 |

成功响应 `data`：

```json
[
  {
    "id": 1,
    "name": "系统管理员",
    "code": "system_admin"
  }
]
```

## 14. 给角色授予权限

- 方法：`POST`
- 路径：`/api/admin/role-permissions`
- 权限：`admin:role_permission:grant`

请求体：

```json
{
  "role_id": 1,
  "permission_code": "device:create"
}
```

字段说明：

| 字段 | 类型 | 必填 | 约束 |
| --- | --- | --- | --- |
| `role_id` | `integer` | 是 | 角色 ID |
| `permission_code` | `string` | 是 | 长度 `1..=128` |

成功响应 `data`：

```json
{
  "role_id": 1,
  "permission_code": "device:create"
}
```

补充说明：

- 非 `root` 用户不能修改 `root` 角色的权限

## 15. 查询当前用户权限

- 方法：`GET`
- 路径：`/api/admin/me/permissions`
- 权限：有效后台用户

成功响应 `data`：

```json
{
  "user_id": "1b1f4e1d-5b4f-4d25-ae07-520f587f8d13",
  "role_codes": [
    "system_admin"
  ],
  "permission_codes": [
    "device:create",
    "device:list"
  ]
}
```

补充说明：

- 如果当前用户拥有 `root` 角色，返回所有权限码

## 16. 查询当前用户菜单树

- 方法：`GET`
- 路径：`/api/admin/me/menus`
- 权限：有效后台用户

成功响应 `data`：

```json
[
  {
    "id": 1,
    "name": "系统设置",
    "parent_id": null,
    "permission_code": null,
    "children": [
      {
        "id": 2,
        "name": "角色管理",
        "parent_id": 1,
        "permission_code": "admin:role:list",
        "children": []
      }
    ]
  }
]
```

补充说明：

- 如果当前用户拥有 `root` 角色，返回全部菜单树
- 非 `root` 用户只返回有权限命中的菜单，以及这些菜单的祖先节点

# Admin API

接口前缀：`/api/admin`

说明：本组接口用于后台用户、角色、权限读取、角色权限配置、菜单、用户角色绑定，以及当前用户可见权限和菜单查询。权限节点由程序员通过代码、迁移或初始化脚本维护；HTTP API 不提供权限节点创建、修改或删除。

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
| `admin:permission:list` | 查看权限配置树 |
| `admin:role_permission:list` | 查看角色权限配置树 |
| `admin:role_permission:update` | 更新角色权限配置 |
| `admin:menu:create` | 创建菜单 |
| `admin:menu:list` | 查看菜单列表 |
| `admin:user_role:list` | 查看用户角色 |
| `admin:user_role:update` | 更新用户角色 |

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
- 路径：`/api/admin/admin-admin_users`
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
  "admin_roles": []
}
```

补充说明：

- 接口会先通过 `identifier` 查询 auth 用户，再写入后台用户表

## 2. 查询后台用户列表

- 方法：`GET`
- 路径：`/api/admin/admin-admin_users`
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
    "admin_roles": [
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
- 路径：`/api/admin/admin-admin_users/{user_id}`
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
  "admin_roles": [
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
- 路径：`/api/admin/admin-admin_users/{user_id}`
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

## 8. 查询总权限配置树

- 方法：`GET`
- 路径：`/api/admin/permissions`
- 权限：`admin:permission:list`

成功响应 `data`：

```json
[
  {
    "id": 1,
    "name": "用户管理",
    "kind": "group",
    "children": [
      {
        "id": 2,
        "name": "查看用户列表",
        "kind": "action",
        "children": []
      }
    ]
  }
]
```

补充说明：

- 返回全部权限节点的树形结构，供角色配置使用
- 不提供权限节点创建、修改或删除接口

## 9. 查询角色权限配置树

- 方法：`GET`
- 路径：`/api/admin/roles/{role_id}/permissions`
- 权限：`admin:role_permission:list`

路径参数：

| 参数 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| `role_id` | `integer` | 是 | 角色 ID |

成功响应 `data`：

```json
[
  {
    "id": 1,
    "name": "用户管理",
    "kind": "group",
    "checked": false,
    "children": [
      {
        "id": 2,
        "name": "查看用户列表",
        "kind": "action",
        "checked": true,
        "children": []
      }
    ]
  }
]
```

补充说明：

- 返回树形结构给前端配置使用
- 不返回权限码，避免后台直接感知内部权限命名

## 10. 更新角色权限配置

- 方法：`PUT`
- 路径：`/api/admin/roles/{role_id}/permissions`
- 权限：`admin:role_permission:update`

请求体：

```json
{
  "permission_ids": [2, 3, 4]
}
```

字段说明：

| 字段 | 类型 | 必填 | 约束 |
| --- | --- | --- | --- |
| `permission_ids` | `integer[]` | 是 | 前端权限树中勾选的权限节点 ID |

成功响应 `data`：同“查询角色权限配置树”。

补充说明：

- 更新不是增量修改
- 服务端会先删除该 `role_id` 下所有 `admin_role_permissions` 记录，再插入本次提交的 `permission_ids`
- 非 `root` 用户不能修改 `root` 角色的权限

## 11. 创建菜单

- 方法：`POST`
- 路径：`/api/admin/menus`
- 权限：`admin:menu:create`

请求体：

```json
{
  "name": "设备管理",
  "parent_id": null
}
```

字段说明：

| 字段 | 类型 | 必填 | 约束 |
| --- | --- | --- | --- |
| `name` | `string` | 是 | 长度 `1..=64` |
| `parent_id` | `integer \| null` | 否 | 父菜单 ID |

成功响应 `data`：

```json
{
  "id": 1,
  "name": "设备管理",
  "parent_id": null
}
```

## 12. 查询菜单列表

- 方法：`GET`
- 路径：`/api/admin/menus`
- 权限：`admin:menu:list`

成功响应 `data`：

```json
[
  {
    "id": 1,
    "name": "设备管理",
    "parent_id": null
  }
]
```

## 13. 查询用户角色配置

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
    "code": "system_admin",
    "checked": true
  },
  {
    "id": 2,
    "name": "客服",
    "code": "support",
    "checked": false
  }
]
```

补充说明：

- 返回所有可配置角色，并用 `checked` 标识目标用户是否拥有该角色
- 非 `root` 用户不会看到 `root` 角色

## 14. 更新用户角色配置

- 方法：`PUT`
- 路径：`/api/admin/users/{user_id}/roles`
- 权限：`admin:user_role:update`

请求体：

```json
{
  "role_ids": [1, 2]
}
```

字段说明：

| 字段 | 类型 | 必填 | 约束 |
| --- | --- | --- | --- |
| `role_ids` | `integer[]` | 是 | 前端角色配置中勾选的角色 ID |

成功响应 `data`：同“查询用户角色配置”。

补充说明：

- 更新不是增量修改
- 服务端会先删除该 `user_id` 下所有 `admin_user_roles` 记录，再插入本次提交的 `role_ids`
- 非 `root` 用户不能给别人分配 `root` 角色
- 非 `root` 用户不能修改带有 `root` 角色的后台用户

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
    "admin:user:list",
    "admin:role:list"
  ]
}
```

补充说明：

- 登录后前端可用此接口判断当前用户能力
- `root` 角色不受角色权限配置管理，`admin_role_permissions` 不需要保存 `root` 权限
- 如果当前用户拥有 `root` 角色，服务端按最高权限处理，并返回全部权限码

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
    "children": [
      {
        "id": 2,
        "name": "角色管理",
        "parent_id": 1,
        "children": []
      }
    ]
  }
]
```

补充说明：

- 如果当前用户拥有 `root` 角色，返回全部菜单树
- 非 `root` 用户只返回有权限命中的菜单，以及这些菜单的祖先节点

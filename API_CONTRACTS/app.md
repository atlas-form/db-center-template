# App API

接口前缀：`/api/app`

说明：本文件只记录普通客户端使用的接口。后台管理 App 用户、App 角色、App 角色授权的接口属于后台管理端，统一记录在 `API_CONTRACTS/admin.md`。

## App 权限码

当前初始化脚本 `scripts/init_app_permissions.sh` 会写入以下 App 权限：

| App 权限码 | 类型 | 用途 |
| --- | --- | --- |
| `profile` | `group` | 个人中心 |
| `profile:view` | `action` | 查看个人资料 |
| `profile:update` | `action` | 更新个人资料 |

## 1. 查询当前 App 用户权限

- 方法：`GET`
- 路径：`/api/app/me/permissions`
- 鉴权：有效 App 用户 JWT

成功响应 `data`：

```json
{
  "userId": "1b1f4e1d-5b4f-4d25-ae07-520f587f8d13",
  "roleCodes": [
    "user"
  ],
  "permissionCodes": [
    "profile",
    "profile:view",
    "profile:update"
  ]
}
```

补充说明：

- 当前用户必须存在于 `app_users`，且状态为 `enabled`
- 返回权限码来自该用户绑定的 App 角色

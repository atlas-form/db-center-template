# 脚本说明

这些脚本主要是给 AI 和自动化流程使用的，不是给普通使用者直接操作的。

目标只有一件事：

把数据库相关动作拆成明确、可组合、可重复执行的独立命令。

## 这套脚本能做什么

1. 用 Docker 启动 PostgreSQL
2. 初始化 `DATABASE_URL` 指向的数据库，默认是 `test`
3. 单独执行 SeaORM 迁移命令
4. 单独生成 entity，且不修改数据库
5. 清空整个数据库，或者清空某一张表

## 推荐入口

优先使用项目根目录的 `Makefile`：

```bash
make help
```

## 脚本列表

### `postgres.sh`

管理 PostgreSQL Docker 容器。

支持：

```bash
./scripts/postgres.sh up
./scripts/postgres.sh status
./scripts/postgres.sh stop
./scripts/postgres.sh rm
```

### `init_db.sh`

初始化数据库。

它会根据 `.env` 或 `DATABASE_URL` 中的数据库名创建目标数据库。

默认数据库名是：

```text
test
```

### `migrate.sh`

直接包装 `sea-orm-cli migrate`。

支持：

```bash
./scripts/migrate.sh up
./scripts/migrate.sh down
./scripts/migrate.sh fresh
./scripts/migrate.sh refresh
./scripts/migrate.sh reset
./scripts/migrate.sh status
./scripts/migrate.sh generate create_users
```

### `generate_entity.sh`

根据当前数据库中的已有表结构生成 entity。

注意：

- 这个脚本不会修改数据库
- 它要求数据库中已经有表

### `clear_db.sh`

清空整个数据库的 `public` schema。

### `truncate_table.sh`

清空指定表，并重置自增：

```bash
./scripts/truncate_table.sh your_table
```

### `fresh_db.sh`

执行：

1. `migrate refresh`
2. `generate entity`

适合在确认要重建数据库后使用。

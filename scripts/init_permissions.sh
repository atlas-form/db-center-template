#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
# shellcheck source=./common.sh
. "$SCRIPT_DIR/common.sh"

load_env
parse_database_url

upsert_permission() {
  local code="$1"
  local name="$2"
  local parent_code="$3"
  local sort="$4"
  local kind="$5"

  local parent_sql="NULL"
  if [ -n "$parent_code" ]; then
    parent_sql="'${parent_code}'"
  fi

  run_psql "$DB_NAME" "
    INSERT INTO permissions (code, name, parent_code, sort, kind)
    VALUES ('${code}', '${name}', ${parent_sql}, ${sort}, '${kind}')
    ON CONFLICT (code) DO UPDATE
    SET
      name = EXCLUDED.name,
      parent_code = EXCLUDED.parent_code,
      sort = EXCLUDED.sort,
      kind = EXCLUDED.kind;
  " >/dev/null
}

upsert_menu() {
  local name="$1"
  local code="$2"
  local parent_code="$3"
  local permission_code="$4"
  local sort_hint="$5"

  local parent_id_sql="NULL"
  if [ -n "$parent_code" ]; then
    parent_id_sql="(SELECT id FROM menus WHERE permission_code = '${parent_code}' LIMIT 1)"
  fi

  run_psql "$DB_NAME" "
    UPDATE menus
    SET
      name = '${name}',
      parent_id = ${parent_id_sql},
      permission_code = '${permission_code}'
    WHERE permission_code = '${code}';

    INSERT INTO menus (name, parent_id, permission_code)
    SELECT '${name}', ${parent_id_sql}, '${permission_code}'
    WHERE NOT EXISTS (
      SELECT 1 FROM menus WHERE permission_code = '${code}'
    );
  " >/dev/null

  # Force a deterministic order when listing by id during development.
  if [ -n "$sort_hint" ]; then
    :
  fi
}

echo "初始化基础权限节点..."

run_psql "$DB_NAME" "
  DELETE FROM role_permissions
  WHERE permission_id IN (
    SELECT id
    FROM permissions
    WHERE code IN ('admin:menu:list', 'admin:menu:create')
  );

  DELETE FROM permissions
  WHERE code IN ('admin:menu:list', 'admin:menu:create');
" >/dev/null

upsert_permission "admin:user" "用户管理" "" 100 "group"
upsert_permission "admin:user:list" "查看用户列表" "admin:user" 110 "action"
upsert_permission "admin:user:create" "创建后台用户" "admin:user" 120 "action"
upsert_permission "admin:user_role:list" "查看用户角色" "admin:user" 130 "action"
upsert_permission "admin:user_role:assign" "分配用户角色" "admin:user" 140 "action"

upsert_permission "admin:access" "权限管理" "" 200 "group"
upsert_permission "admin:role:list" "查看角色列表" "admin:access" 210 "action"
upsert_permission "admin:role:create" "创建角色" "admin:access" 220 "action"
upsert_permission "admin:permission:list" "查看权限树" "admin:access" 230 "action"
upsert_permission "admin:permission:create" "创建权限节点" "admin:access" 240 "action"
upsert_permission "admin:role_permission:list" "查看角色权限" "admin:access" 250 "action"
upsert_permission "admin:role_permission:grant" "配置角色权限" "admin:access" 260 "action"

echo "初始化基础菜单..."

upsert_menu "用户管理" "admin:user" "" "admin:user" "100"
upsert_menu "权限管理" "admin:access" "" "admin:access" "200"

PERMISSION_COUNT="$(
  run_psql "$DB_NAME" "
    SELECT COUNT(*)
    FROM permissions
    WHERE code IN (
      'admin:user',
      'admin:user:list',
      'admin:user:create',
      'admin:user_role:list',
      'admin:user_role:assign',
      'admin:access',
      'admin:role:list',
      'admin:role:create',
      'admin:permission:list',
      'admin:permission:create',
      'admin:role_permission:list',
      'admin:role_permission:grant'
    );
  " | tr -d '[:space:]'
)"

MENU_COUNT="$(
  run_psql "$DB_NAME" "
    SELECT COUNT(*)
    FROM menus
    WHERE permission_code IN ('admin:user', 'admin:access');
  " | tr -d '[:space:]'
)"

echo "基础权限初始化完成"
echo "permissions: ${PERMISSION_COUNT}"
echo "menus: ${MENU_COUNT}"

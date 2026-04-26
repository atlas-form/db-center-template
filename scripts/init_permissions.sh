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
    INSERT INTO admin_permissions (code, name, parent_code, sort, kind)
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
    parent_id_sql="(SELECT id FROM admin_menus WHERE permission_code = '${parent_code}' LIMIT 1)"
  fi

  run_psql "$DB_NAME" "
    UPDATE admin_menus
    SET
      name = '${name}',
      parent_id = ${parent_id_sql},
      permission_code = '${permission_code}'
    WHERE permission_code = '${code}'
       OR (
         permission_code IS NULL
         AND name = '${name}'
         AND parent_id IS NOT DISTINCT FROM ${parent_id_sql}
       );

    INSERT INTO admin_menus (name, parent_id, permission_code)
    SELECT '${name}', ${parent_id_sql}, '${permission_code}'
    WHERE NOT EXISTS (
      SELECT 1 FROM admin_menus WHERE permission_code = '${code}'
    );
  " >/dev/null

  # Force a deterministic order when listing by id during development.
  if [ -n "$sort_hint" ]; then
    :
  fi
}

echo "初始化基础权限节点..."

run_psql "$DB_NAME" "
  DELETE FROM admin_role_permissions;
  DELETE FROM admin_permissions;
" >/dev/null

upsert_permission "user" "用户管理" "" 100 "group"
upsert_permission "user:list" "查看用户列表" "user" 110 "action"
upsert_permission "user:create" "创建后台用户" "user" 120 "action"
upsert_permission "user:update" "更新后台用户" "user" 130 "action"
upsert_permission "user:delete" "删除后台用户" "user" 140 "action"
upsert_permission "user_role:list" "查看用户角色" "user" 150 "action"
upsert_permission "user_role:update" "更新用户角色" "user" 160 "action"

upsert_permission "access" "权限管理" "" 200 "group"
upsert_permission "role:list" "查看角色列表" "access" 210 "action"
upsert_permission "role:create" "创建角色" "access" 220 "action"
upsert_permission "role:delete" "删除角色" "access" 230 "action"
upsert_permission "permission:list" "查看权限配置树" "access" 240 "action"
upsert_permission "role_permission:list" "查看角色权限配置" "access" 250 "action"
upsert_permission "role_permission:update" "更新角色权限配置" "access" 260 "action"
upsert_permission "menu:list" "查看菜单列表" "access" 270 "action"
upsert_permission "menu:create" "创建菜单" "access" 280 "action"

upsert_permission "app" "普通用户权限管理" "" 300 "group"
upsert_permission "app_user:list" "查看普通用户列表" "app" 310 "action"
upsert_permission "app_user:create" "创建普通用户" "app" 320 "action"
upsert_permission "app_user:update" "更新普通用户" "app" 330 "action"
upsert_permission "app_user:delete" "删除普通用户" "app" 340 "action"
upsert_permission "app_user_role:list" "查看普通用户角色" "app" 350 "action"
upsert_permission "app_user_role:update" "更新普通用户角色" "app" 360 "action"
upsert_permission "app_role:list" "查看普通角色列表" "app" 370 "action"
upsert_permission "app_role:create" "创建普通角色" "app" 380 "action"
upsert_permission "app_role:delete" "删除普通角色" "app" 390 "action"
upsert_permission "app_permission:list" "查看普通权限配置树" "app" 400 "action"
upsert_permission "app_role_permission:list" "查看普通角色权限配置" "app" 410 "action"
upsert_permission "app_role_permission:update" "更新普通角色权限配置" "app" 420 "action"

echo "初始化基础菜单..."

upsert_menu "用户管理" "user" "" "user" "100"
upsert_menu "权限管理" "access" "" "access" "200"
upsert_menu "普通用户权限管理" "app" "" "app" "300"

PERMISSION_COUNT="$(
  run_psql "$DB_NAME" "
    SELECT COUNT(*)
    FROM admin_permissions
    WHERE code IN (
      'user',
      'user:list',
      'user:create',
      'user:update',
      'user:delete',
      'user_role:list',
      'user_role:update',
      'access',
      'role:list',
      'role:create',
      'role:delete',
      'permission:list',
      'role_permission:list',
      'role_permission:update',
      'menu:list',
      'menu:create',
      'app',
      'app_user:list',
      'app_user:create',
      'app_user:update',
      'app_user:delete',
      'app_user_role:list',
      'app_user_role:update',
      'app_role:list',
      'app_role:create',
      'app_role:delete',
      'app_permission:list',
      'app_role_permission:list',
      'app_role_permission:update'
    );
  " | tr -d '[:space:]'
)"

MENU_COUNT="$(
  run_psql "$DB_NAME" "
    SELECT COUNT(*)
    FROM admin_menus
    WHERE permission_code IN ('user', 'access', 'app');
  " | tr -d '[:space:]'
)"

echo "基础权限初始化完成"
echo "admin_permissions: ${PERMISSION_COUNT}"
echo "admin_menus: ${MENU_COUNT}"

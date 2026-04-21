#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
# shellcheck source=./common.sh
. "$SCRIPT_DIR/common.sh"

require_command curl
require_command sed
require_command awk
require_command tr
require_command base64

load_env
parse_database_url

CONFIG_PATH="${CONFIG_PATH:-$PROJECT_ROOT/config/services.toml}"

if [ ! -f "$CONFIG_PATH" ]; then
  echo "错误：找不到配置文件 $CONFIG_PATH"
  exit 1
fi

read_jwt_verify_url() {
  awk '
    /^\[jwt_verify\]/ { in_section=1; next }
    /^\[/ && in_section { exit }
    in_section && $0 ~ /^[[:space:]]*url[[:space:]]*=/ {
      sub(/^[[:space:]]*url[[:space:]]*=[[:space:]]*"/, "", $0)
      sub(/"[[:space:]]*$/, "", $0)
      print
      exit
    }
  ' "$CONFIG_PATH"
}

decode_sub_from_jwt() {
  local token="$1"
  local payload
  payload="$(printf '%s' "$token" | cut -d '.' -f 2)"

  if [ -z "$payload" ] || [ "$payload" = "$token" ]; then
    echo "错误：非法 JWT 格式" >&2
    return 1
  fi

  payload="${payload//-/+}"
  payload="${payload//_/\/}"

  case $(( ${#payload} % 4 )) in
    2) payload="${payload}==" ;;
    3) payload="${payload}=" ;;
    1) echo "错误：非法 JWT payload" >&2; return 1 ;;
  esac

  local decoded
  if ! decoded="$(printf '%s' "$payload" | base64 -d 2>/dev/null)"; then
    decoded="$(printf '%s' "$payload" | base64 -D 2>/dev/null)" || {
      echo "错误：JWT payload base64 解码失败" >&2
      return 1
    }
  fi

  printf '%s' "$decoded" \
    | sed -n 's/.*"sub"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p'
}

extract_access_token() {
  sed -n 's/.*"accessToken"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p'
}

json_escape() {
  printf '%s' "$1" \
    | sed \
        -e 's/\\/\\\\/g' \
        -e 's/"/\\"/g' \
        -e 's/\t/\\t/g' \
        -e 's/\r/\\r/g' \
        -e ':a;N;$!ba;s/\n/\\n/g'
}

JWT_VERIFY_URL="$(read_jwt_verify_url)"
if [ -z "$JWT_VERIFY_URL" ]; then
  echo "错误：config/services.toml 中缺少 jwt_verify.url"
  exit 1
fi

AUTH_BASE_URL="${JWT_VERIFY_URL%/internal/jwt_verify_config}"
LOGIN_URL="${AUTH_BASE_URL}/auth/session/login"

printf "请输入 root 账号（用户名或邮箱）: "
read -r IDENTIFIER

if [ -z "$IDENTIFIER" ]; then
  echo "错误：账号不能为空"
  exit 1
fi

printf "请输入 root 密码: "
stty -echo
read -r PASSWORD
stty echo
printf "\n"

if [ -z "$PASSWORD" ]; then
  echo "错误：密码不能为空"
  exit 1
fi

LOGIN_RESPONSE="$(curl -fsS \
  -H 'Content-Type: application/json' \
  -X POST "$LOGIN_URL" \
  -d "{\"identifier\":\"$(json_escape "$IDENTIFIER")\",\"password\":\"$(json_escape "$PASSWORD")\"}"
)"

ACCESS_TOKEN="$(printf '%s' "$LOGIN_RESPONSE" | extract_access_token)"
USER_ID="$(decode_sub_from_jwt "$ACCESS_TOKEN")"

if [ -z "$USER_ID" ]; then
  echo "错误：无法从 auth access token 中解析 user_id"
  exit 1
fi

ROLE_ID="$(
  run_psql "$DB_NAME" "
    INSERT INTO roles (name, code)
    VALUES ('Root', 'root')
    ON CONFLICT (code) DO UPDATE
    SET name = EXCLUDED.name
    RETURNING id
  " | tr -d '[:space:]'
)"

if [ -z "$ROLE_ID" ]; then
  echo "错误：初始化 root 角色失败"
  exit 1
fi

run_psql "$DB_NAME" "
  INSERT INTO admin_users (user_id, status)
  VALUES ('${USER_ID}', 'enabled')
  ON CONFLICT (user_id) DO UPDATE
  SET status = EXCLUDED.status;
"

run_psql "$DB_NAME" "
  INSERT INTO user_roles (user_id, role_id)
  SELECT '${USER_ID}'::uuid, id
  FROM roles
  WHERE code = 'root'
  ON CONFLICT (user_id, role_id) DO NOTHING;
"

run_psql "$DB_NAME" "
  INSERT INTO role_permissions (role_id, permission_code)
  SELECT roles.id, permissions.code
  FROM roles
  CROSS JOIN permissions
  WHERE roles.code = 'root'
  ON CONFLICT (role_id, permission_code) DO NOTHING;
"

USER_ROLE_COUNT="$(
  run_psql "$DB_NAME" "
    SELECT COUNT(*)
    FROM user_roles
    WHERE user_id = '${USER_ID}'::uuid
      AND role_id = ${ROLE_ID};
  " | tr -d '[:space:]'
)"

if [ "${USER_ROLE_COUNT:-0}" != "1" ]; then
  echo "错误：root 用户角色绑定失败"
  exit 1
fi

echo "root 初始化完成"
echo "auth user_id: ${USER_ID}"
echo "role_id: ${ROLE_ID}"
echo "auth login url: ${LOGIN_URL}"

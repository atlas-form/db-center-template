use std::collections::{BTreeSet, HashMap, HashSet};

use db_core::{
    DbContext,
    error::{BIZ_INTERNAL_ERROR, BizError, BizResult},
};
use repo::table::{
    menus::{CreateMenu, Menu, MenuService},
    permissions::{CreatePermission, PermissionService},
    role_permissions::{CreateRolePermission, RolePermissionService},
    roles::{CreateRole, Role, RoleService},
    user_roles::{CreateUserRole, UserRoleService},
};
use uuid::Uuid;

use crate::dto::admin::{
    AssignUserRoleRequest, CreateMenuRequest, CreatePermissionRequest, CreateRoleRequest,
    CurrentUserPermissionsResponse, GrantRolePermissionRequest, MenuResponse, MenuTreeNode,
    PermissionResponse, RolePermissionResponse, RoleResponse, UserRoleResponse,
};

pub struct AdminApi {
    role_svc: RoleService,
    permission_svc: PermissionService,
    user_role_svc: UserRoleService,
    role_permission_svc: RolePermissionService,
    menu_svc: MenuService,
}

impl AdminApi {
    pub fn new(db: DbContext) -> Self {
        Self {
            role_svc: RoleService::new(db.clone()),
            permission_svc: PermissionService::new(db.clone()),
            user_role_svc: UserRoleService::new(db.clone()),
            role_permission_svc: RolePermissionService::new(db.clone()),
            menu_svc: MenuService::new(db),
        }
    }

    pub async fn create_role(&self, req: CreateRoleRequest) -> BizResult<RoleResponse> {
        let role = self
            .role_svc
            .create(CreateRole {
                name: req.name,
                code: req.code,
            })
            .await?;
        Ok(Self::map_role(role))
    }

    pub async fn list_roles(&self) -> BizResult<Vec<RoleResponse>> {
        Ok(self
            .role_svc
            .list_all()
            .await?
            .into_iter()
            .map(Self::map_role)
            .collect())
    }

    pub async fn create_permission(
        &self,
        req: CreatePermissionRequest,
    ) -> BizResult<PermissionResponse> {
        let permission = self
            .permission_svc
            .create(CreatePermission { code: req.code })
            .await?;
        Ok(PermissionResponse {
            id: permission.id,
            code: permission.code,
        })
    }

    pub async fn list_permissions(&self) -> BizResult<Vec<PermissionResponse>> {
        Ok(self
            .permission_svc
            .list_all()
            .await?
            .into_iter()
            .map(|permission| PermissionResponse {
                id: permission.id,
                code: permission.code,
            })
            .collect())
    }

    pub async fn create_menu(&self, req: CreateMenuRequest) -> BizResult<MenuResponse> {
        let menu = self
            .menu_svc
            .create(CreateMenu {
                name: req.name,
                path: req.path,
                parent_id: req.parent_id,
                permission_code: req.permission_code,
            })
            .await?;
        Ok(Self::map_menu(menu))
    }

    pub async fn list_menus(&self) -> BizResult<Vec<MenuResponse>> {
        Ok(self
            .menu_svc
            .list_all()
            .await?
            .into_iter()
            .map(Self::map_menu)
            .collect())
    }

    pub async fn assign_user_role(
        &self,
        req: AssignUserRoleRequest,
    ) -> BizResult<UserRoleResponse> {
        let user_id = parse_user_id(&req.user_id)?;
        let user_role = self
            .user_role_svc
            .create(CreateUserRole {
                user_id,
                role_id: req.role_id,
            })
            .await?;

        Ok(UserRoleResponse {
            user_id: user_role.user_id.to_string(),
            role_id: user_role.role_id,
        })
    }

    pub async fn list_user_roles(&self, user_id: String) -> BizResult<Vec<RoleResponse>> {
        let user_id = parse_user_id(&user_id)?;
        let user_roles = self.user_role_svc.list_by_user_id(user_id).await?;
        let role_ids = user_roles.into_iter().map(|item| item.role_id).collect();

        Ok(self
            .role_svc
            .list_by_ids(role_ids)
            .await?
            .into_iter()
            .map(Self::map_role)
            .collect())
    }

    pub async fn grant_role_permission(
        &self,
        req: GrantRolePermissionRequest,
    ) -> BizResult<RolePermissionResponse> {
        let role_permission = self
            .role_permission_svc
            .create(CreateRolePermission {
                role_id: req.role_id,
                permission_code: req.permission_code,
            })
            .await?;

        Ok(RolePermissionResponse {
            role_id: role_permission.role_id,
            permission_code: role_permission.permission_code,
        })
    }

    pub async fn get_current_user_permissions(
        &self,
        user_id: String,
    ) -> BizResult<CurrentUserPermissionsResponse> {
        let parsed_user_id = parse_user_id(&user_id)?;
        let user_roles = self.user_role_svc.list_by_user_id(parsed_user_id).await?;
        let role_ids: Vec<i64> = user_roles.iter().map(|item| item.role_id).collect();
        let roles = self.role_svc.list_by_ids(role_ids.clone()).await?;

        let permission_codes = self.collect_permission_codes(role_ids).await?;
        let role_codes = roles.into_iter().map(|role| role.code).collect();

        Ok(CurrentUserPermissionsResponse {
            user_id,
            role_codes,
            permission_codes,
        })
    }

    pub async fn get_current_user_menus(&self, user_id: String) -> BizResult<Vec<MenuTreeNode>> {
        let parsed_user_id = parse_user_id(&user_id)?;
        let user_roles = self.user_role_svc.list_by_user_id(parsed_user_id).await?;
        let role_ids: Vec<i64> = user_roles.into_iter().map(|item| item.role_id).collect();
        let permission_codes: HashSet<String> = self
            .collect_permission_codes(role_ids)
            .await?
            .into_iter()
            .collect();

        let all_menus = self.menu_svc.list_all().await?;
        let menu_map: HashMap<i64, Menu> = all_menus
            .iter()
            .cloned()
            .map(|menu| (menu.id, menu))
            .collect();

        let direct_ids: Vec<i64> = all_menus
            .iter()
            .filter(|menu| {
                menu.permission_code
                    .as_ref()
                    .is_some_and(|code| permission_codes.contains(code))
            })
            .map(|menu| menu.id)
            .collect();

        let mut include_ids: HashSet<i64> = HashSet::new();
        for menu_id in direct_ids {
            self.collect_ancestors(menu_id, &menu_map, &mut include_ids);
        }

        let included_menus: Vec<Menu> = all_menus
            .into_iter()
            .filter(|menu| include_ids.contains(&menu.id))
            .collect();

        Ok(Self::build_menu_tree(included_menus))
    }

    async fn collect_permission_codes(&self, role_ids: Vec<i64>) -> BizResult<Vec<String>> {
        let role_permissions = self.role_permission_svc.list_by_role_ids(role_ids).await?;
        let codes: BTreeSet<String> = role_permissions
            .into_iter()
            .map(|item| item.permission_code)
            .collect();
        Ok(codes.into_iter().collect())
    }

    fn collect_ancestors(
        &self,
        menu_id: i64,
        menu_map: &HashMap<i64, Menu>,
        include_ids: &mut HashSet<i64>,
    ) {
        let mut current = Some(menu_id);
        let mut visited = HashSet::new();

        while let Some(id) = current {
            if !visited.insert(id) {
                break;
            }
            include_ids.insert(id);
            current = menu_map.get(&id).and_then(|menu| menu.parent_id);
        }
    }

    fn build_menu_tree(menus: Vec<Menu>) -> Vec<MenuTreeNode> {
        let nodes: HashMap<i64, MenuTreeNode> = menus
            .into_iter()
            .map(|menu| {
                (
                    menu.id,
                    MenuTreeNode {
                        id: menu.id,
                        name: menu.name,
                        path: menu.path,
                        parent_id: menu.parent_id,
                        permission_code: menu.permission_code,
                        children: Vec::new(),
                    },
                )
            })
            .collect();

        let mut parent_children: HashMap<i64, Vec<i64>> = HashMap::new();
        let mut roots = Vec::new();

        for node in nodes.values() {
            if let Some(parent_id) = node.parent_id {
                parent_children.entry(parent_id).or_default().push(node.id);
            } else {
                roots.push(node.id);
            }
        }

        roots.sort_unstable();
        for children in parent_children.values_mut() {
            children.sort_unstable();
        }

        fn build_node(
            id: i64,
            nodes: &HashMap<i64, MenuTreeNode>,
            parent_children: &HashMap<i64, Vec<i64>>,
        ) -> MenuTreeNode {
            let mut node = nodes.get(&id).cloned().expect("menu node must exist");
            if let Some(child_ids) = parent_children.get(&id) {
                node.children = child_ids
                    .iter()
                    .map(|child_id| build_node(*child_id, nodes, parent_children))
                    .collect();
            }
            node
        }

        roots
            .into_iter()
            .map(|root_id| build_node(root_id, &nodes, &parent_children))
            .collect()
    }

    fn map_role(role: Role) -> RoleResponse {
        RoleResponse {
            id: role.id,
            name: role.name,
            code: role.code,
        }
    }

    fn map_menu(menu: Menu) -> MenuResponse {
        MenuResponse {
            id: menu.id,
            name: menu.name,
            path: menu.path,
            parent_id: menu.parent_id,
            permission_code: menu.permission_code,
        }
    }
}

fn parse_user_id(user_id: &str) -> BizResult<Uuid> {
    Uuid::parse_str(user_id)
        .map_err(|err| BizError::new(BIZ_INTERNAL_ERROR, format!("invalid user_id uuid: {err}")))
}

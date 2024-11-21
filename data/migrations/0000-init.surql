BEGIN TRANSACTION;

REMOVE TABLE IF EXISTS schemas;
DEFINE TABLE schemas SCHEMAFULL;

DEFINE FIELD slug ON TABLE schemas TYPE string;
DEFINE FIELD kind ON TABLE schemas TYPE int default 0; 
DEFINE FIELD title ON TABLE schemas TYPE string;
DEFINE FIELD permission ON TABLE schemas TYPE string DEFAULT 'public';
DEFINE FIELD fields ON TABLE schemas FLEXIBLE TYPE option<array>;
DEFINE FIELD created_at ON TABLE schemas TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE schemas TYPE datetime VALUE time::now();
DEFINE FIELD created_by ON TABLE schemas TYPE string;
DEFINE FIELD updated_by ON TABLE schemas TYPE string;
DEFINE INDEX idx_schemas_slug ON TABLE schemas COLUMNS slug UNIQUE;

CREATE schemas CONTENT {
    slug: 'schemas',
    title: 'Schemas',
    permission: 'schemas',
    created_by: $login,
    updated_by: $login
};

REMOVE TABLE IF EXISTS sessions;

CREATE schemas CONTENT {
    slug: 'sessions',
    title: 'Sessions',
    permission: 'schemas',
    created_by: $login,
    updated_by: $login
};

REMOVE TABLE IF EXISTS mtc_system;
DEFINE TABLE mtc_system SCHEMAFULL;

CREATE schemas CONTENT {
    slug: 'mtc_system',
    title: 'System',
    permission: 'schemas',
    created_by: $login,
    updated_by: $login
};

DEFINE FIELD c_key ON TABLE mtc_system TYPE string;
DEFINE FIELD c_value ON TABLE mtc_system FLEXIBLE;

CREATE mtc_system CONTENT {
    c_key: 'migrations',
    c_value: ['init']
};

REMOVE TABLE IF EXISTS users;
DEFINE TABLE users SCHEMAFULL;

CREATE schemas CONTENT {
    kind: 1,
    slug: 'users',
    title: 'Users',
    permission: 'users',
    created_by: $login,
    updated_by: $login
};

DEFINE FIELD login ON TABLE users TYPE string;
DEFINE FIELD password ON TABLE users TYPE string;
DEFINE FIELD blocked ON TABLE users TYPE bool DEFAULT false;
DEFINE FIELD access_count ON TABLE users TYPE int DEFAULT 0;
DEFINE FIELD last_access ON TABLE users TYPE option<datetime>;
DEFINE FIELD fields ON TABLE users FLEXIBLE TYPE option<object>;
DEFINE FIELD created_at ON TABLE users TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE users TYPE datetime VALUE time::now();
DEFINE FIELD created_by ON TABLE users TYPE string;
DEFINE FIELD updated_by ON TABLE users TYPE string;
DEFINE INDEX idx_users_login ON TABLE users COLUMNS login UNIQUE;

CREATE users CONTENT {
    id: 'sa',
    login: $login,
    password: $password,
    access_level: 0,
    created_by: $login,
    updated_by: $login
};

REMOVE TABLE IF EXISTS roles;
DEFINE TABLE roles SCHEMAFULL;

CREATE schemas CONTENT {
    slug: 'roles',
    title: 'Roles',
    permission: 'roles',
    created_by: $login,
    updated_by: $login
};

DEFINE FIELD slug ON TABLE roles TYPE string;
DEFINE FIELD title ON TABLE roles TYPE string;
DEFINE FIELD user_access_level ON TABLE roles TYPE int DEFAULT 999;
DEFINE FIELD user_access_all ON TABLE roles TYPE bool DEFAULT false;
DEFINE FIELD created_at ON TABLE roles TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE roles TYPE datetime VALUE time::now();
DEFINE FIELD created_by ON TABLE roles TYPE string;
DEFINE FIELD updated_by ON TABLE roles TYPE string;
DEFINE INDEX idx_roles_slug ON TABLE roles COLUMNS slug UNIQUE;

CREATE roles CONTENT {
    id: 'administrator',
    slug: 'administrator',
    title: 'Administrator',
    user_access_level: 0,
    user_access_all: true,
    created_by: $login,
    updated_by: $login
};

CREATE roles CONTENT {
    id: 'writer',
    slug: 'writer',
    title: 'Writer',
    user_access_level: 999,
    user_access_all: false,
    created_by: $login,
    updated_by: $login
};

CREATE roles CONTENT {
    id: 'anonymous',
    slug: 'anonymous',
    title: 'Anonymous',
    user_access_level: 999,
    user_access_all: false,
    created_by: $login,
    updated_by: $login
};

REMOVE TABLE IF EXISTS permissions;
DEFINE TABLE permissions SCHEMAFULL;

CREATE schemas CONTENT {
    slug: 'permissions',
    title: 'Permissions',
    permission: 'roles',
    created_by: $login,
    updated_by: $login
};

DEFINE FIELD slug ON TABLE permissions TYPE string;
DEFINE FIELD is_custom ON TABLE permissions TYPE bool DEFAULT false;
DEFINE FIELD created_by ON TABLE permissions TYPE string DEFAULT $login;
DEFINE FIELD created_at ON TABLE permissions TYPE datetime DEFAULT time::now();
DEFINE INDEX idx_permissions_slug ON TABLE permissions COLUMNS slug UNIQUE;

CREATE permissions CONTENT {
    id: 'public_read',
    slug: 'public::read',
    is_custom: true
};
CREATE permissions CONTENT {
    id: 'public_write',
    slug: 'public::write',
    is_custom: true
};
CREATE permissions CONTENT {
    id: 'public_delete',
    slug: 'public::delete',
    is_custom: true
};
CREATE permissions CONTENT {
    id: 'storage_read',
    slug: 'storage::read'
};
CREATE permissions CONTENT {
    id: 'storage_write',
    slug: 'storage::write'
};
CREATE permissions CONTENT {
    id: 'storage_delete',
    slug: 'storage::delete'
};
CREATE permissions CONTENT {
    id: 'private_storage_read',
    slug: 'private_storage::read'
};
CREATE permissions CONTENT {
    id: 'private_storage_write',
    slug: 'private_storage::write'
};
CREATE permissions CONTENT {
    id: 'private_storage_delete',
    slug: 'private_storage::delete'
};
CREATE permissions CONTENT {
    id: 'roles_read',
    slug: 'roles::read'
};
CREATE permissions CONTENT {
    id: 'roles_write',
    slug: 'roles::write'
};
CREATE permissions CONTENT {
    id: 'roles_delete',
    slug: 'roles::delete'
};
CREATE permissions CONTENT {
    id: 'groups_read',
    slug: 'groups::read'
};
CREATE permissions CONTENT {
    id: 'groups_write',
    slug: 'groups::write'
};
CREATE permissions CONTENT {
    id: 'groups_delete',
    slug: 'groups::delete'
};
CREATE permissions CONTENT {
    id: 'users_read',
    slug: 'users::read'
};
CREATE permissions CONTENT {
    id: 'users_write',
    slug: 'users::write'
};
CREATE permissions CONTENT {
    id: 'users_delete',
    slug: 'users::delete'
};
CREATE permissions CONTENT {
    id: 'schemas_read',
    slug: 'schemas::read'
};
CREATE permissions CONTENT {
    id: 'schemas_write',
    slug: 'schemas::write'
};
CREATE permissions CONTENT {
    id: 'schemas_delete',
    slug: 'schemas::delete'
};
CREATE permissions CONTENT {
    id: 'course_read',
    slug: 'course::read'
};
CREATE permissions CONTENT {
    id: 'course_write',
    slug: 'course::write'
};
CREATE permissions CONTENT {
    id: 'course_delete',
    slug: 'course::delete'
};

REMOVE TABLE IF EXISTS role_permissions;
DEFINE TABLE role_permissions SCHEMAFULL TYPE RELATION IN roles OUT permissions;

CREATE schemas CONTENT {
    slug: 'role_permissions',
    title: 'Role permissions',
    permission: 'roles',
    created_by: $login,
    updated_by: $login
};

DEFINE FIELD created_at ON TABLE role_permissions TYPE datetime VALUE time::now();
DEFINE INDEX idx_role_permissions ON TABLE role_permissions COLUMNS in, out UNIQUE;

RELATE roles:anonymous->role_permissions->permissions:public_read;
RELATE roles:anonymous->role_permissions->permissions:storage_read;

RELATE roles:writer->role_permissions->permissions:public_read;
RELATE roles:writer->role_permissions->permissions:public_write;
RELATE roles:writer->role_permissions->permissions:public_delete;
RELATE roles:writer->role_permissions->permissions:storage_read;
RELATE roles:writer->role_permissions->permissions:storage_write;
RELATE roles:writer->role_permissions->permissions:storage_delete;
RELATE roles:writer->role_permissions->permissions:private_storage_read;
RELATE roles:writer->role_permissions->permissions:private_storage_write;
RELATE roles:writer->role_permissions->permissions:private_storage_delete;
RELATE roles:writer->role_permissions->permissions:course_read;
RELATE roles:writer->role_permissions->permissions:course_write;
RELATE roles:writer->role_permissions->permissions:course_delete;

RELATE roles:administrator->role_permissions->permissions:public_read;
RELATE roles:administrator->role_permissions->permissions:public_write;
RELATE roles:administrator->role_permissions->permissions:public_delete;
RELATE roles:administrator->role_permissions->permissions:storage_read;
RELATE roles:administrator->role_permissions->permissions:storage_write;
RELATE roles:administrator->role_permissions->permissions:storage_delete;
RELATE roles:administrator->role_permissions->permissions:private_storage_read;
RELATE roles:administrator->role_permissions->permissions:private_storage_write;
RELATE roles:administrator->role_permissions->permissions:private_storage_delete;
RELATE roles:administrator->role_permissions->permissions:roles_read;
RELATE roles:administrator->role_permissions->permissions:roles_write;
RELATE roles:administrator->role_permissions->permissions:roles_delete;
RELATE roles:administrator->role_permissions->permissions:groups_read;
RELATE roles:administrator->role_permissions->permissions:groups_write;
RELATE roles:administrator->role_permissions->permissions:groups_delete;
RELATE roles:administrator->role_permissions->permissions:users_read;
RELATE roles:administrator->role_permissions->permissions:users_write;
RELATE roles:administrator->role_permissions->permissions:users_delete;
RELATE roles:administrator->role_permissions->permissions:schemas_read;
RELATE roles:administrator->role_permissions->permissions:schemas_write;
RELATE roles:administrator->role_permissions->permissions:schemas_delete;
RELATE roles:administrator->role_permissions->permissions:course_read;
RELATE roles:administrator->role_permissions->permissions:course_write;
RELATE roles:administrator->role_permissions->permissions:course_delete;

REMOVE TABLE IF EXISTS user_roles;
DEFINE TABLE user_roles SCHEMAFULL TYPE RELATION IN users OUT roles;

CREATE schemas CONTENT {
    slug: 'user_roles',
    title: 'User roles',
    permission: 'roles',
    created_by: $login,
    updated_by: $login
};

DEFINE FIELD created_at ON TABLE user_roles TYPE datetime VALUE time::now();
DEFINE INDEX idx_user_roles ON TABLE user_roles COLUMNS in, out UNIQUE;

RELATE users:sa->user_roles->roles:administrator;
RELATE users:sa->user_roles->roles:writer;

REMOVE TABLE IF EXISTS groups;
DEFINE TABLE groups SCHEMAFULL;

CREATE schemas CONTENT {
    slug: 'groups',
    title: 'Groups',
    permission: 'groups',
    created_by: $login,
    updated_by: $login
};

DEFINE FIELD slug ON TABLE groups TYPE string;
DEFINE FIELD title ON TABLE groups TYPE string;
DEFINE FIELD created_at ON TABLE groups TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE groups TYPE datetime VALUE time::now();
DEFINE FIELD created_by ON TABLE groups TYPE string;
DEFINE FIELD updated_by ON TABLE groups TYPE string;
DEFINE INDEX idx_groups_slug ON TABLE groups COLUMNS slug UNIQUE;

REMOVE TABLE IF EXISTS user_groups;
DEFINE TABLE user_groups SCHEMAFULL TYPE RELATION IN users OUT groups;

CREATE schemas CONTENT {
    slug: 'user_groups',
    title: 'User groups',
    permission: 'groups',
    created_by: $login,
    updated_by: $login
};

DEFINE FIELD created_at ON TABLE user_groups TYPE datetime VALUE time::now();
DEFINE INDEX idx_user_groups ON TABLE user_groups COLUMNS in, out UNIQUE;

REMOVE TABLE IF EXISTS page;
DEFINE TABLE pages SCHEMAFULL;

CREATE schemas CONTENT {
    slug: 'page',
    title: 'Page',
    permission: 'schemas',
    created_by: $login,
    updated_by: $login
};

DEFINE FIELD slug ON TABLE page TYPE string;
DEFINE FIELD title ON TABLE page TYPE string;
DEFINE FIELD data ON TABLE page FLEXIBLE TYPE option<object>;
DEFINE FIELD published ON TABLE page TYPE bool DEFAULT false;
DEFINE FIELD created_at ON TABLE page TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE page TYPE datetime VALUE time::now();
DEFINE FIELD created_by ON TABLE page TYPE string;
DEFINE FIELD updated_by ON TABLE page TYPE string;
DEFINE INDEX idx_page_slug ON TABLE page COLUMNS slug UNIQUE;

CREATE schemas CONTENT {
    slug: 'pages',
    title: 'Pages',
    permission: 'schemas',
    created_by: $login,
    updated_by: $login
};

REMOVE TABLE IF EXISTS course;
DEFINE TABLE course SCHEMAFULL;

CREATE schemas CONTENT {
    slug: 'course',
    title: 'Course',
    permission: 'schemas',
    created_by: $login,
    updated_by: $login
};

DEFINE FIELD slug ON TABLE course TYPE string;
DEFINE FIELD title ON TABLE course TYPE string;
DEFINE FIELD data ON TABLE course FLEXIBLE TYPE option<object>;
DEFINE FIELD published ON TABLE course TYPE bool DEFAULT false;
DEFINE FIELD created_at ON TABLE course TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE course TYPE datetime VALUE time::now();
DEFINE FIELD created_by ON TABLE course TYPE string;
DEFINE FIELD updated_by ON TABLE course TYPE string;
DEFINE INDEX idx_links_slug ON TABLE course COLUMNS slug UNIQUE;

REMOVE TABLE IF EXISTS search_index;
DEFINE TABLE search_index SCHEMAFULL;

CREATE schemas CONTENT {
    slug: 'search_index',
    title: 'Search indexes',
    permission: 'schemas',
    created_by: $login,
    updated_by: $login
};

DEFINE FIELD kind ON TABLE search_index TYPE int;
DEFINE FIELD title ON TABLE search_index TYPE string;
DEFINE FIELD url ON TABLE search_index TYPE string;
DEFINE FIELD permission ON TABLE search_index TYPE string;
DEFINE INDEX idx_search_url ON TABLE search_index COLUMNS url UNIQUE;

DEFINE ANALYZER search_lowercase TOKENIZERS blank, class, punct
FILTERS lowercase, ascii, ngram(1,20), snowball(russian), snowball(english);
DEFINE INDEX idx_search_title ON search_index FIELDS title SEARCH ANALYZER search_lowercase BM25;

CREATE schemas CONTENT {
    slug: 'schemas',
    title: 'Schemas',
    permission: 'schemas',
    created_by: $login,
    updated_by: $login
};

REMOVE TABLE IF EXISTS api_keys;
DEFINE TABLE api_keys SCHEMAFULL;

CREATE schemas CONTENT {
    slug: 'api_keys',
    title: 'Users API keys',
    permission: 'schemas',
    created_by: $login,
    updated_by: $login
};

DEFINE FIELD api_key ON TABLE api_keys TYPE string;
DEFINE FIELD os ON TABLE api_keys TYPE string DEFAULT '';
DEFINE FIELD device ON TABLE api_keys TYPE string DEFAULT '';
DEFINE FIELD is_active ON TABLE api_keys TYPE bool DEFAULT false;
DEFINE INDEX idx_api_keys ON TABLE api_keys COLUMNS api_key UNIQUE;

REMOVE TABLE IF EXISTS user_api_keys;
DEFINE TABLE user_api_keys SCHEMAFULL TYPE RELATION IN users OUT api_keys;

CREATE schemas CONTENT {
    slug: 'user_api_keys',
    title: 'user_api_keys',
    permission: 'schemas',
    created_by: $login,
    updated_by: $login
};

COMMIT TRANSACTION;
BEGIN TRANSACTION;

REMOVE TABLE IF EXISTS links;
DEFINE TABLE links SCHEMAFULL;

CREATE schemas CONTENT {
    slug: 'links',
    title: 'Links',
    permission: 'schemas',
    created_by: $login,
    updated_by: $login
};

DEFINE FIELD slug ON TABLE links TYPE string;
DEFINE FIELD title ON TABLE links TYPE string;
DEFINE FIELD data ON TABLE links FLEXIBLE TYPE option<object>;
DEFINE FIELD published ON TABLE links TYPE bool DEFAULT false;
DEFINE FIELD created_at ON TABLE links TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE links TYPE datetime VALUE time::now();
DEFINE FIELD created_by ON TABLE links TYPE string;
DEFINE FIELD updated_by ON TABLE links TYPE string;
DEFINE INDEX idx_links_created ON TABLE links COLUMNS created_at;
DEFINE INDEX idx_links_slug ON TABLE links COLUMNS slug UNIQUE;

CREATE permissions CONTENT {
    id: 'links_read',
    slug: 'links::read'
};
CREATE permissions CONTENT {
    id: 'links_write',
    slug: 'links::write'
};
CREATE permissions CONTENT {
    id: 'links_delete',
    slug: 'links::delete'
};

RELATE roles:writer->role_permissions->permissions:links_read;
RELATE roles:writer->role_permissions->permissions:links_write;
RELATE roles:writer->role_permissions->permissions:links_delete;
RELATE roles:administrator->role_permissions->permissions:links_read;
RELATE roles:administrator->role_permissions->permissions:links_write;
RELATE roles:administrator->role_permissions->permissions:links_delete;

COMMIT TRANSACTION;
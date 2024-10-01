BEGIN TRANSACTION;

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
DEFINE INDEX idx_links_created ON TABLE course COLUMNS created_at;
DEFINE INDEX idx_links_slug ON TABLE course COLUMNS slug UNIQUE;

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

RELATE roles:writer->role_permissions->permissions:course_read;
RELATE roles:writer->role_permissions->permissions:course_write;
RELATE roles:writer->role_permissions->permissions:course_delete;
RELATE roles:administrator->role_permissions->permissions:course_read;
RELATE roles:administrator->role_permissions->permissions:course_write;
RELATE roles:administrator->role_permissions->permissions:course_delete;

COMMIT TRANSACTION;
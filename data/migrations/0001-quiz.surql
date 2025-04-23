BEGIN TRANSACTION;
REMOVE TABLE IF EXISTS quizzes;
DEFINE TABLE quizzes SCHEMAFULL;

DEFINE FIELD slug ON TABLE quizzes TYPE string;
DEFINE FIELD title ON TABLE quizzes TYPE string;
DEFINE FIELD description ON TABLE quizzes TYPE string;
DEFINE FIELD duration_ms ON TABLE quizzes TYPE int;
DEFINE FIELD scoring_system ON TABLE quizzes TYPE string;
DEFINE FIELD success_score ON TABLE quizzes TYPE int;
DEFINE FIELD category_ids ON TABLE quizzes TYPE array<record<categories>> DEFAULT [];
DEFINE FIELD created_at ON TABLE quizzes TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE quizzes TYPE datetime VALUE time::now();
DEFINE FIELD created_by ON TABLE quizzes TYPE string;
DEFINE FIELD updated_by ON TABLE quizzes TYPE string;

DEFINE INDEX idx_quizzes_slug ON TABLE quizzes COLUMNS slug UNIQUE;

CREATE schemas CONTENT {
    slug: 'quizzes',
    title: 'Quizzes',

    permission: 'quizzes',
    created_by: $login,
    updated_by: $login
};


REMOVE TABLE IF EXISTS categories;
DEFINE TABLE categories SCHEMAFULL;

DEFINE FIELD title ON TABLE categories TYPE string;
DEFINE FIELD success_score ON TABLE categories TYPE option<int>;
DEFINE FIELD sample_size ON TABLE categories TYPE int;
DEFINE FIELD questions ON TABLE categories FLEXIBLE TYPE array;


REMOVE TABLE IF EXISTS user_quizzes;
DEFINE TABLE user_quizzes SCHEMAFULL TYPE RELATION IN users OUT quizzes;

DEFINE FIELD quiz_settings ON TABLE user_quizzes FLEXIBLE TYPE option<object>;
DEFINE FIELD quiz_result ON TABLE user_quizzes FLEXIBLE TYPE option<object>;
DEFINE FIELD created_at ON TABLE user_quizzes TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE user_quizzes TYPE datetime VALUE time::now();
DEFINE FIELD created_by ON TABLE user_quizzes TYPE string;
DEFINE FIELD updated_by ON TABLE user_quizzes TYPE string;

DEFINE INDEX idx_user_quizzes ON TABLE user_quizzes COLUMNS in, out UNIQUE;

CREATE schemas CONTENT {
    slug: 'user_quizzes',
    title: 'User Quizzes',
    permission: 'quizzes',
    created_by: $login,
    updated_by: $login
};


CREATE permissions CONTENT {
    id: 'quizzes_read',
    slug: 'quizzes::read',
    created_by: $login
};
CREATE permissions CONTENT {
    id: 'quizzes_write',
    slug: 'quizzes::write',
    created_by: $login
};
CREATE permissions CONTENT {
    id: 'quizzes_delete',
    slug: 'quizzes::delete',
    created_by: $login
};

RELATE roles:administrator->role_permissions->permissions:quizzes_read;
RELATE roles:administrator->role_permissions->permissions:quizzes_write;
RELATE roles:administrator->role_permissions->permissions:quizzes_delete;
COMMIT TRANSACTION;
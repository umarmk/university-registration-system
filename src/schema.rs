// @generated automatically by Diesel CLI.

diesel::table! {
    audit_logs (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        #[max_length = 100]
        action -> Varchar,
        #[max_length = 100]
        entity_type -> Varchar,
        entity_id -> Nullable<Int4>,
        details -> Nullable<Jsonb>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        user_agent -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    students (id) {
        id -> Int4,
        name -> Varchar,
        phone -> Varchar,
        email -> Varchar,
        course -> Varchar,
        created_by -> Nullable<Int4>,
        updated_by -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        token -> Varchar,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 100]
        first_name -> Nullable<Varchar>,
        #[max_length = 100]
        last_name -> Nullable<Varchar>,
        role_id -> Int4,
        is_active -> Bool,
        last_login -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(audit_logs -> users (user_id));
diesel::joinable!(user_tokens -> users (user_id));
diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    audit_logs,
    roles,
    students,
    user_tokens,
    users,
);

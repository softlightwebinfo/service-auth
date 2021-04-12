table! {
    invitations (id) {
        id -> Uuid,
        email -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    login_history (id) {
        id -> Int4,
        user_id -> Int4,
        login_timestamp -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        login_session -> Varchar,
    }
}

table! {
    webs (web) {
        web -> Varchar,
        token -> Varchar,
        url -> Varchar,
        image -> Nullable<Varchar>,
    }
}

joinable!(login_history -> users (user_id));

allow_tables_to_appear_in_same_query!(
    invitations,
    login_history,
    users,
    webs,
);

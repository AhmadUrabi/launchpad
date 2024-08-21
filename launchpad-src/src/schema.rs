// @generated automatically by Diesel CLI.

diesel::table! {
    access_tokens (id) {
        id -> Unsigned<Bigint>,
        user_id -> Unsigned<Bigint>,
        #[max_length = 255]
        token -> Varchar,
        created_at -> Nullable<Timestamp>,
        last_used -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        role -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::joinable!(access_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    access_tokens,
    users,
);

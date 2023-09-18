// @generated automatically by Diesel CLI.

diesel::table! {
    address (id) {
        id -> Integer,
        #[max_length = 255]
        state -> Varchar,
        #[max_length = 255]
        city -> Varchar,
        #[max_length = 255]
        street -> Varchar,
        #[max_length = 255]
        postal_code -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Nullable<Integer>,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        #[max_length = 16]
        uuid -> Binary,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        address_id -> Nullable<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    address,
    user,
);

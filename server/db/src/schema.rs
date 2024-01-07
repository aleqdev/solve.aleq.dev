// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Uuid,
        #[max_length = 64]
        username -> Varchar,
        salt -> Bytea,
        password_hash -> Bytea,
    }
}

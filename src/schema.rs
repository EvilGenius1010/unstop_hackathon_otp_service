// @generated automatically by Diesel CLI.

diesel::table! {
    OTP (timestamp) {
        otp -> Int4,
        timestamp -> Timestamp,
        status -> Bool,
        mainUser -> Text,
    }
}

diesel::table! {
    User (phone_no) {
        phone_no -> Text,
        name -> Text,
    }
}

diesel::table! {
    _prisma_migrations (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 64]
        checksum -> Varchar,
        finished_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        migration_name -> Varchar,
        logs -> Nullable<Text>,
        rolled_back_at -> Nullable<Timestamptz>,
        started_at -> Timestamptz,
        applied_steps_count -> Int4,
    }
}

diesel::joinable!(OTP -> User (mainUser));

diesel::allow_tables_to_appear_in_same_query!(
    OTP,
    User,
    _prisma_migrations,
);

// @generated automatically by Diesel CLI.

diesel::table! {
    implementation_wrappers (implementation_wrapper_id) {
        implementation_wrapper_id -> Int4,
        implementation_id -> Int4,
        identifier -> Text,
        label -> Text,
        code -> Text,
        ordering -> Int4,
        is_formatter -> Bool,
        is_asm -> Bool,
    }
}

diesel::table! {
    implementations (implementation_id) {
        implementation_id -> Int4,
        language_id -> Int4,
        identifier -> Text,
        label -> Text,
        ordering -> Int4,
    }
}

diesel::table! {
    languages (language_id) {
        language_id -> Int4,
        priority -> Int4,
        name -> Text,
        identifier -> Text,
        hello_world -> Nullable<Text>,
    }
}

diesel::table! {
    pastes (paste_id) {
        paste_id -> Int4,
        identifier -> Text,
        delete_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        language_id -> Int4,
        paste -> Text,
        stdin -> Text,
        exit_code -> Nullable<Int4>,
        output -> Nullable<Text>,
    }
}

diesel::joinable!(implementation_wrappers -> implementations (implementation_id));
diesel::joinable!(implementations -> languages (language_id));
diesel::joinable!(pastes -> languages (language_id));

diesel::allow_tables_to_appear_in_same_query!(
    implementation_wrappers,
    implementations,
    languages,
    pastes,
);

table! {
    implementations (implementation_id) {
        implementation_id -> Int4,
        language_id -> Int4,
        identifier -> Text,
        label -> Text,
    }
}

table! {
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

table! {
    languages (language_id) {
        language_id -> Int4,
        priority -> Int4,
        name -> Text,
        highlighter_mode -> Nullable<Text>,
        mime -> Text,
        is_markdown -> Bool,
        identifier -> Text,
    }
}

table! {
    pastes (paste_id) {
        paste_id -> Int4,
        identifier -> Text,
        delete_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        language_id -> Int4,
        paste -> Text,
    }
}

table! {
    sessions (session_id) {
        session_id -> Int4,
        identifier -> Text,
        user_id -> Int4,
        start_time -> Timestamptz,
    }
}

table! {
    shared_wrappers (wrapper_id) {
        wrapper_id -> Int4,
        language_id -> Int4,
        label -> Text,
        code -> Text,
        ordering -> Int4,
        is_formatter -> Bool,
        is_asm -> Bool,
        identifier -> Text,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        nickname -> Text,
        password -> Text,
    }
}

joinable!(implementation_wrappers -> implementations (implementation_id));
joinable!(implementations -> languages (language_id));
joinable!(pastes -> languages (language_id));
joinable!(sessions -> users (user_id));
joinable!(shared_wrappers -> languages (language_id));

allow_tables_to_appear_in_same_query!(
    implementations,
    implementation_wrappers,
    languages,
    pastes,
    sessions,
    shared_wrappers,
    users,
);

table! {
    replies (id) {
        id -> Int4,
        created -> Timestamp,
        content -> Text,
        thread_id -> Int4,
    }
}

table! {
    threads (id) {
        id -> Int4,
        created -> Timestamp,
        title -> Nullable<Text>,
        content -> Text,
        bumped -> Timestamp,
        locked -> Bool,
    }
}

joinable!(replies -> threads (thread_id));

allow_tables_to_appear_in_same_query!(
    replies,
    threads,
);

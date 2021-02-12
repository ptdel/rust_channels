table! {
    threads (id) {
        id -> Int4,
        created -> Timestamp,
        title -> Nullable<Text>,
        content -> Text,
        bumped -> Timestamp,
        parent -> Nullable<Int4>,
        locked -> Bool,
    }
}

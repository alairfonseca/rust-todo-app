table! {
    boards (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    lists (id) {
        id -> Int4,
        board_id -> Int4,
        title -> Varchar,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        list_id -> Int4,
        title -> Varchar,
        description -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    boards,
    lists,
    tasks,
);

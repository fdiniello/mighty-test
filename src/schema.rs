table! {
    posts (id) {
        id -> Int8,
        time_stamp -> Timestamp,
        user_id -> Int8,
        file_path -> Varchar,
        comment -> Nullable<Text>,
        likes -> Nullable<Array<Int8>>,
    }
}

table! {
    users (id) {
        id -> Int8,
        user_name -> Varchar,
        password -> Varchar,
        display_name -> Varchar,
        can_upload -> Bool,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);

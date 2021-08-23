table! {
    likes (id) {
        id -> Int8,
    }
}

table! {
    post2_likes (id) {
        id -> Int8,
    }
}

table! {
    post3_likes (id) {
        id -> Int8,
    }
}

table! {
    post4_likes (id) {
        id -> Int8,
    }
}

table! {
    post5_likes (id) {
        id -> Int8,
    }
}

table! {
    posts (id) {
        id -> Int8,
        time_stamp -> Timestamp,
        user_id -> Int8,
        file_path -> Varchar,
        comment -> Text,
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

allow_tables_to_appear_in_same_query!(
    likes,
    post2_likes,
    post3_likes,
    post4_likes,
    post5_likes,
    posts,
    users,
);

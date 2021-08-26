table! {
    likes (post_id, user_id) {
        post_id -> Int8,
        user_id -> Int8,
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

joinable!(likes -> posts (post_id));

allow_tables_to_appear_in_same_query!(likes, posts, users,);

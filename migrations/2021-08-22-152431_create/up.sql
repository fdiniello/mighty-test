-- Your SQL goes here

-- CREATE SEQUENCE users_id_seq;
CREATE TABLE Users ( 
    id BIGSERIAL PRIMARY KEY,
    user_name VARCHAR(25) UNIQUE NOT NULL,
    password VARCHAR(50) NOT NULL,
    display_name VARCHAR(50) NOT NULL,
    can_upload BOOL DEFAULT 't' NOT NULL
); 
-- ALTER SEQUENCE users_id_seq OWNED BY Users.uid;


CREATE TABLE Posts (
    id BIGSERIAL PRIMARY KEY,
    time_stamp TIMESTAMP DEFAULT NOW() NOT NULL,
    user_id BIGSERIAL NOT NULL,
    file_path VARCHAR UNIQUE NOT NULL,
    comment TEXT,
    likes BIGINT[],
    CONSTRAINT fk_user
        FOREIGN KEY(user_id) 
            REFERENCES users(id)

);
-- CREATE SEQUENCE posts_id_seq;
-- ALTER SEQUENCE posts_id_seq OWNED BY Posts.uid;
-- Your SQL goes here

CREATE TABLE Users ( 
    id BIGSERIAL PRIMARY KEY,
    user_name VARCHAR(25) UNIQUE NOT NULL,
    password VARCHAR(50) NOT NULL,
    display_name VARCHAR(50) NOT NULL,
    can_upload BOOL DEFAULT 't' NOT NULL
); 


CREATE TABLE Posts (
    id BIGSERIAL PRIMARY KEY,
    time_stamp TIMESTAMP DEFAULT NOW() NOT NULL,
    user_id BIGSERIAL NOT NULL,
    file_path VARCHAR UNIQUE NOT NULL,
    comment TEXT NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY(user_id) 
            REFERENCES users(id)
);

CREATE TABLE Likes (
    id BIGINT UNIQUE PRIMARY KEY NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY(id) 
            REFERENCES users(id)
);
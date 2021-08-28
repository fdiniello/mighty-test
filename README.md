# Mighty-Test
This program was develop as an assesstment during the interview process with Mighty Block. You can find the full assignment in the file `./misc/Mighty Block - RUST Test .pdf`, the reference images as well can be found under the same directory.

The solution proposed in this repo is based in Diesel as SQL model manager, and Rocket for the REST API. The SQL server used for that porpuse is a PostgreSQL and you can find a `docker-compose` file to ease the usage.

## Environment set up

Once you've downloaded this repository you need to perform the following steps to get the service up un running. 

    cargo install diesel_cli --no-default-features --features postgres
    docker-compose up
    diesel migrations run

If the previous commands were executed without errors you should by now have the DB models/tables created and ready to use.
To get the service running execute:
    
    cargo run

Permanent data for DB and Photo files will be stored in the data folder as configured in the `.env` and the `docker-compose.yml` files 
    
## API and Endpoints usage

### `GET /post/get/<id>`

Get a post by its uniquie post id. The response follows the JSON format:
    JSON:{
        "id": 123,
        "time_stamp": {
            "secs_since_epoch": 1630161545,
            "nanos_since_epoch": 473292000
        },
        "user_id": 321,
        "file_path": "path/to/image.png",
        "comment": "Comment for the post",
        "likes": 157
    }

### `GET /post/get/page/<nth>/size/<size>`

Allows to navigate the post with a pagination system. Being `nth` the requested page when using a `size` number of elements per page.
The response consists in a JSON format with an Array of element of individual posts following the same format as before.

### `POST /post/new` json(newPost) 
    => POST /like/post/<post_id>/by/<user_id> 
    => GET /like/for/<post_id> 
    => GET /like/by/<user_id> 
    => POST /photo/upload/by/<user_id> 
    => GET /photo/<file..> 

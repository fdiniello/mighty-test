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

    JSON:[
        {
            "id": 123,
            ....
            "likes": 157
        },
        {
            "id": 124,
            ....
            "likes": 36
        }
    ]

When there are no more post left to send it returns an empty Array `[]`


### `POST /post/new` json(newPost) 

To submit a new post you have to POST the following JSON body to the endopoint:

    {
        "user_id": 2,
        "file_path": "Z7PFCXguwJ5SZNB.png",
        "comment": "Comment for the photo" 
    }

Where `user_id` and `comment` are the creator of said post and is the comment for the photo, respectevly. The `file_path` is the name of a recently uploaded photo that was correctely submited to the endpoint `/photo/upload/by/<user_id>`. 

### `POST /like/post/<post_id>/by/<user_id> `

To like a given post, you have to POST to this method without any body. The fields in the URL `post_id` and `user_id` refer to the given post and the user that wants to like it. If the user has already liked that given post the returned result will be an Error.

### `POST /photo/upload/by/<user_id>` raw(Photo)

To upload a new photo you have to POST it into this endpoint with the photo as body. The system checks whether the user given by its `user_id` is able to post or not. Once the file is uploaded and stored in a temporary folder the name of the file will be provided as a result, this result is for later use during post creation with endpoint `/post/new`. Files that remain in the temporary folder more than a pre-configured time will be deleted for not completing the proper upload process.

### `GET /photo/<file..>`

Once a photo has been properly uploaded it's stored in it's final path, the same that's provided in any post GET method. This method is used to retrieve the photograph from the configured storage folder with the `<file..>` path.

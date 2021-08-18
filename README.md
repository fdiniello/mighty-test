# Mighty Block RUST Developer Test

## Exercise:
You need to develop the backend API for a single page application based on the provided mockup (`Login.png`, `MightyBlock-Gram.png` and `UploadPopup.png`).
The app consists of a feature limited *"Instagram"* clone, with just the option to upload images with a description text associated.

### How it Works
- First time you enter the App it should show a signup/sign-in popup (`Login.png`).
- Once logged in it shows all the current posts made (with its image, description and upload time) arranged chronologically (from newest to oldest) (`MightyBlock-Gram.png`).
- It should support pagination of content.
- To create a new post you need to drag the image you want to upload onto the drag area, that will upload the image and show the post popup (`UploadPopup.png`).
- Once you click the *post* button, the new entry will appear in the main page.
- The app should allow posts to be *liked* and should track *likes* amount.

### Tasks
- Design and implement the backend API for supporting all the functionality mentioned.
- Design and implement the DB schema.
- The whole API should functional and covered by tests.
- Document both the API and data model.

#### Bonus Points
- Dockerized for easy dev environment setup (preferably using Docker-Compose) and deploy.
- Documentation in Markdown format.
- Frontend implementation.

### Tools/Tech to use
- Use `git` for version control.
- Backend should be done in RUST.
- Use any DB of your choice (justify your selection in your Readme file).
- **Any other tech you think it's needed**: You should explain why you chose that tech, instead of other.

### Deliveries
- You should send the whole project sources (also including `.git` folder so we can check your commits).
- Documentation (feel free to include diagrams, Swagger files or anything you think is needed).
- A *readme.txt* file explaining anything you think is important about your solution.

All this should be inside a zip file named `web_rust_test_exercise.zip`

## What we will evaluate?
- Code tidiness/source code organization
- Functional aspects of the exercise
- Design and and architecture of the solution.
- Use of the requested tech/tools

## Contact info
For any doubts you could contact us over email:
- Gianluca <gianluca@mightyblock.co>

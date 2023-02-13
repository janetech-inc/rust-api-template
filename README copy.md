# Rust API Template


A test driven micro-service template to build and deploy a rust API service with Docker and on API Gateway / AWS Lambda.

<br />

## Requirements

### - [`Rust`](https://www.rust-lang.org/tools/install)
### - ['Node']()
### - [AWS CLI]()


<br />

## Components 

  - **Rust** 
    - [`Cargo.toml`](Cargo.toml) 
    - linting and testing:
      - ```cargo clippy```
      - ```cargo fmt```
      - ```cargo test```

  - **Docker** is used to run locally and in the cloud.
    - [`Dockerfile`](Dockerfile) Dockerfile with multi-architecture support 
    - [`docker-compose`](docker-compose.yml) API Gateway and Lambda functions built from docker image


  - **API** routes are mapped to:
    - ```GET /```
    - ```GET /objects``` 
    - ```POST /objects``` 
    - ```PUT /objects/{:id}```
    - ```DELETE /objects/{:id}```
  
  - **Serverless** API Gateway and functions mapped to lambda-web on cloud (HttpServer locally).
    - [`serverless.yml`](serverless.yml)

  - **Deployment** Actions to deploy on branch commits
    - [.github/workflows](.github/workflows/development.yml) Github actions 
      -  lint   [```feature```]
      -  test   [```feature```]
      -  deploy [```main```, ```production```]
      -  tag    [```production```]

<br />


## Project Directory Structure

      ├── src
          ├── models
          ├── routes
          ├── services
      ├── tests
          ├── integration
          ├── unit
      ├── assets
          ├── images
      ├── .github
          ├── workflows

          
<br />


## Starting a new project

Start a new project by selecting this button above :point_up_2: <img src="assets/images/use-template.png" width="100">

<br />

## Development

<br />

> While working on a feature to start the service locally run: 

      cargo run 



> To run locally using docker, run 

      docker-compose up 
      docker-compose up --build

> Before you are ready to merge your feature, to test running lamnbda, you can run 

      serverless offline 

<br />

## Adding a new route

<br />

> For CRUD applications, follow the example in [resource.rs](src/routes/resource.rs) and add your resource route to: 
      
      src/routes 

> For example if you are building an API to manage Users and Shares, you would create:
      
      src/routes/users.rs
      src/routes/shares.rs

<br />

## Adding a new model

> Similar to above step, the example [resource.rs](src/models/resource.rs) add your models:
      src/models/users.rs
      src/models/shares.rs

<br />

## Connecting to a source DB

> TODO: Add DB examokes

<br />


## Test Driven Development

Before writing any functionality, we recommend you write your tests first 

> To add a new unit test
      make play

### Unit tests


### Integration tests

```bash
$ cd flask-soft-ui-dashboard-pro
$ docker-compose up --build 
```

<br />

## Continious Integration

## Permissions


## ✨ Create a new `.env` file using sample `env.sample`

The meaning of each variable can be found below: 

- `DEBUG`: if `True` the SQLite persistence is used.
  - For production use `False` = this will switch to MySql persistence
- Flask `environment variables` (used in development)
  - `FLASK_APP=run.py`
  - `FLASK_ENV=development`
- `ASSETS_ROOT`: used in assets management
  - default value: `/static/assets`
- FTP Settings: used by users to upload their profile photo. To test the connection, run `flask test_ftp`.
  - `FTP_SERVER`
  - `FTP_USER`
  - `FTP_PASSWORD`
- `MYSQL` credentials (when DEBUG=`False`)
  - `DB_ENGINE`, default value = `mysql`
  - `DB_NAME`, default value = `appseed_db`
  - `DB_HOST`, default value = `localhost`
  - `DB_PORT`, default value = `3306`
  - `DB_USERNAME`, default value = `appseed_db_usr`
  - `DB_PASS`, default value = `pass`
- `SOCIAL AUTH` Github (optional)
  - `GITHUB_ID`=YOUR_GITHUB_ID
  - `GITHUB_SECRET`=YOUR_GITHUB_SECRET
- `SOCIAL AUTH` TWITTER (optional)
  - `TWITTER_ID`=YOUR_GITHUB_ID
  - `TWITTER_SECRET`=YOUR_GITHUB_SECRET 

<br />

## ✨ Set up the MySql Database

**Note:** Make sure your Mysql server is properly installed and accessible. 

> **Step 1** - Create the MySql Database to be used by the app

- `Create a new MySql` database
- `Create a new user` and assign full privilegies (read/write)

<br />

> **Step 2** - Edit the `.env` to match your MySql DB credentials. Make sure `DB_ENGINE` is set to `mysql`.

- `DB_ENGINE`  : `mysql` 
- `DB_NAME`    : default value = `appseed_db`
- `DB_HOST`    : default value = `localhost`
- `DB_PORT`    : default value = `3306`
- `DB_USERNAME`: default value = `appseed_db_usr`
- `DB_PASS`    : default value = `pass`

<br />

Here is a sample:  

```txt
# .env sample

DEBUG=False                # False enables the MySql Persistence

DB_ENGINE=mysql            # Database Driver
DB_NAME=appseed_db         # Database Name
DB_USERNAME=appseed_db_usr # Database User
DB_PASS=STRONG_PASS_HERE   # Password 
DB_HOST=localhost          # Database HOST, default is localhost 
DB_PORT=3306               # MySql port, default = 3306 
```

<br />

## ✨ Manual Build

> - Download the [code](https://appseed.us/product/datta-able-pro/flask/) and unzip the sources (requires a `purchase`). 

```bash
$ # Get the code
$ unzip flask-datta-able-enh.zip
$ cd flask-datta-able-enh
```

<br />

### 👉 Set Up for `Unix`, `MacOS` 

> Install modules via `VENV`  

```bash
$ virtualenv env
$ source env/bin/activate
$ pip3 install -r requirements.txt
```

<br />

> Set Up Flask Environment

```bash
$ export FLASK_APP=run.py
$ export FLASK_ENV=development
```

<br />

> Set Up Database

```bash
# Init migration folder
$ flask db init # to be executed only once         
```

```bash
$ flask db migrate # Generate migration SQL
$ flask db upgrade # Apply changes
```

<br />

> Create super admin 

```bash
$ flask create_admin
```

<br />

> Start the app

```bash
$ flask run
// OR
$ flask run --cert=adhoc # For HTTPS server
```

At this point, the app runs at `http://127.0.0.1:5000/`. 

<br />

### 👉 Set Up for `Windows` 

> Install modules via `VENV` (windows) 

```
$ virtualenv env
$ .\env\Scripts\activate
$ pip3 install -r requirements.txt
```

<br />

> Set Up Flask Environment

```bash
$ # CMD 
$ set FLASK_APP=run.py
$ set FLASK_ENV=development
$
$ # Powershell
$ $env:FLASK_APP = ".\run.py"
$ $env:FLASK_ENV = "development"
```

<br />

> Start the app

```bash
$ flask run
// OR
$ flask run --cert=adhoc # For HTTPS server
```

At this point, the app runs at `http://127.0.0.1:5000/`. 

<br />

### 👉 Create (ordinary) Users

By default, the app redirects guest users to authenticate. In order to access the private pages, follow this set up: 

- Start the app via `flask run`
- Access the `registration` page and create a new user:
  - `http://127.0.0.1:5000/register`
- Access the `sign in` page and authenticate
  - `http://127.0.0.1:5000/login`

<br />

## ✨ Code-base structure

The project is coded using blueprints, app factory pattern, dual configuration profile (development and production) and an intuitive structure presented bellow:

```bash
< PROJECT ROOT >
   |
   |-- apps/
   |    |
   |    |-- home/                           # A simple app that serve HTML files
   |    |    |-- routes.py                  # Define app routes
   |    |
   |    |-- authentication/                 # Handles auth routes (login and register)
   |    |    |-- routes.py                  # Define authentication routes  
   |    |    |-- models.py                  # Defines models  
   |    |    |-- forms.py                   # Define auth forms (login and register) 
   |    |
   |    |-- static/
   |    |    |-- <css, JS, images>          # CSS files, Javascripts files
   |    |
   |    |-- templates/                      # Templates used to render pages
   |    |    |-- includes/                  # HTML chunks and components
   |    |    |    |-- navigation.html       # Top menu component
   |    |    |    |-- sidebar.html          # Sidebar component
   |    |    |    |-- footer.html           # App Footer
   |    |    |    |-- scripts.html          # Scripts common to all pages
   |    |    |
   |    |    |-- layouts/                   # Master pages
   |    |    |    |-- base-fullscreen.html  # Used by Authentication pages
   |    |    |    |-- base.html             # Used by common pages
   |    |    |
   |    |    |-- accounts/                  # Authentication pages
   |    |    |    |-- login.html            # Login page
   |    |    |    |-- register.html         # Register page
   |    |    |
   |    |    |-- home/                      # UI Kit Pages
   |    |         |-- index.html            # Index page
   |    |         |-- 404-page.html         # 404 page
   |    |         |-- *.html                # All other pages
   |    |    
   |  config.py                             # Set up the app
   |    __init__.py                         # Initialize the app
   |
   |-- requirements.txt                     # App Dependencies
   |
   |-- .env                                 # Inject Configuration via Environment
   |-- run.py                               # Start the app - WSGI gateway
   |
   |-- ************************************************************************
```

<br />

## ✨ Unitary tests

The application is tested using an isolated SQLite test database, deleted after the test suite ends.

```bash
$ python.exe run.py tests
```

| Feature | Status | 
| - | - | 
| **Registration** | - | 
| `Nominal Case`: user created with a strong password | ✔️ |  
| `Error Case1`: user creation with a weak password | ✔️ | 
| `Error Case2`: user creation using an existing username | ✔️ | 
| `Error Case3`: user creation using existing email | ✔️ | 
| - | - | 
| **Login** | - | 
| `Nominal Case`: SIGN IN ok | ✔️|  
| `Error Case 1`: existing user, wrong password | ✔️ | 
| `Error Case 2`: suspend user via three failed login attempts | ✔️ | 
| - | - |
| **Access Pages** | - | 
| `Nominal Case`: Sign in and check if the HOMEpage is successfully accessed | ✔️|  

<br />

---
[Flask Datta Able PRO](https://appseed.us/product/datta-able-pro/flask/) - Starter generated by **[AppSeed Generator](https://appseed.us/generator/)**.

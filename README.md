# Rust API Template


A test driven micro-service template to build and deploy a rust API service with Docker and on API Gateway / AWS Lambda.

## Components 

  - **Rust** 
    - [`Cargo.toml`](Cargo.toml) 
    - linting and testing:
      - ```cargo clippy```
      - ```cargo fmt```
      - ```cargo test```
      - ```pre-commit```

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


## Requirements

- [`Rust`](https://www.rust-lang.org/tools/install)
- ['Node'](https://nodejs.org/en/download/)
- [AWS CLI](https://aws.amazon.com/cli/)

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

        


## Creating a new service

Create a new service by selecting the button above :point_up_2:

<img src="assets/images/use-template.png" width="150"> 

> To setup your local environment:

      npm install
      cargo build


## Development

> While working on a feature to start the service locally run: 

      cargo run 

> To run locally using docker, run 

      docker-compose up 
      docker-compose up --build

> Before you are ready to merge your feature, to test running lamnbda, you can run 

      serverless offline 

> pre-commit 

## Adding a new route

> For CRUD applications, follow the example in [resource.rs](src/routes/resource.rs) and add your resource route to: 
      
      src/routes 

> For example if you are building an API to manage Users and Shares, you would create:
      
      src/routes/users.rs
      src/routes/shares.rs

## Adding a new model

> Similar to above step, the example [resource.rs](src/models/resource.rs) add your models:
      src/models/users.rs
      src/models/shares.rs


## Connecting to a source DB

> TODO: Add DB support


## Test Driven Development (TDD)

Before writing any functionality, we recommend you write your tests first. See the examples in:

- [unit](tests/unit.rs)
- [integration](tests/integration.rs)

To add tests, simply add to the appropriate package and using [automod]() it will be automatically detected to run.

> To run tests

      cargo test

> To run specific tests

      cargo test --package <package-name>

** When using actix-web modules, it is not recommended (or easy) to unit test a route module. Follow the example in [tests/integration/routes/resource.rs](tests/integration/routes/resource_test.rs) to add an route integration test.

## Continous Integration

Github workflows are will trigger of off specific branches

- [feature.yml](.github/workflows/feature.yml): Lint and test worflows will run any time there is a push to any feature branch.
- [development.yml](.github/workflows/development.yml): Lint, test and deployment worflows will run any time there is a push to **main** branch.
- [production.yml](.github/workflows/production.yml): Lint, test, deployment and release worflows will run any time there is a push to **main** branch.

## Permissions

Github deployment actions are setup to use [assume a role](https://github.com/marketplace/actions/aws-assume-role-github-actions).

See the [assumed rule](.github/workflows/_deploy.yml) in AWS Console for permissions.

TODO: How to update role permissions via serverless.yml


## Contributing to this template

If you have additions, changes, fixes, create a Pull Request and tag any reviewers and it will be reviewed promptly.


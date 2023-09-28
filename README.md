# Actix service bootstrap


## What is this

This repository contain a fully working example of a service written with actix. It is meant to be as close as production ready as possible while remaining minimal and easy to understand.
The service itself is a basic Create / Read / Update / Delete of a todo list, showcasing good production practices for web applications

## What you may reuse pulling this example

- A fast web service in an extensible port and adapter pattern, using dependency injection
- Multi stage docker file for building minimal images
- API documentation and manual testing with Swagger UI, Redoc, Rapi
- Async postgres client storage example
- Unit testing using fixtures
- Integration testing
- Access logging

## Run and build the project

### With docker:

Run `docker compose up -d`. On the first creation of db, it sends too soon a ready signal which may crash the web app. You may do `docker compose start app` to restart the service

### Without docker

You will need a running postgres local instance. The code is provided without Tls option enabled. Once your postgres server is running, simply `cargo run` 

### Testing

- Unit testing  `cargo test  --lib --bins`
- Integration testing `cargo test --test '*'`. You will need to run the server instance to be able to pass integration test. See notes below.

## Integrate with your tools

- [ ] [Set up project integrations](https://gitlab.com/arthur-hav/new-backend-examples/-/settings/integrations)

## Collaborate with your team

- [ ] [Invite team members and collaborators](https://docs.gitlab.com/ee/user/project/members/)
- [ ] [Create a new merge request](https://docs.gitlab.com/ee/user/project/merge_requests/creating_merge_requests.html)
- [ ] [Automatically close issues from merge requests](https://docs.gitlab.com/ee/user/project/issues/managing_issues.html#closing-issues-automatically)
- [ ] [Enable merge request approvals](https://docs.gitlab.com/ee/user/project/merge_requests/approvals/)
- [ ] [Set auto-merge](https://docs.gitlab.com/ee/user/project/merge_requests/merge_when_pipeline_succeeds.html)

## Test and Deploy

Use the built-in continuous integration in GitLab.

- [ ] [Get started with GitLab CI/CD](https://docs.gitlab.com/ee/ci/quick_start/index.html)
- [ ] [Analyze your code for known vulnerabilities with Static Application Security Testing(SAST)](https://docs.gitlab.com/ee/user/application_security/sast/)
- [ ] [Deploy to Kubernetes, Amazon EC2, or Amazon ECS using Auto Deploy](https://docs.gitlab.com/ee/topics/autodevops/requirements.html)
- [ ] [Use pull-based deployments for improved Kubernetes management](https://docs.gitlab.com/ee/user/clusters/agent/)
- [ ] [Set up protected environments](https://docs.gitlab.com/ee/ci/environments/protected_environments.html)

***

# Editing this README

When you're ready to make this README your own, just edit this file and use the handy template below (or feel free to structure it however you want - this is just a starting point!). Thank you to [makeareadme.com](https://www.makeareadme.com/) for this template.

## License
MIT
Meant to be used, derived or commercialised freely and openly anywhere.
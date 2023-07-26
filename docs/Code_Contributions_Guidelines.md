# Contributing Code

## Getting Started

We use GitHub to host code, to track issues and feature requests, as well as accept pull requests.
Before raising a pull request, ensure you have raised a corresponding issue and discussed a possible solution with a maintainer. This gives your pull request the highest chance of getting merged quickly.

## 🍴 Git Workflow

We use [Github Flow](https://guides.github.com/introduction/flow/index.html), so all code changes happen through pull requests. 

1. Fork the repo and create a new branch from the `develop` branch.
2. Branches are named as `fix/fix-name` or `feature/feature-name`
3. Please test your changes.
4. Once you are confident in your code changes, create a pull request in your fork to the `develop` branch in the `illacloud/illa` base repository.
5. If you've changed any APIs, please call this out in the pull request and ensure backward compatibility.
6. Link the issue of the base repository in your Pull request description. [Guide](https://docs.github.com/en/free-pro-team@latest/github/managing-your-work-on-github/linking-a-pull-request-to-an-issue)
7. When you raise a pull request, we automatically run tests on our CI. Please ensure that all the tests are passing for your code change. We will not be able to accept your change if the test suite doesn't pass.

## 🏡 Setup for local development

- [Running the illa cli](Setup.md)

## 📄 Read more

- [ILLA cli subcommands](Subcommands.md)
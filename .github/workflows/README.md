# GitHub Actions

There are two main types of actions.

## Run on push
Only one action is set to run on push, the build action. This action builds the docker image and pushes it to GitHub packages.

## Run on pull request
All other actions fall into this category. These run on pull request, and the PR author should make sure they all pass before merging (rebasing) their PR.

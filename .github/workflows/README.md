# GitHub Actions

There are two main types of jobs. They are all in `all_actions.yml`.

## Run on push
Only one job is set to run on push, the `build_image` job. This job builds the docker image and pushes it to GitHub packages. It will only run if the style checks and tests pass.

## Run on push and pull request
All other jobs fall into this category. These run on push and pull requests, and the PR author should make sure they all pass before merging (rebasing) their PR.

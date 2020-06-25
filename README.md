# Team Heist Tactics

Yayyy!!!!!!!!

For UI specific stuff see ui/

## Developing
First, make sure you have all this stuff:

- Yarn (https://classic.yarnpkg.com/en/docs/install/)
- Cargo (https://rustup.rs/)
- Rust nightly-2020-06-11 (try `rustup default nightly-2020-06-11`)

Building the UI, generating protobuf types, building the server, and then running it:
```
./run.sh
```
Linting:
```
rg --files | grep '\.rs' | xargs rustfmt --edition 2018
```

**Note**: If you're not using run.sh, make sure to generate the types yourself with `ui/generate_types.sh`, I don't check them in.

## Deploying
Build container:
```
docker build . -t team_heist_tactics
```
Run container:
```
docker run -p 19996:19996 -it team_heist_tactics:latest
```

## Properly deploying
Use https://github.com/banool/server-setup with something like this:
```
ansible-playbook -i hosts_external everything.yaml --extra-vars "@vars.json" --tags base,tht,https,nginx
```
This setup binds a static directory into the container from the host. When the container starts, it copies the static content in to it. Nginx on the host serves the content in there itself.

## Other
I use git lfs to manage the static content right here in the repo instead of using some other static content hosting thingo.

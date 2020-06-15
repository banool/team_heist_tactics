# Team Heist Tactics

Yayyy!!!!!!!!

For UI specific stuff see ui/

## Developing
```
./run.sh
```

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

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
Use https://github.com/banool/server-setup

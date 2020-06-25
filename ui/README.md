# UI

## Developing
For necessary dependencies, see the top level README.

Generate typescript from protobuf definitions:
```
./generate_types.sh
```

Build js:
```
yarn install
yarn run devbuild
```
Run the web UI:
```
python3 -m http.server --directory dist
# Go to the address it spits out.
```

Make things pretty with this:
```
yarn run style-fix
```

## TODO
- Use this for the websockets: https://github.com/giantmachines/redux-websocket


# UI

## Developing
For necessary dependencies, see the top level README.

Generate typescript from protobuf definitions:
```
./generate_types.sh
```

Build js:
```
npm i
npm run build
```
Run the web UI:
```
python3 -m http.server --directory dist
# Go to the address it spits out.
```

Make things pretty with this:
```
npm run style-fix
```

## TODO
- Use this for the websockets: https://github.com/giantmachines/redux-websocket


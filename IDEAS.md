## Message about how to send messages

I want to stream updates / the full state of the game back to the client as quickly as possible (because it is real time), so I should use websockets. The client also sends things back to the server like requests to move tokens, enter queues, etc. This is where things get confusing to me. In a normal web thingo, you would break up the different things a user can do into different POST endpoints, but I don't have the ability to do that if everything is going over one socket. My thinking then is instead I have json that is like {"action": <action>", "data": <data>}. Does anyone have experience with these kinds of requirements and can think of better ways than my own home cooked JSON format?

This is of course in rust, and I'm looking at actix-web. Saying that, I'd like to make the game state backend sort of agnostic from the fact that it's a web server, so I'm imagining just some kind of two way channel abstraction that the webserver could hook in to with a websocket endpoint.

The outcome was to try protobuf over websockets. Gus and Rust Foundation do _not_ like Actix, so I might try tide + my own websockets (https://github.com/sdroege/async-tungstenite).

"Again probably don't care for your case but some people cache js for a crazy long time so if you don't have the version in the url you may have old proto buf defs"

For protobuf, I think https://github.com/danburkert/prost.

Use tera for the templates.

## Thoughts on storage

I think I want to sync the state of the game to disk every 5 seconds or so. The server could have multiple active games at a time. How do I collect all their states and write to disk at once?

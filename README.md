## What Axum on Spin?

To put it simple, use Axum framework to write web application, but hook it into Spin framework to let it run in a serverless/wasm way.

## Why ?

At EightFish team, we we development our base application based on Spin, at the same time we have a changed version of sapper being used.
This repo is an output of we were trying to find a full web framework to be used on Spin.

## Getting started

```
$spin build
```

```
$ spin up
```

Some simple CRUD test:

```
curl http://127.0.0.1:3000/todos
curl -X POST http://127.0.0.1:3000/todos -H 'Content-Type: application/json' -d '{"text":"my first todo"}'
curl -X PATCH http://127.0.0.1:3000/todos/47b18889-7026-4c5e-9456-1a9d61ded798 -H 'Content-Type: application/json' -d '{"text":"my_login_again_updated", "completed":true}'
curl -X DELETE http://127.0.0.1:3000/todos/99f3cc5b-f5e1-4a24-ab80-1ff9fc8eeb72
```

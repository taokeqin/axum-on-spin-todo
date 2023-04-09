## What Axum on Spin?

To put it simple, use Axum framework to write web application, but hook it into Spin framework to let it run in a serverless/wasm way.

The axum wasm code learn from
https://github.com/tokio-rs/axum/tree/main/examples/simple-router-wasm

The todo application is a copy from axum repo.
https://github.com/tokio-rs/axum/tree/main/examples/todos

## Why ?

At EightFish team, we we development our base application based on Spin, at the same time we have a changed version of sapper being used.
This repo is an output of we were trying to find a full web framework to be used on Spin.

## Getting started

```
$ spin build
```

```
$ spin up
```

Some simple CRUD test:

```
curl http://127.0.0.1:3000/todos
curl -X POST http://127.0.0.1:3000/todos -H 'Content-Type: application/json' -d '{"text":"my first todo"}'
curl -X PATCH http://127.0.0.1:3000/todos/{id} -H 'Content-Type: application/json' -d '{"text":"my first todo, updated", "completed":true}'
curl -X DELETE http://127.0.0.1:3000/todos/{id}
```

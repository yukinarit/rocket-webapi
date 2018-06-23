Making a RESTful JSON API with Rust/Rocket
------------------------------------------

[日本語/Japanese](README_ja.md)

This repository is an attempt to implement Rust/Rocket version
of the RESTful JSON API server written in Go in this
[article](https://blog.miguelgrinberg.com/post/designing-a-restful-api-with-python-and-flask).


Setup
-----

```Bash
# Install Rust.
$ curl https://sh.rustup.rs -sSf | sh

# Install Rust nightly-2018-05-31.
# NOTE: The Rocket version (v0.3.12) that I use in this repo can not be
# compiled with the latest Rust nightly as of Jun 24, 2018.
# Use 2018-05-31 version of Rust nightly instead.
$ rustup toolchain install nightly-2018-05-31

# Clone the repository.
$ git clone https://github.com/yukinarit/rocket-jsonapi.git
```


Run
---

```Bash
$ cd rocket-jsonapi
$ cargo +nightly-2018-05-31 run


🔧  Configured for development.
    => address: localhost
    => port: 8000
    => log: normal
    => workers: 8
    => secret key: generated
    => limits: forms = 32KiB
    => tls: disabled
🛰  Mounting '/':
    => GET /
    => GET /todos
    => GET /todos/<todoid>
🚀  Rocket has launched from http://localhost:8000
```


Usage
-----

* Hello, world!
```
http://localhost:8000
```

* Get all ToDOs
```
http://localhost:8000/todos
```

* Get ToDO by ID
```
http://localhost:8000/todos/10
```


Tutorial
--------

* Hello Rocket!

Rocket is a type-safe and micro web application framework written in Rust. Rocket's design is somewhat similar to Flask, the famous micro web application framework for Python programming language.

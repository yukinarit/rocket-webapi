Making a RESTful JSON API with Rust and Rocket
==============================================

[English](README.md)

Rocketは素晴らしいWebアプリケーションフレームワークで、サンプルコードも充実していますが、WebAPIサーバーを作る包括的なチュートリアルが無かったように思えたので書いてみた。なお、このリポジトリコードはこの[記事](https://blog.miguelgrinberg.com/post/designing-a-restful-api-with-python-and-flask)を参考にしている。


Table of Contents
=================

* [Setup](#setup)
* [Run](#run)
* [Usage](#usage)
* [Tutorial](#tutorial)
  - [Rocketとは？](#rocketとは)
  - [Hello Rocket!](#hello-rocket)
  - [ToDOアプリのWebAPIをつくる](#todoアプリのwebapiをつくる)
  - [Responder](#responder)


Setup
=====

```bash
# rustupをインストールする.
$ curl https://sh.rustup.rs -sSf | /bin/bash -s -- -y --default-toolchain nightly
```

Run
===

```bash
$ cd rocket-webapi
$ cargo +nightly run


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
=====

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
========

なぜRustなのか？
----------------

Rustは型安全でゼロコスト抽象化を実現したシステムプログラミング言語である。巷ではC言語の代替と言われることもあるが、実際使ってみるとより安全なC++としての趣きが強いと思う。筆者は10年以上C++でMMOゲームや金融系のハイパフォーマンスサーバーを書いているが、C++17とかModern CMakeとか、Rangeとかテンプレートプログラミングとかを色々追ってきたが、Rustに出会ってからC++の最新を追うのをやめました。Rustを例えるなら(まだない)C++23に安全性と最高のビルドシステムと最高のパッケージマネージャが付いてきた、みたいな感じです。それほどまでにRustは素晴らしい機能と言語としての表現力を持っている。

Rocketとは？
-----------

RocketはRustで書かれたタイプセーフなマイクロウェブアプリケーションフレームワークである。Rocketのミニマムで柔軟性のあるデザインはPythonのFlaskに似ている(と思う)。それでは、シンプルなTODOアプリのWebAPIサーバーを作ってみて、Rocketの使い方を解説してみる。

Hello Rocket!
-------------

まずは、Hello, worldを返すだけのアプリを作ってみる。

* プロジェクト作成
	```bash
	$ cargo new rocket-webapi
	$ cd rocket-webapi
	```

* Cargo.toml
	```
	[package]
	name = "rocket-jsonapi"
	version = "0.1.0"
	
	[dependencies]
	rocket = "0.3.12"
	rocket_codegen = "0.3.12"
	rocket_contrib = { version = "0.3.12", features = ["json"] }
	```

**rustcのバージョンによってはコンパイルできないかもしれません。その場合はいかを実行して再ビルドしてみてください**

```bash
rustup update
```
　
* src/main.rs


```rust
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

/// GETがきたときに"Hello, world!"というレスポンスを返す
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])  // ここにルーティングをセットする
        .launch();
}
```

実行してみる。

```bash
$ cargo run
```

```bash
$ cargo run


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
🚀  Rocket has launched from http://localhost:8000
```

```bash
$ curl http://localhost:8000
Hello, world!
```

動いた！ ＼(^o^)／

ToDOアプリのWebAPIをつくる
--------------------------

次にいきおいでToDoのWebAPIを作ってみる。

* Cargo.toml
	```
	[package]
	name = "rocket-webapi"
	version = "0.1.0"
	
	[dependencies]
	rocket = "=0.3.12"
	rocket_codegen = "=0.3.12"
	rocket_contrib = { version = "=0.3.12", features = ["json"] }
	# serdeのcrateを追加する
	serde = "1.0.0"
	serde_json = "1.0.0"
	serde_derive = "1.0.0"
	```

* main.rs 
	```rust
	#![feature(plugin)]
	#![plugin(rocket_codegen)]
	
	extern crate rocket;
	#[macro_use]
	extern crate rocket_contrib;
	extern crate serde;
	#[macro_use]
	extern crate serde_derive;
	extern crate chrono;
	
	mod models;
	mod routes;
	
	# WebAPIのURLルーティングはroutes.rsに移動する
	use routes::*;
	
	fn main() {
	    rocket::ignite()
	        .mount("/", routes![index, todos, new_todo, todo_by_id])
	        .launch();
	}
	```

* routes.rs
	```rust
	// JSONを返すのに必要
	use rocket_contrib::Json;
	
	use models::ToDo;
	
	#[get("/")]
	fn index() -> &'static str {
	    "Hello, world!"
	}
	
	/// TODOリストを返す。
	/// Jsonの型がResponderをimplしているので、JSON文字列を返すことができる
	#[get("/todos")]
	fn todos() -> Json<Vec<ToDo>> {
	    Json(vec![ToDo {
	        id: 1,
	        title: "Read Rocket tutorial".into(),
	        description: "Read https://rocket.rs/guide/quickstart/".into(),
	        done: false,
	    }])
	}
	
	/// 新しいTODOを作成する
	/// POSTの時はこうする
	#[post("/todos", data = "<todo>")]
	fn new_todo(todo: Json<ToDo>) -> String {
	    format!("Accepted post request! {:?}", todo.0)
	}
	
	
	/// TODOを取得する
	#[get("/todos/<todoid>")]
	fn todo_by_id(todoid: u32) -> String {
	    let todo = ToDo {
	        id: 1,
	        title: "Read Rocket tutorial".into(),
	        description: "Read https://rocket.rs/guide/quickstart/".into(),
	        done: false,
	    };
	    format!("{:?}", todo)
	}
	```

* models.rs
	```rust
	/// TODOのモデルはmodels.rsに定義
	#[derive(Debug, Serialize, Deserialize)]
	pub struct ToDo {
	    pub id: u32,
	    pub title: String,
	    pub description: String,
	    pub done: bool,
	}
	```

実行してみる。

```bash
$ cargo run

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
    => POST /todos
    => GET /todos/<todoid>
🚀  Rocket has launched from http://localhost:8000
```

curlでリクエストしてみる。

```bash
$ curl -i http://localhost:8000/todos

HTTP/1.1 200 OK
Content-Type: application/json
Server: Rocket
Content-Length: 111
Date: Wed, 04 Jul 2018 13:44:50 GMT

[{"id":1,"title":"Read Rocket tutorial","description":"Read https://rocket.rs/guide/quickstart/","done":false}]
```

OK, いい感じ。

次、POST。

```bash
$ curl -i -H "Content-Type: application/json" -X POST -d '{"id": 100, "title":"Read this book", "description": "http://shop.oreilly.com/product/0636920040385.do", "done": false}' http://localhost:20000/todos

HTTP/1.1 200 OK
Content-Type: text/plain; charset=utf-8
Server: Rocket
Content-Length: 142
Date: Thu, 05 Jul 2018 03:55:22 GMT

Accepted post request! ToDo { id: 100, title: "Read this book", description: "http://shop.oreilly.com/product/0636920040385.do", done: false }
```

POSTも大丈夫ですね。

Responder
---------

上記の例でJson型の戻り値を返すとJSONの文字列がレスポンスとして返った。この仕組みを実現いているのがResponderトレイトだ。Rocketのルーティングの関数はすべてResponderトレイトをimplしなければならない。

難しそうい聞こえるが、実際にはRocketがいろいろな型のResponderトレイトをあらかじめimplしといてくれるので、自分でimplする場面は意外に少ないかもしれない。以下に主なResponderのimplを示す。

| 型                         | レスポンス               |
| -------------------------- | ------------------------ |
| &'static str, &str, String | text/plainの文字列が返る |
| NamedFile                  | ファイルの中身おの文字列が返る |
| Redirect                   | 別のURLにリダイレクトする |
| Stream                     | HTTPストリーミングレスポンスが返る |
| Json                       | application/jsonのJSON文字列が返る |
| Template                   | Templateをレンダリングした結果が返る |
| rocket::response::statusにある型 | 例えばAcceptedの場合あ203 Acceptedになる |
| Option<T> | Some(T)の場合はTのResponder、Noneの場合は404 Not Foundになる |
| Result<T,E> | Ok(T)の場合はT、Err(E)の場合はUのResponderの結果が返る |

最後の3つはWrapping Responderと言われておりWrapした中身のResponderの結果を装飾したり、中身の型によって動きを動的に返る役割を持つ。

以下の例を見てみよう。

* JSONを一つ返す
	```rust
	fn sample() -> Json<ToDo> {
	    Json(ToDo {
	        id: 1,
	        title: "Read Rocket tutorial".into(),
	        description: "Read https://rocket.rs/guide/quickstart/".into(),
	        done: false,
	    })
	}
	```
	```bash
	$ curl http://localhost:8000/todos

	{"id":1,"title":"Read Rocket tutorial","description":"Read https://rocket.rs/guide/quickstart/","done":false}
	```

なるほど。

* Vectorにしてみる
	```rust
	fn sample() -> Vec<Json<ToDo>> {
		Json(vec![ToDo {
		    id: 1,
		    title: "Read Rocket tutorial".into(),
		    description: "Read https://rocket.rs/guide/quickstart/".into(),
		    done: false,
		}])
	}
	```
	```bash
	$ curl http://localhost:8000/todos

	[{"id":1,"title":"Read Rocket tutorial","description":"Read https://rocket.rs/guide/quickstart/","done":false}]
	```

JSONがArrayになった。おぉいいね。

* rocket::response::status::Acceptedでwrapすると
	```rust
	use rocket::response::status::Accepted;
	fn sample() -> Accepted<Vec<Json<ToDo>>> {
		Accepted(Some(Json(vec![ToDo {
			id: 1,
			title: "Read Rocket tutorial".into(),
			description: "Read https://rocket.rs/guide/quickstart/".into(),
			done: false,
		}])))
	}
	```
	```bash
	$ curl -i http://localhost:8000/todos
	HTTP/1.1 202 Accepted
	Content-Type: application/json
	Server: Rocket
	Content-Length: 111
	Date: Fri, 06 Jul 2018 15:32:19 GMT

	[{"id":1,"title":"Read Rocket tutorial","description":"Read https://rocket.rs/guide/quickstart/","done":false}]
	```
Status codeが202 Acceptedになる。

スバラシイ。

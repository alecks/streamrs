/**
 * Copyright 2020 Alex P. (of Elitis <elitis.xyz>) <al@imalex.me>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// TODO: Find some way to pass the same futures::stream::Stream to other functions.
// TODO: Implement an API.
// TODO: Allow files of other content-types to be served.

use actix_web::{get, middleware, App, HttpResponse, HttpServer};
use bytes::Bytes;
use futures::stream::iter;
use futures::StreamExt;
use std::fs;

static API_PATH: &str = "/api/v1";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| App::new().service(home)/*.wrap_fn(|req, srv| {
        srv.call(req)
    })*/.wrap(middleware::Logger::default()))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/")]
fn home() -> HttpResponse {
    let content = fs::read("a.mp3").unwrap();
    let body = std::iter::repeat(Bytes::from(content));

    HttpResponse::Ok()
        .content_type("audio/mpeg")
        .streaming(iter(body).map(|b| Ok(b) as Result<Bytes, ()>))
}

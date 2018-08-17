/*
 * Copyright 2018 The original author or authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

extern crate function_types;
extern crate libloading as lib;
extern crate iron;

use std::env;
use std::io::Read;
use function_types::Basic;
use iron::prelude::*;
use iron::status;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Expected a single argument of the path to the function library");
        std::process::exit(-1);
    }

    let library = lib::Library::new(&args[1]).unwrap();

    let handler = move |req: &mut Request| -> IronResult<Response> {
        // Load function from shared library.
        let func: lib::Symbol<Basic>;
        unsafe {
            func = library.get(b"function").unwrap();
        }

        // Apply function to request body to produce response body.
        let mut buf: String = "".to_string();
        req.body.read_to_string(&mut buf).unwrap();
        unsafe {
            Ok(Response::with((status::Ok, func(buf))))
        }
    };
    let _server = Iron::new(handler).http("localhost:8080").unwrap();
}

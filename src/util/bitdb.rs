#![allow(non_snake_case)]

#[derive(Deserialize, Debug)]
pub struct Query {
    v: u8,
    q: Q,
}

// impl fmt::Display for Query {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Circle of radius {}", 3)
//     }
// }

#[derive(Deserialize, Debug)]
pub struct Q {
    db: Vec<String>,
    limit: u8,
}

#[derive(Deserialize, Debug)]
pub struct Find {}

use core::result::Result;
use reqwest::Error;
use serde_derive::Deserialize;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct BitDB {}

impl Display for BitDB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let query: Query = Query {
            v: 3,
            q: Q {
                db: vec![String::from("c")],
                // find: {},
                limit: 10,
            },
        };
        write!(f, "")
    }
}

impl BitDB {
    pub fn get() {
        let query: Query = Query {
            v: 3,
            q: Q {
                db: vec![String::from("c")],
                // find: {},
                limit: 10,
            },
        };
        println!("Query: {:#?}", query);
        // println!("Query: {:#?}", query.toString());
    }
}

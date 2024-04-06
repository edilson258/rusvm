use std::{
    io::{self, Write},
    process::exit,
};

use crate::{
    query::query::{Query, QueryType},
    utils::dump::dump_class_file,
    JavaClassFile,
};

pub fn prompt(class_file: &JavaClassFile, query: Query) {
    println!(
        r"
        =====================================

             Welcome to RusVM v.0.1 alpha

          [1] => Search a method
          [2] => List all methods
          [3] => dump entire class file

"
    );

    let mut user_input = String::new();
    print!("Option > ");
    io::stdout()
        .flush()
        .expect("[ERROR]: Failed to flush stdout");
    io::stdin()
        .read_line(&mut user_input)
        .expect("[ERROR]: Provide an input");

    match user_input.trim() {
        "1" => {
            let mut user_input = String::new();
            print!("Method Name > ");
            io::stdout()
                .flush()
                .expect("[ERROR]: Failed to flush stdout");
            io::stdin()
                .read_line(&mut user_input)
                .expect("[ERROR]: Provide an input");

            user_input = user_input.trim().to_string();

            let method = query.query(QueryType::QMethod(user_input.clone()));
            if method.is_none() {
                eprintln!("[ERROR]: method {user_input} not found");
                exit(1)
            }
            println!("\nMethod\n");
            println!("{:#?}\n", method.unwrap());
        }
        "2" => {
            println!("\nList of avalible methods\n");
            println!("{:#?}\n", query.query(QueryType::QMethodList).unwrap());
        }
        "3" => dump_class_file(&class_file),
        _ => {
            eprintln!("[ERROR]: Invalid Option: {user_input}");
            exit(1);
        }
    }
}

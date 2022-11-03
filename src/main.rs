
use std::{fmt, error::Error};

use clap::Parser;
use rand::{seq::SliceRandom, thread_rng};

extern crate tinytemplate;
use tinytemplate::TinyTemplate;
use serde::{Serialize};

#[derive(clap::ValueEnum, Clone, Debug,  Copy, PartialEq)]
enum Operator {
    Plus,
    Minus,
    Multiplication,
    Division
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
        Operator::Plus => write!(f, "+"),
        Operator::Minus => write!(f, "-"),
        Operator::Multiplication => write!(f, "x"),
        Operator::Division => write!(f, ":"),
       }
    }
}

impl Serialize for Operator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match *self {
            Operator::Plus => serializer.serialize_char('+'),
            Operator::Minus => serializer.serialize_char('-'),
            Operator::Multiplication => serializer.serialize_char('x'),
            Operator::Division => serializer.serialize_char(':'),
            }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 0)]
    min_first_operand: i32,

    #[arg(long, default_value_t = 0)]
    max_first_operand: i32,

    #[arg(long, default_value_t = 0)]
    min_second_operand: i32,

    #[arg(long, default_value_t = 0)]
    max_second_operand: i32,

    #[arg(long, value_enum)]
    operator: Operator,
    
    #[arg(short, long, default_value_t = 10)]
    count: usize
}

#[derive(Serialize)]
struct Operation { 
    a: i32,
    b: i32,
    ops: Operator
}

fn build_operation(a: i32, b: i32, ops: Operator) -> Option<Operation> {
    if ops == Operator::Division {
        if b == 0 {
            None
        }
        else if a % b == 0 {
            Some(Operation{a, b, ops})
        } else {
            None
        }
    } else {
        Some(Operation{a, b, ops})
    }
}


#[derive(Serialize)]
struct Context {
    operations: Vec<Operation>
}


static TEMPLATE : &'static str = r##"
<html>
<body>
{{ for operation in operations }}
<p>{operation.a} {operation.ops} {operation.b} = ...</p>
{{ endfor }}
</body>
</html>

"##;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let Args { min_first_operand, max_first_operand, min_second_operand, max_second_operand, operator, count } = args;



    let mut operations = Vec::new();
    for a in min_first_operand..max_first_operand {
        for b in min_second_operand..max_second_operand {
            if let Some(ops) = build_operation(a, b, operator) {
                operations.push(ops);
            }
        }
    }

    let mut rng = thread_rng();
    operations.shuffle(&mut rng);
    operations.truncate(count);

    // operations
    // .iter()
    // .for_each(|ops| println!("{} {} {} = ...", ops.a, ops.ops, ops.b));
    

    let mut tt = TinyTemplate::new();
    tt.add_template("output", TEMPLATE)?;

    let context = Context {
        operations: operations
    };

    let rendered = tt.render("output", &context)?;
    println!("{}", rendered);


    Ok(())
    
}

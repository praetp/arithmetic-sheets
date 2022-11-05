
use std::{fmt::{self}, error::Error, io};
use clap::Parser;
use prettytable::{Table, format, row};
use rand::{seq::SliceRandom, thread_rng};

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

#[derive(clap::ValueEnum, Clone, PartialEq)]
enum OutputFormat {
    Html,
    Table
}


#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long, default_value_t = 0)]
    min_first_operand: i32,

    #[arg(long, default_value_t = 100)]
    max_first_operand: i32,

    #[arg(long, default_value_t = 0)]
    min_second_operand: i32,

    #[arg(long, default_value_t = 100)]
    max_second_operand: i32,

    #[arg(long, default_value_t = i32::MIN)]
    min_result: i32,

    #[arg(long, default_value_t = i32::MAX)]
    max_result: i32,

    #[arg(long, value_enum)]
    operator: Operator,
    
    #[arg(short, long, default_value_t = 10)]
    count: usize,

    #[arg(long, value_enum)]
    output_format: OutputFormat
}

struct Operation { 
    a: i32,
    b: i32,
    ops: Operator
}

impl fmt::Display for Operation { 
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{} {} {} = ...", self.a, self.ops, self.b)
    }
}


fn build_operation(a: i32, b: i32, ops: Operator, min_result: i32, max_result: i32) -> Option<Operation> {
    let operation = if ops == Operator::Division {
        if b == 0 { //no divide by zero
            None
        }
        else if a % b == 0 { //only if remainder is zero
            Some(Operation{a, b, ops})
        } else {
            None
        }
    } else {
        Some(Operation{a, b, ops})
    };

    if operation.is_some() {
        let result = match ops {
            Operator::Plus => a + b,
            Operator::Minus => a -b,
            Operator::Multiplication => a * b,
            Operator::Division => a / b
        };
        if min_result <= result && result <= max_result {
            operation
        } else {
            None
        }
        
   } else {
       operation   
   }
   
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let Args { min_first_operand, max_first_operand, min_second_operand, max_second_operand, 
        min_result, max_result, operator, count, output_format } = args;

    if min_first_operand >= max_first_operand {
        Err("min_first_operand must be > max_first_operand")?
    }

    if min_second_operand >= max_second_operand {
        Err("min_second_operand must be > max_second_operand")?
    }

    if min_result >= max_result {
        Err("min_result must be > max_result")?
    }

    let mut operations = Vec::new();
    for a in min_first_operand..max_first_operand+1 {
        for b in min_second_operand..max_second_operand+1 {
            if let Some(ops) = build_operation(a, b, operator, min_result, max_result) {
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
    write(operations, output_format)?;
   

    Ok(())
    
}

fn write(operations: Vec<Operation>, output_format: OutputFormat) -> Result<(), Box<dyn Error>> {
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(&[format::LinePosition::Top,
                      format::LinePosition::Bottom],
                    format::LineSeparator::new('-', '+', '+', '+'))
        .padding(1, 4)
        .build();
    table.set_format(format);
    
    let chunks = operations.chunks_exact(3);
    for chunk in chunks {
        table.add_row(row![chunk[0], chunk[1], chunk[2]]);
    }
   

    if output_format == OutputFormat::Html {
        let mut stdout = io::stdout().lock();
        table.print_html( &mut stdout)?;

    } else if output_format == OutputFormat::Table {
        table.printstd();
    }

    Ok(())
}

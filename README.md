# arithmetic-sheets
Generator for practice sheets for elementary school arithmetic

# Example usage:
To print out 30 additions where the operands are between 10 and 100 with a maximum result of 100 in a table format
`cargo run -- --min-first-operand 10 --max-first-operand 100 --min-second-operand 10 --max-second-operand 100 --count 30  --max-result 100 --operator plus --output-format table`

To print out 30 subtractions where the first operand is between 50 and 100 and the second operand between 10 and 100 with a minimum result of 0 in a table format
`cargo run -- --min-first-operand 50 --max-first-operand 100 --min-second-operand 10 --max-second-operand 100 --count 30  --min-result 0 --operator minus --output-format table`

To print out 30 multiplications where the first operand is between 0 and 10 and the second operand between 0 and 10 with a maximum result of 100 in a table format
`cargo run -- --min-first-operand 0 --max-first-operand 10 --min-second-operand 0 --max-second-operand 10 --count 30  --max-result 100 --operator multiplication --output-format table`

To print out 30 divisions where the first operand is between 0 and 100 and the second operand between 1 and 10 in a html format
`cargo run -- --min-first-operand 0 --max-first-operand 100 --min-second-operand 1 --max-second-operand 10 --count 30 --operator division --output-format html > division.html && firefox division.html`

To print out 30 Euclidean divisions where the first operand is between 0 and 100 and the second operand between 1 and 10 with a maximum result of 10 in a table format
`cargo run -- --min-first-operand 0 --max-first-operand 100 --min-second-operand 1 --max-second-operand 10 --count 30 --max-result 10 --operator euclid --output-format table`

To generate a PDF with 50 additions with operands between 100 and 1000 and a max result of 1000
`cargo run -- --min-first-operand 100 --max-first-operand 1000 --min-second-operand 100 --max-second-operand 1000 --count 50  --max-result 1000 --operator plus --output-format table | enscript -p - | ps2pdf - output.pdf`
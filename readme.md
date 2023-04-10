csharp
Copy code
# Colops

Colops is a command-line tool for performing operations on columns of CSV and JSON files.

## Usage

To use Colops, first install it with `cargo`:

cargo install colops

arduino
Copy code

Then, run it by specifying the input file, column, and operation:

colops -f input.csv -c name -o count

markdown
Copy code

This will count the number of values in the `name` column of the `input.csv` file.

### Options

- `-f, --file <FILE>`: Sets the input file (CSV or JSON).
- `-c, --column <COLUMN>`: Specifies the column to perform the operation on. This can be either the name of the column or its index (starting from 0).
- `-o, --operation <OPERATION>`: Specifies the operation to perform on the column. Currently, only `sum` and `count` are supported.

### Examples

Count the number of values in the `name` column of a CSV file:

colops -f input.csv -c name -o count

perl
Copy code

Calculate the sum of the values in the `age` column of a CSV file:

colops -f input.csv -c age -o sum

typescript
Copy code

Count the number of values in the `users` array of a JSON file:

colops -f input.json -c users -o count

javascript
Copy code

Calculate the sum of the values in the `ages` array of a JSON file:

colops -f input.json -c ages -o sum

csharp
Copy code

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
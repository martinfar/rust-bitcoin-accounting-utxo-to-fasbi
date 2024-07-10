# Bitcoin Accounting UTXO to FASB

Bitcoin Accounting App is a Rust application designed to manage UTXOs, transactions, and generate financial accounting reports for Bitcoin transactions. The app also supports the calculation of realized gains and losses based on historical exchange rates.

## Features

- Manage UTXOs (Unspent Transaction Outputs).
- Record transactions with inputs and outputs.
- Generate FASB (Financial Accounting Standards Board) reports.
- Calculate realized gains and losses based on exchange rates.
- Support for serialization and deserialization using `serde`.


## Installation

**Build the project**:
```sh
cargo run
```

## Usage

The application can be used to manage UTXOs, add transactions, generate FASB reports, and calculate realized gains/losses.

### Adding a Transaction

Transactions consist of inputs and outputs which are represented by UTXOs. Adding a transaction updates the UTXO set and creates corresponding accounting entries.

### Generating FASB Report

FASB reports can be generated for a specified date range, listing all accounting entries within that period.

### Calculating Realized Gains/Losses

Realized gains and losses can be calculated for a specified date range, based on historical exchange rates.


## License

This project is licensed under the MIT License.
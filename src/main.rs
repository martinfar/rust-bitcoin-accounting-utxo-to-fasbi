use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UTXO {
    txid: String,
    vout: u32,
    amount: Decimal,
    address: String,
    confirmations: u64,
    spendable: bool,
    timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transaction {
    txid: String,
    timestamp: DateTime<Utc>,
    inputs: Vec<UTXO>,
    outputs: Vec<UTXO>,
    fee: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AccountingEntry {
    date: DateTime<Utc>,
    description: String,
    debit: Decimal,
    credit: Decimal,
}

struct BitcoinAccountingApp {
    utxo_set: HashMap<String, UTXO>,
    transactions: Vec<Transaction>,
    accounting_entries: Vec<AccountingEntry>,
    exchange_rates: HashMap<DateTime<Utc>, Decimal>,
}

impl BitcoinAccountingApp {
    fn new() -> Self {
        BitcoinAccountingApp {
            utxo_set: HashMap::new(),
            transactions: Vec::new(),
            accounting_entries: Vec::new(),
            exchange_rates: HashMap::new(),
        }
    }

    fn add_transaction(&mut self, transaction: Transaction) {
        // Remove spent UTXOs
        for input in &transaction.inputs {
            self.utxo_set.remove(&format!("{}:{}", input.txid, input.vout));
        }

        // Add new UTXOs
        for output in &transaction.outputs {
            let key = format!("{}:{}", transaction.txid, output.vout);
            self.utxo_set.insert(key, output.clone());
        }

        // Create accounting entries
        let total_input: Decimal = transaction.inputs.iter().map(|utxo| utxo.amount).sum();
        let total_output: Decimal = transaction.outputs.iter().map(|utxo| utxo.amount).sum();

        // Debit entry (for received funds)
        if total_output > Decimal::ZERO {
            self.accounting_entries.push(AccountingEntry {
                date: transaction.timestamp,
                description: format!("Received BTC - {}", transaction.txid),
                debit: total_output,
                credit: Decimal::ZERO,
            });
        }

        // Credit entry (for sent funds)
        if total_input > Decimal::ZERO {
            self.accounting_entries.push(AccountingEntry {
                date: transaction.timestamp,
                description: format!("Sent BTC - {}", transaction.txid),
                debit: Decimal::ZERO,
                credit: total_input,
            });
        }

        // Fee entry
        if transaction.fee > Decimal::ZERO {
            self.accounting_entries.push(AccountingEntry {
                date: transaction.timestamp,
                description: format!("Transaction fee - {}", transaction.txid),
                debit: Decimal::ZERO,
                credit: transaction.fee,
            });
        }

        self.transactions.push(transaction);
    }

    fn generate_fasb_report(&self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Vec<AccountingEntry> {
        self.accounting_entries
            .iter()
            .filter(|entry| entry.date >= start_date && entry.date <= end_date)
            .cloned()
            .collect()
    }

    fn calculate_realized_gains_losses(&self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Decimal {
        let mut realized_gain_loss = Decimal::ZERO;

        for transaction in &self.transactions {
            if transaction.timestamp < start_date || transaction.timestamp > end_date {
                continue;
            }

            let acquisition_cost: Decimal = transaction.inputs.iter().map(|utxo| {
                let acquisition_rate = self.exchange_rates.get(&utxo.timestamp).unwrap_or(&Decimal::ONE);
                utxo.amount * acquisition_rate
            }).sum();

            let sale_value: Decimal = transaction.outputs.iter().map(|utxo| {
                let sale_rate = self.exchange_rates.get(&transaction.timestamp).unwrap_or(&Decimal::ONE);
                utxo.amount * sale_rate
            }).sum();

            realized_gain_loss += sale_value - acquisition_cost;
        }

        realized_gain_loss
    }

    fn add_exchange_rate(&mut self, date: DateTime<Utc>, rate: Decimal) {
        self.exchange_rates.insert(date, rate);
    }
}

fn main() {
    let mut app = BitcoinAccountingApp::new();

    // Example usage
    let utxo1 = UTXO {
        txid: "tx1".to_string(),
        vout: 0,
        amount: dec!(1.0), // 1 BTC
        address: "addr1".to_string(),
        confirmations: 6,
        spendable: true,
        timestamp: Utc::now(),
    };

    let utxo2 = UTXO {
        txid: "tx2".to_string(),
        vout: 1,
        amount: dec!(0.5), // 0.5 BTC
        address: "addr2".to_string(),
        confirmations: 6,
        spendable: true,
        timestamp: Utc::now(),
    };

    let transaction = Transaction {
        txid: "tx3".to_string(),
        timestamp: Utc::now(),
        inputs: vec![utxo1],
        outputs: vec![utxo2.clone()],
        fee: dec!(0.0001), // 0.0001 BTC
    };

    app.add_transaction(transaction);

    // Add exchange rates
    app.add_exchange_rate(Utc::now(), dec!(50000)); // Assume 1 BTC = $50,000 USD

    // Generate FASB report
    let report = app.generate_fasb_report(Utc::now() - chrono::Duration::days(1), Utc::now());
    println!("FASB Report: {:?}", report);

    // Calculate realized gains/losses
    let gains_losses = app.calculate_realized_gains_losses(Utc::now() - chrono::Duration::days(1), Utc::now());
    println!("Realized Gains/Losses: ${}", gains_losses);
}
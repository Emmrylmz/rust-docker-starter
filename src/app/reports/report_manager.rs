use crate::app::inventory::inventory_manager::Product;
use crate::app::transactions::transaction::TransactionManager;
use crate::app::errors::errors::CustomError;
use std::io;

pub struct ReportManager;

impl ReportManager {
    pub fn display_inventory(inventory: &[Product]) -> Result<(), CustomError> {
        if inventory.is_empty() {
            return Err(CustomError::InvalidInput(
                "Inventory is empty.".to_string(),
            ));
        }

        println!("\n--- Current Inventory ---");
        for product in inventory {
            println!(
                "ID: {}, Name: {}, Quantity: {}, Price: {:.2}",
                product.id, product.name, product.quantity, product.price
            );
        }
        Ok(())
    }

    pub fn display_transactions(
        transaction_manager: &TransactionManager,
    ) -> Result<(), CustomError> {
        let transactions = transaction_manager.get_transactions();
        let total_transactions = transactions.len();
        let page_size = 20;

        if total_transactions == 0 {
            return Err(CustomError::InvalidInput(
                "No transactions to display.".to_string(),
            ));
        }

        let mut current_page = 0;

        loop {
            let start_index = current_page * page_size;
            let end_index = (start_index + page_size).min(total_transactions);

            println!(
                "\n--- Transactions Page {}/{} ---",
                current_page + 1,
                (total_transactions + page_size - 1) / page_size
            );

            println!(
                "{:<15} {:<10} {:<10}",
                "Transaction ID", "Total Items", "Total Price"
            );
            println!("{}", "-".repeat(40));

            for transaction in &transactions[start_index..end_index] {
                println!(
                    "{:<15} {:<10} {:.2}",
                    transaction.transaction_id,
                    transaction.items.len(),
                    transaction.total_price
                );
            }

            println!("\nNavigation:");
            println!("n - Next Page");
            println!("p - Previous Page");
            println!("q - Quit");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| CustomError::IOError(e.to_string()))?;

            match input.trim() {
                "n" => {
                    if end_index < total_transactions {
                        current_page += 1;
                    } else {
                        println!("Already at the last page.");
                    }
                }
                "p" => {
                    if current_page > 0 {
                        current_page -= 1;
                    } else {
                        println!("Already at the first page.");
                    }
                }
                "q" => break,
                _ => println!("Invalid input. Use 'n', 'p', or 'q'."),
            }
        }
        Ok(())
    }
}

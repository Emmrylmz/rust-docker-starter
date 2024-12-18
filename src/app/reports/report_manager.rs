use crate::app::inventory::inventory_manager::Product;
use crate::app::transactions::transaction::{TotalTransaction,  TransactionManager};

pub struct ReportManager;

impl ReportManager {
    pub fn display_inventory(inventory: &[Product]) {
        println!("\n--- Current Inventory ---");
        for product in inventory {
            println!(
                "ID: {}, Name: {}, Quantity: {}, Price: {:.2}",
                product.id, product.name, product.quantity, product.price
            );
        }
    }

    pub fn display_transactions(transactions: &[TotalTransaction]) {
        println!("\n--- Recent Transactions (Max 50) ---");
        let count = transactions.len().min(50);
        for transaction in transactions.iter().take(count) {
            println!(
                "Transaction ID: {}, Items: {:?}, Total Price: {:.2}",
                transaction.transaction_id, transaction.items, transaction.total_price
            );
        }
    }
}

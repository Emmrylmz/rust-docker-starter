pub mod errors;
pub mod inventory;
pub mod reports;
pub mod transactions;
pub mod tests;

use csv::{Reader, Writer};
use reports::report_manager;

use crate::app::inventory::inventory_manager::{InventoryManager, Product};
use crate::app::reports::report_manager::ReportManager;
use crate::app::transactions::transaction::{TotalTransaction, TransactionManager};
use std::error::Error;
use std::{io};

pub struct App {
    inventory_manager: InventoryManager,
    transaction_manager: TransactionManager,
    report_manager: ReportManager
}

impl App {
    pub fn new(inventory_file: &str) -> Result<Self, Box<dyn Error>> {
        let mut rdr = Reader::from_path("/app/src/inventory.csv")?;
        let mut rdr_ = Reader::from_path("/app/src/transactionHistory.csv")?;
        let mut products = vec![];
        let mut transaction_history: Vec<TotalTransaction> = vec![];

        for result in rdr.deserialize() {
            let product: Product = result?;
            products.push(product);
        }

        for result_ in rdr_.deserialize() {
            let t_history: TotalTransaction = result_?;
            transaction_history.push(t_history);
        }

        let mut inventory_manager = InventoryManager::new();
        inventory_manager.load_inventory(products);

        let mut transaction_manager = TransactionManager::new();
        transaction_manager.load_transaction_history(transaction_history);


        Ok(Self {
            inventory_manager,
            transaction_manager,
            report_manager: ReportManager,
        })
    }

    pub fn run(&mut self) {
        loop {
            println!("\n1. Manage Inventory");
            println!("2. Record Sale");
            println!("3. Record Purchase");
            println!("4. Generate Reports");
            println!("5. Exit");

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();

            match choice.trim() {
                "1" => self.manage_inventory(),
                "2" => self.record_sale(),
                "3" => self.record_purchase(),
                "4" => self.generate_reports(),
                "5" => break,
                _ => println!("Invalid option! Try again."),
            }
        }

        self.save_inventory("products.csv").unwrap();
    }

    fn manage_inventory(&mut self){
        self.inventory_manager.manage_inventory();
    }

    fn record_sale(&mut self) {
        match self
            .transaction_manager
            .record_sale(&mut self.inventory_manager)
        {
            Ok(_) => println!("Sale recorded successfully!"),
            Err(e) => eprintln!("Error recording sale: {}", e),
        }
    }

    fn record_purchase(&mut self) {
        match self
            .transaction_manager
            .record_purchase(&mut self.inventory_manager)
        {
            Ok(_) => println!("Purchase recorded successfully!"),
            Err(e) => eprintln!("Error recording purchase: {}", e),
        }
    }

    fn generate_reports(&self) {
        ReportManager::display_inventory(self.inventory_manager.get_inventory());
        ReportManager::display_transactions(&self.transaction_manager);
    }

    fn save_inventory(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let mut writer = Writer::from_path(file_path)?;
        for product in self.inventory_manager.get_inventory() {
            writer.serialize(product)?;
        }
        println!("Inventory saved to {}", file_path);
        Ok(())
    }
}

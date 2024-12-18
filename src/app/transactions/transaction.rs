use crate::app::inventory::inventory_manager::{self, InventoryManager, Product};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::error::Error;
use std::io::{self, Read};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TotalTransaction {
    pub transaction_id: u32,
    #[serde(deserialize_with = "deserialize_items")]
    pub items: Vec<Product>,
    pub total_price: f64,
    pub sale: bool,
}
fn deserialize_items<'de, D>(deserializer: D) -> Result<Vec<Product>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    serde_json::from_str(s).map_err(serde::de::Error::custom)
}

pub struct TransactionManager {
    transactions: Vec<TotalTransaction>,
    inventory: Vec<Product>,
}

impl TransactionManager {
    // Constructor for TransactionManager
    pub fn new() -> Self {
        Self {
            transactions: vec![],
            inventory: vec![],
        }
    }
    pub fn load_transaction_history(&mut self, t_history: Vec<TotalTransaction>) {
        self.transactions = t_history;
    }
    pub fn get_transactions(&self) -> &Vec<TotalTransaction> {
        &self.transactions
    }

    // Record a sale
    pub fn record_sale(&mut self, inventory_manager: &mut InventoryManager) {
        use std::io;

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() != 2 {
            println!("Invalid input format. Please enter product ID and quantity.");
            return;
        }

        match (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
            (Ok(product_id), Ok(quantity)) => {
                // Find the product in inventory
                if let Some(product) = inventory_manager
                    .get_mut_inventory()
                    .iter_mut()
                    .find(|p| p.id == product_id)
                {
                    if product.quantity >= quantity {
                        // Deduct quantity from inventory
                        product.quantity -= quantity;

                        // Create a Product struct for the sale
                        let sale_product = Product {
                            id: product.id,
                            name: product.name.clone(),
                            description: product.description.clone(),
                            price: product.price,
                            quantity,
                        };
                        println!("Sold Successfully : {:?}", sale_product);
                        println!(
                            "Sold Successfully : {:?}",
                            inventory_manager.get_inventory()
                        );
                    } else {
                        println!(
                            "Not enough stock available. Available quantity: {}",
                            product.quantity
                        );
                    }
                } else {
                    println!("Product not found.");
                }
            }
            _ => {
                println!("Invalid input. Please enter valid product ID and quantity.");
            }
        }
    }

    // Record a purchase
    pub fn record_purchase(&mut self, inventory_manager: &mut InventoryManager) {
        println!("Enter the purchased product in JSON format (array of products):");
    
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input) // Read a single line of input
            .expect("Failed to read input");
    
        println!("DEBUG: Received input - {}", input.trim());
    
        // Parse the JSON input
        match serde_json::from_str::<Vec<Product>>(input.trim()) {
            Ok(products) => {
                println!("DEBUG: Parsed products - {:?}", products);
    
                let transaction_id = self.transactions.len() as u32 + 1;
                let mut total_price = 0.0;
    
                for each in products.clone() {
                    println!("DEBUG: Processing product - {:?}", each);
    
                    match inventory_manager
                        .get_mut_inventory()
                        .iter_mut()
                        .find(|p| p.id == each.id)
                    {
                        Some(existing_product) => {
                            existing_product.quantity += each.quantity;
                            println!("DEBUG: Updated existing product - {:?}", existing_product);
                        }
                        None => {
                            println!("DEBUG: Adding new product - {:?}", each);
                            inventory_manager.add_product(each.clone());
                        }
                    }
    
                    total_price += (each.quantity as f64) * each.price;
                }
    
                self.transactions.push(TotalTransaction {
                    transaction_id,
                    items: products,
                    total_price,
                    sale: false,
                });
    
                println!("DEBUG: Added transaction - {:?}", self.transactions.last());
                println!("Purchase successfully recorded!");
            }
            Err(err) => {
                eprintln!("Failed to parse JSON: {}", err);
            }
        }
    }
    

    // Print all transactions
    pub fn print_transactions(&self) {
        for transaction in &self.transactions {
            println!("{:?}", transaction);
        }
    }

    // Print current inventory
    pub fn print_inventory(&self) {
        for product in &self.inventory {
            println!("{:?}", product);
        }
    }
}

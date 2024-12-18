use crate::app::errors::errors::CustomError;
use crate::app::inventory::inventory_manager::{self, InventoryManager, Product};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
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
    pub fn record_sale(
        &mut self,
        inventory_manager: &mut InventoryManager,
    ) -> Result<(), CustomError> {
        println!("Enter product ID and quantity separated by a space:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|err| CustomError::IOError(err.to_string()))?;

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() != 2 {
            return Err(CustomError::InvalidInput(
                "Expected two values: product ID and quantity".to_string(),
            ));
        }

        let product_id: u32 = parts[0]
            .parse()
            .map_err(|_| CustomError::InvalidInput("Invalid product ID".to_string()))?;
        let quantity: u32 = parts[1]
            .parse()
            .map_err(|_| CustomError::InvalidInput("Invalid quantity".to_string()))?;

        if let Some(product) = inventory_manager
            .get_mut_inventory()
            .iter_mut()
            .find(|p| p.id == product_id)
        {
            if product.quantity >= quantity {
                product.quantity -= quantity;
                let sale_product = Product {
                    id: product.id,
                    name: product.name.clone(),
                    description: product.description.clone(),
                    price: product.price,
                    quantity,
                };
                println!("Sold successfully: {:?}", sale_product);
                Ok(())
            } else {
                Err(CustomError::InsufficientStock {
                    product_id,
                    available: product.quantity,
                })
            }
        } else {
            Err(CustomError::ProductNotFound(product_id))
        }
    }

    // Record a purchase
    pub fn record_purchase(
        &mut self,
        inventory_manager: &mut InventoryManager,
    ) -> Result<(), CustomError> {
        println!("Enter the purchased product in JSON format (array of products):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|err| CustomError::IOError(err.to_string()))?;

        println!("DEBUG: Received input - {}", input.trim());

        let products: Vec<Product> = serde_json::from_str(input.trim())
            .map_err(|err| CustomError::JsonParseError(err.to_string()))?;

        println!("DEBUG: Parsed products - {:?}", products);

        let transaction_id = self.transactions.len() as u32 + 1;
        let mut total_price = 0.0;

        for each in products.clone() {
            if let Some(existing_product) = inventory_manager
                .get_mut_inventory()
                .iter_mut()
                .find(|p| p.id == each.id)
            {
                existing_product.quantity += each.quantity;
                println!("DEBUG: Updated existing product - {:?}", existing_product);
            } else {
                println!("DEBUG: Adding new product - {:?}", each);
                inventory_manager.add_product(each.clone());
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

        Ok(())
    }
}

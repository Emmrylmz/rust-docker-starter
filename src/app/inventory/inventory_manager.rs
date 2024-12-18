use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)] // Add these derives
pub struct Product {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: u32,
}

impl Product {
    pub fn new(id: u32, name: &str, description: &str, price: f64, quantity: u32) -> Self {
        Product {
            id,
            name: name.to_string(),
            description: description.to_string(),
            price,
            quantity,
        }
    }
}

pub struct InventoryManager {
    inventory: Vec<Product>,
}

impl InventoryManager {
    pub fn new() -> Self {
        Self { inventory: vec![] }
    }

    pub fn load_inventory(&mut self, products: Vec<Product>) {
        self.inventory = products;
    }

    pub fn get_inventory(&self) -> &Vec<Product> {
        &self.inventory
    }

    pub fn get_mut_inventory(&mut self) -> &mut Vec<Product> {
        &mut self.inventory
    }

    pub fn add_product(&mut self, product: Product) {
        self.inventory.push(product);
    }

    pub fn delete_product(&mut self, product_id: u32) -> Result<(), String> {
        if let Some(index) = self.inventory.iter().position(|p| p.id == product_id) {
            self.inventory.remove(index);
            Ok(())
        } else {
            Err(format!("Product with ID {} not found.", product_id))
        }
    }

    pub fn edit_product(&mut self, product_id: u32, new_product: Product) -> Result<(), String> {
        if let Some(product) = self.inventory.iter_mut().find(|p| p.id == product_id) {
            *product = new_product;
            Ok(())
        } else {
            Err(format!("Product with ID {} not found.", product_id))
        }
    }
}

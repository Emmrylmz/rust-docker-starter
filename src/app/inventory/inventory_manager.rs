use std::io;
use serde::{Deserialize, Serialize};
use crate::app::errors::errors::CustomError; 

#[derive(Debug, Deserialize, Serialize, Clone,PartialEq)]
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

    pub fn delete_product(&mut self, product_id: u32) -> Result<(), CustomError> {
        if let Some(index) = self.inventory.iter().position(|p| p.id == product_id) {
            self.inventory.remove(index);
            Ok(())
        } else {
            Err(CustomError::ProductNotFound(product_id))
        }
    }

    pub fn edit_product(&mut self, product_id: u32, new_product: Product) -> Result<(), CustomError> {
        if let Some(product) = self.inventory.iter_mut().find(|p| p.id == product_id) {
            *product = new_product;
            Ok(())
        } else {
            Err(CustomError::ProductNotFound(product_id))
        }
    }

    pub fn manage_inventory(&mut self) -> Result<(), CustomError> {
        loop {
            println!("\nManage Inventory:");
            println!("1. View Inventory");
            println!("2. Add Product");
            println!("3. Edit Product");
            println!("4. Delete Product");
            println!("5. Back to Main Menu");

            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .map_err(|e| CustomError::IOError(e.to_string()))?;

            match choice.trim() {
                "1" => self.view_inventory(),
                "2" => self.add_product_interactive()?,
                "3" => self.edit_product_interactive()?,
                "4" => self.delete_product_interactive()?,
                "5" => break,
                _ => println!("Invalid option! Please try again."),
            }
        }
        Ok(())
    }

    pub fn view_inventory(&self) {
        if self.inventory.is_empty() {
            println!("\nInventory is empty.");
        } else {
            println!("\nCurrent Inventory:");
            for product in &self.inventory {
                println!(
                    "ID: {}, Name: {}, Quantity: {}, Price: {:.2}",
                    product.id, product.name, product.quantity, product.price
                );
            }
        }
    }

    fn add_product_interactive(&mut self) -> Result<(), CustomError> {
        println!("Enter product details:");

        let id = Self::get_input("ID")?
            .parse::<u32>()
            .map_err(|_| CustomError::InvalidInput("Invalid ID".to_string()))?;
        let name = Self::get_input("Name")?;
        let description = Self::get_input("Description")?;
        let price = Self::get_input("Price")?
            .parse::<f64>()
            .map_err(|_| CustomError::InvalidInput("Invalid price".to_string()))?;
        let quantity = Self::get_input("Quantity")?
            .parse::<u32>()
            .map_err(|_| CustomError::InvalidInput("Invalid quantity".to_string()))?;

        let product = Product::new(id, &name, &description, price, quantity);
        self.add_product(product);

        println!("Product added successfully!");
        Ok(())
    }

    fn edit_product_interactive(&mut self) -> Result<(), CustomError> {
        let id = Self::get_input("Enter the ID of the product to edit")?
            .parse::<u32>()
            .map_err(|_| CustomError::InvalidInput("Invalid ID".to_string()))?;

        if let Some(product) = self.inventory.iter().find(|p| p.id == id) {
            println!("Current details of the product: {:?}", product);

            let name_input = Self::get_input("New Name (leave blank to keep current)")?;
            let name = if name_input.is_empty() {
                product.name.clone()
            } else {
                name_input
            };

            let description_input =
                Self::get_input("New Description (leave blank to keep current)")?;
            let description = if description_input.is_empty() {
                product.description.clone()
            } else {
                description_input
            };

            let price = Self::get_input("New Price (leave blank to keep current)")?
                .parse::<f64>()
                .unwrap_or(product.price);

            let quantity = Self::get_input("New Quantity (leave blank to keep current)")?
                .parse::<u32>()
                .unwrap_or(product.quantity);

            let new_product = Product::new(id, &name, &description, price, quantity);
            self.edit_product(id, new_product)?;

            println!("Product updated successfully!");
            Ok(())
        } else {
            Err(CustomError::ProductNotFound(id))
        }
    }

    fn delete_product_interactive(&mut self) -> Result<(), CustomError> {
        let id = Self::get_input("Enter the ID of the product to delete")?
            .parse::<u32>()
            .map_err(|_| CustomError::InvalidInput("Invalid ID".to_string()))?;

        self.delete_product(id)?;
        println!("Product deleted successfully!");
        Ok(())
    }

    fn get_input(prompt: &str) -> Result<String, CustomError> {
        println!("{}: ", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| CustomError::IOError(e.to_string()))?;
        Ok(input.trim().to_string())
    }
}

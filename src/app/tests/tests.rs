#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;
    use csv::Writer;
    use crate::app::inventory::inventory_manager::{InventoryManager, Product};
    use crate::app::transactions::transaction::{TransactionManager, TotalTransaction};
    use crate::app::reports::report_manager::ReportManager;
    use crate::app::errors::errors::CustomError;

    // Helper Functions
    fn setup_inventory_manager() -> InventoryManager {
        let mut manager = InventoryManager::new();
        manager.add_product(Product::new(1, "Test Product", "Description", 10.0, 5));
        manager
    }

    fn setup_transaction_manager() -> (TransactionManager, InventoryManager) {
        let mut t_manager = TransactionManager::new();
        let mut i_manager = InventoryManager::new();
        i_manager.add_product(Product::new(1, "Test Product", "Description", 10.0, 10));
        (t_manager, i_manager)
    }

    pub fn create_test_csv() -> NamedTempFile {
        let file = NamedTempFile::new().unwrap();
        let mut writer = Writer::from_writer(file.reopen().unwrap());
        
        writer.serialize(Product::new(1, "Test Product", "Description", 10.0, 5)).unwrap();
        writer.flush().unwrap();
        
        file
    }

    pub fn create_test_transaction_csv() -> NamedTempFile {
        let file = NamedTempFile::new().unwrap();
        let mut writer = Writer::from_writer(file.reopen().unwrap());
        
        let product = Product::new(1, "Test Product", "Description", 10.0, 5);
        let transaction = TotalTransaction {
            transaction_id: 1,
            items: vec![product],
            total_price: 50.0,
            sale: true,
        };
        
        writer.serialize(transaction).unwrap();
        writer.flush().unwrap();
        
        file
    }

    // InventoryManager Tests
    #[test]
    fn test_new_inventory_manager() {
        let manager = InventoryManager::new();
        assert!(manager.get_inventory().is_empty());
    }

    #[test]
    fn test_add_product() {
        let mut manager = InventoryManager::new();
        let product = Product::new(1, "Test Product", "Description", 10.0, 5);
        manager.add_product(product.clone());
        
        assert_eq!(manager.get_inventory().len(), 1);
        assert_eq!(manager.get_inventory()[0], product);
    }

    #[test]
    fn test_delete_product() {
        let mut manager = setup_inventory_manager();
        
        assert!(manager.delete_product(1).is_ok());
        assert!(manager.get_inventory().is_empty());
        
        assert!(matches!(
            manager.delete_product(1),
            Err(CustomError::ProductNotFound(1))
        ));
    }

    #[test]
    fn test_edit_product() {
        let mut manager = setup_inventory_manager();
        let new_product = Product::new(1, "Updated Product", "New Description", 15.0, 10);
        
        assert!(manager.edit_product(1, new_product.clone()).is_ok());
        assert_eq!(manager.get_inventory()[0], new_product);
        
        let invalid_product = Product::new(999, "Invalid", "Invalid", 0.0, 0);
        assert!(matches!(
            manager.edit_product(999, invalid_product),
            Err(CustomError::ProductNotFound(999))
        ));
    }

    // TransactionManager Tests
    #[test]
    fn test_new_transaction_manager() {
        let manager = TransactionManager::new();
        assert!(manager.get_transactions().is_empty());
    }

    #[test]
    fn test_load_transaction_history() {
        let mut manager = TransactionManager::new();
        let product = Product::new(1, "Test Product", "Description", 10.0, 5);
        let transaction = TotalTransaction {
            transaction_id: 1,
            items: vec![product],
            total_price: 50.0,
            sale: true,
        };
        
        manager.load_transaction_history(vec![transaction.clone()]);
        assert_eq!(manager.get_transactions().len(), 1);
        assert_eq!(manager.get_transactions()[0], transaction);
    }

    // ReportManager Tests
    #[test]
    fn test_display_inventory_empty() {
        let inventory: Vec<Product> = vec![];
        assert!(matches!(
            ReportManager::display_inventory(&inventory),
            Err(CustomError::InvalidInput(_))
        ));
    }

    #[test]
    fn test_display_inventory() {
        let inventory = vec![
            Product::new(1, "Test Product 1", "Description 1", 10.0, 5),
            Product::new(2, "Test Product 2", "Description 2", 20.0, 10),
        ];
        assert!(ReportManager::display_inventory(&inventory).is_ok());
    }

    #[test]
    fn test_display_transactions_empty() {
        let transaction_manager = TransactionManager::new();
        assert!(matches!(
            ReportManager::display_transactions(&transaction_manager),
            Err(CustomError::InvalidInput(_))
        ));
    }

    #[test]
    fn test_display_transactions() {
        let mut transaction_manager = TransactionManager::new();
        let product = Product::new(1, "Test Product", "Description", 10.0, 5);
        let transaction = TotalTransaction {
            transaction_id: 1,
            items: vec![product],
            total_price: 50.0,
            sale: true,
        };
        transaction_manager.load_transaction_history(vec![transaction]);
        // Note: Requires stdin mocking for navigation input.
    }
}

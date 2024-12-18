# Inventory Management Application 🛒

## 📋 Project Description

This project is a Rust-based inventory management application designed to manage products, transactions, and generate reports. The project is integrated with Docker and Docker Compose for seamless setup and execution.

## 🛠 Requirements

Before running the project, ensure the following tools are installed on your system:

- Docker
- Docker Compose
- Git

## 🚀 Setup and Start

### 1. Clone the Repository

```bash
git clone https://github.com/Emmrylmz/inventory-management.git

# Navigate to the project directory
cd <repository-folder>
```

### 2. Run with Docker Compose

```bash
# Start the project
docker-compose up --build

# Run in detached mode
docker-compose up -d --build
```

## 🔍 Monitoring and Testing

`cargo-watch` inside the container automatically watches source files and runs tests. Any code changes will automatically restart the application.

### Run Tests

```bash
docker-compose run app cargo test
```

### Stop the Service

```bash
docker-compose down
```

## 📂 Project Structure

```
project-directory/
│
├── src/
│   ├── app/                       # Application modules
│   │   ├── errors/                # Custom error definitions
│   │   ├── inventory/             # Inventory logic
│   │   │   ├── inventory_manager.rs
│   │   │   └── mod.rs
│   │   ├── reports/               # Reporting logic
│   │   │   ├── report_manager.rs
│   │   │   └── mod.rs
│   │   ├── tests/                 # Test suite
│   │   │   ├── tests.rs
│   │   │   └── mod.rs
│   │   ├── transactions/          # Transaction logic
│   │   │   ├── transaction.rs
│   │   │   └── mod.rs
│   │   └── mod.rs                 # Application module root
│   │
│   ├── inventory.csv              # Inventory CSV data
│   ├── transactionHistory.csv     # Transaction history CSV
│   ├── main.rs                    # Application entry point
│
├── Dockerfile                     # Docker configuration file
├── docker-compose.yml             # Docker Compose configuration
├── Cargo.toml                     # Rust project dependencies
├── Cargo.lock                     # Cargo lock file
└── README.md                      # Project documentation
```

### File Functions

- `app/errors/`: Contains custom error types used across the application.
- `app/inventory/`: Handles inventory-related logic such as adding, editing, and deleting products.
- `app/transactions/`: Manages sales and purchase transactions.
- `app/reports/`: Generates inventory and transaction reports.
- `app/tests/`: Contains integration tests and helper test modules.
- `src/main.rs`: Entry point for running the application.
- `inventory.csv`: CSV file for storing product inventory.
- `transactionHistory.csv`: CSV file for storing transaction history.

## 🛠 Tools Used

- **Rust**: For backend application logic.
- **cargo-watch**: Automatically restarts the application and runs tests on code changes.
- **Docker & Docker Compose**: For containerizing and managing the application.
- **CSV**: Data storage format for inventory and transactions.

## 🧠 Application Logic

### Inventory Management
The `InventoryManager` module provides functionalities to:
- Add, edit, and delete products.
- Load and save inventory data from/to a CSV file.

### Transaction Management
The `TransactionManager` module includes features to:
- Record sales and purchases.
- Load and view transaction history.

### Reporting
The `ReportManager` module enables:
- Displaying inventory and transaction summaries.
- Generating detailed reports for analysis.

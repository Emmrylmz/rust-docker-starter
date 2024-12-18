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
│   ├── inventory_manager.rs  # Inventory management logic
│   ├── transaction_manager.rs # Transaction management logic
│   ├── report_manager.rs      # Reporting logic
│   └── main.rs               # Application entry point
│
├── Dockerfile                # Docker configuration file
├── docker-compose.yml        # Docker Compose configuration
└── README.md                 # Project documentation
```

### File Functions

- `src/inventory_manager.rs`: Handles inventory-related logic such as adding, editing, and deleting products.
- `src/transaction_manager.rs`: Manages sales and purchase transactions.
- `src/report_manager.rs`: Generates inventory and transaction reports.
- `src/main.rs`: Entry point for running the application.
- `Dockerfile`: Configures the Docker environment for the application.
- `docker-compose.yml`: Manages multiple containers for the application.

## 🛠 Tools Used

- **Rust**: For backend application logic.
- **cargo-watch**: Automatically restarts the application and runs tests on code changes.
- **Docker & Docker Compose**: For containerizing and managing the application.

## 🧠 Application Logic

### Inventory Management
The `InventoryManager` module provides functionalities to:
- Add, edit, and delete products.
- Retrieve the current inventory.

### Transaction Management
The `TransactionManager` module includes features to:
- Record sales and purchases.
- Load and view transaction history.

### Reporting
The `ReportManager` module enables:
- Displaying inventory and transaction summaries.
- Generating detailed reports for analysis.

# 🏷️ Inventory Management System in Rust

This is a simple **Inventory Management System** developed in **Rust**, created to help beginners understand fundamental Rust concepts such as:

- Structs (`struct`)
- Enums (`enum`)
- Ownership and Borrowing (`&`, `&mut`)
- Conditional statements
- Loops
- Functions

## 📦 Features

### 🔹 Item Management
- Add new items to the inventory
- Update item quantity and price
- Sell items and decrease stock
- Display inventory status

### 🔹 Structs & Enums
- `Item` struct contains:
  - `name` (String)
  - `quantity` (i32)
  - `price` (f64)
  - `status` (ItemStatus)
- `ItemStatus` enum includes:
  - `InStock`
  - `OutOfStock`
  - `Discontinued`

### 🔹 Ownership & Borrowing
- Uses both immutable (`&str`) and mutable (`&mut`) references
- Demonstrates correct memory and resource handling in Rust

### 🔹 Extra Functionalities
- Show full inventory
- Search and filter items by name




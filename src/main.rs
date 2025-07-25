use std::io;

#[derive(Debug)]
struct Item {
    name: String,
    quantity: i32,
    price: f64,
}

impl Item {
    fn new(name: &str, quantity: i32, price: f64) -> Self {
        Item {
            name: name.to_string(),
            quantity,
            price,
        }
    }

    fn update_quantity(&mut self, quantity: i32) {
        self.quantity = quantity;
    }

    fn sell(&mut self, amount: i32) {
        if self.quantity >= amount {
            self.quantity -= amount;
            println!("Sold {} of {}.", amount, self.name);
        } else {
            println!("Not enough stock to sell {} of {}.", amount, self.name);
        }
    }

    fn display_info(&self) {
        println!("Name: {}", self.name);
        println!("Quantity: {}", self.quantity);
        println!("Price: {:.2}", self.price);
    }
}

fn main() {
    let mut items: Vec<Item> = Vec::new();

    loop {
        println!("\nSelect an option:");
        println!("0. Exit");
        println!("1. Add Item");
        println!("2. Update Quantity");
        println!("3. Sell Item");
        println!("4. Search and Display Item");
        println!("5. Display All Items");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read choice");
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice.");
                continue;
            }
        };

        match choice {
            0 => {
                println!("Goodbye!");
                break;
            }
            1 => {
                println!("Enter item name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read name");
                let name = name.trim();

                println!("Enter quantity:");
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity).expect("Failed to read quantity");
                let quantity: i32 = quantity.trim().parse().expect("Please enter a number");

                println!("Enter price:");
                let mut price = String::new();
                io::stdin().read_line(&mut price).expect("Failed to read price");
                let price: f64 = price.trim().parse().expect("Please enter a number");

                let item = Item::new(name, quantity, price);
                items.push(item);
                println!("Item added successfully.");
            }
            2 => {
                println!("Enter item name to update:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read name");
                let name = name.trim();

                if let Some(item) = items.iter_mut().find(|item| item.name == name) {
                    println!("Enter new quantity:");
                    let mut quantity = String::new();
                    io::stdin().read_line(&mut quantity).expect("Failed to read quantity");
                    let quantity: i32 = quantity.trim().parse().expect("Please enter a number");

                    item.update_quantity(quantity);
                    println!("Quantity updated.");
                } else {
                    println!("Item not found.");
                }
            }
            3 => {
                println!("Enter item name to sell:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read name");
                let name = name.trim();

                if let Some(item) = items.iter_mut().find(|item| item.name == name) {
                    println!("Enter amount to sell:");
                    let mut amount = String::new();
                    io::stdin().read_line(&mut amount).expect("Failed to read amount");
                    let amount: i32 = amount.trim().parse().expect("Please enter a number");

                    item.sell(amount);
                } else {
                    println!("Item not found.");
                }
            }
            4 => {
                println!("Enter item name to search:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read name");
                let name = name.trim();

                if let Some(item) = items.iter().find(|item| item.name == name) {
                    item.display_info();
                } else {
                    println!("Item not found.");
                }
            }
            5 => {
                for item in &items {
                    item.display_info();
                }
            }
            _ => {
                println!("Invalid option.");
            }
        }
    }
}

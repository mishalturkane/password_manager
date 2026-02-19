pub mod handlers;
pub mod input;

use sqlx::PgPool;
use input::read_input;
use handlers::*;

/// Show menu options
fn show_menu() {
    println!("\n================================");
    println!("  🔐 Rust Password Manager");
    println!("================================");
    println!("  1 → Add password");
    println!("  2 → Get password");
    println!("  3 → List all IDs");
    println!("  4 → Update ID or Password");
    println!("  5 → Delete password");
    println!("  6 → Exit");
    println!("================================");
}

/// Main CLI loop
pub async fn run(pool: &PgPool) {
    loop {
        show_menu();

        let choice = read_input("Enter choice: ");

        match choice.as_str() {
            "1" => handle_add(pool).await,
            "2" => handle_get(pool).await,
            "3" => handle_list(pool).await,
            "4" => handle_update(pool).await,
            "5" => handle_delete(pool).await,
            "6" => {
                println!("👋 Bye!");
                break;
            }
            _ => println!("❌ Invalid choice! Enter 1-6"),
        }
    }
}
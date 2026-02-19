use sqlx::PgPool;
use crate::repository::password_repo;
use crate::cli::input::read_input;

/// Handle Add password
pub async fn handle_add(pool: &PgPool) {
    let id       = read_input("Enter ID (e.g. gmail, github): ");
    let password = read_input("Enter Password: ");

    match password_repo::add(pool, &id, &password).await {
        Ok(_)  => println!("✅ Password saved for '{}'!", id),
        Err(_) => println!("❌ ID '{}' already exists!", id),
    }
}

/// Handle Get password
pub async fn handle_get(pool: &PgPool) {
    let id = read_input("Enter ID to get password: ");

    match password_repo::get_by_id(pool, &id).await {
        Ok(p)  => println!("🔑 Password for '{}' → {}", p.id, p.password),
        Err(_) => println!("❌ No password found for '{}'!", id),
    }
}

/// Handle List all IDs
pub async fn handle_list(pool: &PgPool) {
    match password_repo::get_all_ids(pool).await {
        Ok(ids) => {
            if ids.is_empty() {
                println!("📭 No passwords saved yet!");
            } else {
                println!("📋 Saved IDs:");
                for id in ids {
                    println!("  → {}", id);
                }
            }
        }
        Err(_) => println!("❌ Failed to fetch list!"),
    }
}

/// Handle Update — both ID and password can be updated
pub async fn handle_update(pool: &PgPool) {
    let old_id       = read_input("Enter current ID to update: ");
    let new_id       = read_input("Enter new ID       (press Enter to keep same): ");
    let new_password = read_input("Enter new Password (press Enter to keep same): ");

    // If empty → keep old value
    let final_id = if new_id.is_empty() {
        old_id.clone()
    } else {
        new_id
    };

    // If empty → fetch current password from DB
    let final_password = if new_password.is_empty() {
        match password_repo::get_by_id(pool, &old_id).await {
            Ok(p)  => p.password,
            Err(_) => {
                println!("❌ No password found for '{}'!", old_id);
                return;
            }
        }
    } else {
        new_password
    };

    match password_repo::update(pool, &old_id, &final_id, &final_password).await {
        Ok(true)  => println!("✅ Updated successfully!"),
        Ok(false) => println!("❌ No password found for '{}'!", old_id),
        Err(_)    => println!("❌ Failed to update!"),
    }
}

/// Handle Delete password
pub async fn handle_delete(pool: &PgPool) {
    let id = read_input("Enter ID to delete: ");

    match password_repo::delete(pool, &id).await {
        Ok(true)  => println!("🗑️  Deleted password for '{}'!", id),
        Ok(false) => println!("❌ No password found for '{}'!", id),
        Err(_)    => println!("❌ Failed to delete!"),
    }
}
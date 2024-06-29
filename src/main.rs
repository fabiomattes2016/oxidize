use Oxidize::engine::Database;

fn main() {
    let mut db = Database::new("meu_db.data");

    db.insert("Olá, mundo!");
    match db.get(0, 12) {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Failed to read data: {}", e),
    }
}
#[derive(Debug)]
struct SqliteDB {
    filename: String,
    data: Vec<String>,
}

impl SqliteDB {
    fn init(&self) {
        println!("Inicializando o {}", self.filename);
    }
    fn size(&self) {
        println!("{} elementos", self.data.len())
    }
    fn insert(&mut self, value: String) {
        self.data.push(value);
        println!("Inserção OK");

    }
}

fn main() {
    let mut slq_db: SqliteDB = SqliteDB {
        filename: String::from("db.json"),
        data: Vec::new(),
    };
    slq_db.init();
    let data_test = vec![
        String::from("Mateus"),
        String::from("José"),
        String::from("da"),
        String::from("Silva"),
    ];
    for v in data_test {
        slq_db.insert(v)
    }
    slq_db.size();
}

use std::collections::HashMap;

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().expect("param key required");
    let value = args.next().expect("param value required");
    println!("Adding key value to db - {} {}", key, value);

    let mut database = Database::new().expect("Error creating DB");
    database.insert(key.clone(), value.clone());
    database.insert(key.to_uppercase(), value.clone());
    // database.write();
}

struct Database {
    map: HashMap<String, String>,
    did_write: bool,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // read the file contents
        let contents = match std::fs::read_to_string("kv.txt") {
            Ok(c) => c,
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => String::new(),
                _ => return Err(error),
            },
        };

        let mut map = HashMap::<String, String>::new();

        for line in contents.lines() {
            let mut chunks = line.splitn(2, "\t");
            let key = chunks.next().expect("bad row, not key");
            let value = chunks.next().expect("bad row, not value");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database {
            map,
            did_write: false,
        })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
        self.did_write = false;
    }

    fn write_and_done(mut self) {
        self.write()
    }

    fn write(&mut self) {
        self.did_write = true;
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }
        std::fs::write("kv.txt", contents).expect("Cannot write to disk");
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.did_write {
            self.write();
        }
    }
}

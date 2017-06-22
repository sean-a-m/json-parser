use std::collections::HashMap;

#[derive(Debug)]
pub enum JsonType {
    String,
    Number,
    Object,
    Array,
    Boolean,
    Null,
    Error,
}

pub struct JsonTerminal {
    pub json_type: JsonType,
    pub val: String,
}

pub struct JsonObject {
    json_type: JsonType,
    val: HashMap<String, Box<Json>>,
}

pub struct JsonArray {
    json_type: JsonType,
    val: Vec<Box<Json>>,
}

pub trait Json {
    fn get_value(&self) -> &str;
    fn get_type(&self) -> &JsonType;
    fn get_path(&self, path: &str) -> Result<&Json, &'static str>;
    fn print_val(&self) -> ();
}

impl Json for JsonTerminal {
    fn get_value(&self) -> &str {
        self.val.as_str()
    }

    fn get_type(&self) -> &JsonType {
        &self.json_type
    }

    fn get_path(&self, path: &str) -> Result<&Json, &'static str> {
        println!("Getting {} on a JSON terminal", path);
        let error_string = "Something happened";
        Err(error_string)
    }

    fn print_val(&self) -> () {
        println!("{}", self.val);
    }
}

impl Json for JsonObject {
    fn get_value(&self) -> &str {
        "this is an object"
    }

    fn get_type(&self) -> &JsonType {
        &self.json_type
    }

    fn get_path(&self, path: &str) -> Result<&Json, &'static str> {
        println!("Getting key: {} on a JSON Object", path);
        match self.val.get(&path.to_string()) {
            Some(result) => Ok(&**result),
            None => Err("Something happened"),
        }

    }

    fn print_val(&self) -> () {
        for (key, val) in &self.val {
            println!("Key: {}, val:", key);
            val.print_val();
        }
    }
}

impl Json for JsonArray {
    fn get_value(&self) -> &str {
        "this is an array"
    }

    fn get_type(&self) -> &JsonType {
        &self.json_type
    }

    fn get_path(&self, path: &str) -> Result<&Json, &'static str> {
        let index = path.parse::<usize>();
        println!("Getting index: {} on a JSON array", path);
        if let Ok(i) = index {
            match self.val.get(i) {
                Some(result) => Ok(&**result),
                None => Err("Something happened"),
            }
        } else {
            Err("Something happened")
        }
    }

    fn print_val(&self) -> () {
        println!("This is an array");
    }
}


impl JsonObject {
    pub fn new() -> JsonObject {
        JsonObject {
            json_type: JsonType::Object,
            val: HashMap::<String, Box<Json>>::new(),
        }
    }

    pub fn insert(&mut self, name: &str, val: Box<Json>) -> () {
        self.val.insert(name.to_string(), val);
    }
}

impl JsonArray {
    pub fn new() -> JsonArray {
        JsonArray {
            json_type: JsonType::Array,
            val: Vec::<Box<Json>>::new(),
        }
    }

    pub fn push(&mut self, val: Box<Json>) -> () {
        self.val.push(val);
    }
}

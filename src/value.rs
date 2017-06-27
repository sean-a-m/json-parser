use std::collections::HashMap;
use std::iter;
use regex::Regex;

pub type JsonObject = HashMap<String, Box<Json>>;
pub type JsonArray = Vec<Box<Json>>;

#[derive(Copy, Clone, Debug)]
pub enum JsonType {
    String,
    Number,
    Object,
    Array,
    Boolean,
    Null,
    Error,
}

pub trait Json {
    fn get_value(&self) -> &str;
    fn get_type(&self) -> JsonType;
    fn get_path(&self, path: &str) -> Result<&Json, &'static str>;
    fn print_val(&self) -> ();
}

impl Json for String {
    fn get_value(&self) -> &str {
        &self
    }

    fn get_type(&self) -> JsonType {
        lazy_static! {
            static ref RE_STRING: Regex = Regex::new(r#"^"(\\.|[^\\"])*"#).unwrap();
            static ref RE_NUM: Regex = Regex::new(r#"^-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?"#).unwrap();
        }
        if (self == "true") | (self == "false") {
            JsonType::Boolean
        } else if self == "null" {
            JsonType::Null
        } else if RE_STRING.is_match(self) {
            JsonType::String
        } else if RE_NUM.is_match(self) {
            JsonType::Number
        } else {
            JsonType::Error
        }
    }
    
    fn get_path(&self, path: &str) -> Result<&Json, &'static str> {
        println!("Getting {} on a JSON terminal", path);
        let error_string = "Could not get path";
        Err(error_string)
    }

    fn print_val(&self) -> () {
        println!("{}", self);
    }
}

impl Json for JsonObject {
    fn get_value(&self) -> &str {
        let mut object_iter = self.iter();
        let mut object_strings = object_iter.map(|(k, v)| String::from(k.as_str()).push_str(v.get_value()));
        let mut json_string = String::new();
        json_string.push('{');
        let mut object_sep = object_strings.zip(iter::repeat(","));
        "this is an object"
    }

    fn get_type(&self) -> JsonType {
        JsonType::Object
    }
    
    fn get_path(&self, path: &str) -> Result<&Json, &'static str> {
        println!("Getting key: {} on a JSON Object", path);
        match self.get(&path.to_string()) {
            Some(result) => Ok(&**result),
            None => Err("Could not get path"),
        }
    }

    fn print_val(&self) -> () {
        for (key, val) in self {
            println!("Key: {}, val:", key);
            val.print_val();
        }
    }
}

impl Json for JsonArray {
    fn get_value(&self) -> &str {
        "this is an array"
    }

    fn get_type(&self) -> JsonType {
        JsonType::Array
    }

    fn get_path(&self, path: &str) -> Result<&Json, &'static str> {
        let index = path.parse::<usize>();
        println!("Getting index: {} on a JSON array", path);
        if let Ok(i) = index {
            match self.get(i) {
                Some(result) => Ok(&**result),
                None => Err("Could not get path"),
            }
        } else {
            Err("Could not get path")
        }
    }

    fn print_val(&self) -> () {
        println!("This is an array");
    }
}

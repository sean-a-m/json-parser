use std::collections::HashMap;

#[derive(Debug)]
pub enum Json_Type {
    string,
    number,
    object,
    array,
    boolean,
    null,
}

pub struct Json_Terminal {
    pub json_type: Json_Type,
    pub val: String,
}

pub struct Json_Object {
    json_type: Json_Type,
    val: HashMap<String, Box<json>>,
}

pub struct Json_Array {
    json_type: Json_Type,
    val: Vec<Box<json>>,
}

pub struct Json_Value {
    pub val: Box<json>,
}

pub trait json {
    fn get_value(&self) -> &str;
    fn get_type(&self) -> &Json_Type;
    fn get_path(&self, path: &str) -> Result<&json, &'static str>;
    fn print_val(&self) -> ();
}

impl json for Json_Terminal {
    fn get_value(&self) -> &str {
        self.val.as_str()
    }

    fn get_type(&self) -> &Json_Type {
        &self.json_type
    }

    fn get_path(&self, path: &str) -> Result<&json, &'static str> {
        println!("Getting {}",path);
        let error_string = "Something happened";
        Err(error_string)
    }

    fn print_val(&self) -> () {
        println!("{}", self.val);
    }
}

impl json for Json_Object {
    fn get_value(&self) -> &str {
        "this is an object"
    }
    
    fn get_type(&self) -> &Json_Type {
        &self.json_type
    }

    fn get_path(&self, path: &str) -> Result<&json, &'static str> {
         println!("Getting {}",path);
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

impl json for Json_Array {
    fn get_value(&self) -> &str {
        "this is an array"
    }
    
    fn get_type(&self) -> &Json_Type {
        &self.json_type
    }

    fn get_path(&self, path: &str) -> Result<&json, &'static str> {
        let index = path.parse::<usize>();
        println!("Getting??? {}",path);
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


impl Json_Object {
    pub fn new() -> Json_Object {
        Json_Object {
             json_type: Json_Type::object,
             val: HashMap::<String, Box<json>>::new(),
        }
    }

    pub fn insert(&mut self, name: &str, val: Box<json>) ->() {
        self.val.insert(name.to_string(), val);
    }

}

impl Json_Array {
    pub fn new() -> Json_Array {
        Json_Array {
            json_type: Json_Type::array,
            val: Vec::<Box<json>>::new(),
        }
    }

    pub fn push(&mut self, val: Box<json>) -> () {
        self.val.push(val);
    }

}


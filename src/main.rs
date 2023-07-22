 
use std::vec;
use std::str;
use clap::Parser; 

mod error;

use error::Error;

struct Input {
    file: String,
    config: Vec<String>
}

impl Input {
    fn new(file:String, config:Vec<String>) -> Self {
        Self{
            file,
            config
        }
    }
    fn isValid(&self) -> bool {
        if self.file.is_empty() {
            return false;
        }

        if self.config.len() == 0 {
            return false;
        }

        true
    }
}

fn parse_args(args: Vec<String>, i: i8) -> Result<Input, Box<dyn std::error::Error>>{
    let mut file = "";
    let mut conf = vec![];
    //if i == 0 {
        //Check that the first flag is -f
        if args[0] == "-f"{
             file = args.iter().skip(1).next().unwrap();
              if args[2] == "-c" { 
                conf = args.iter().skip(3).collect();
            }
        } else {
             // return a custom error type here
        } 
    //}
    println!("{:?}  {:?} {:?}",file, conf, args.iter().skip(3));
    let t = conf.iter().map(|f| f.to_owned().to_string()).collect();
    Ok(Input{
        file: file.to_string(),
        config: t
    })
}


fn parse_to_json(i: Input) ->Result<Json, Error>{
    // read the file
    // assume the file has been read {"a": "b"}
     
    let mut json = Json {
        lines: vec![],
        is_valid: false
    };
    let mut stack = JsonStack { data: vec![]};
    let mut line = JsonLine::new();
    let t =i.file.chars();
    let mut str_holder = String::new();
    let mut vec_holder = Vec::new(); // a temporary data structures for holding data
    let mut on_vec = false; // a variable set when we are working on an array. It helps to avoid checking the length of the array
    let mut _ptr = ""; // This is to track what we are parsing now

    let mut it = i.file.chars().peekable();
    while let Some(&c) = it.peek() { 
        // if c == ' ' || c == '\n' {
        //     continue;
        // }
        match c {
            '0' ..='9' | '.' | '-' => {
                // This should just be value -right?
                while it.peek().unwrap().is_numeric() || it.peek() == Some(&'.') {  
                    str_holder.push(it.next().unwrap());
                } 
                let k = line.clone();
                if k.key.is_none() { return Err(Error::InvalidKey(str_holder)); } //Error parsing Some parsing error
                
                // if the str_holder has . then we parse Float, otherwise we parse Int -s.parse::<f32>().unwrap();
                if str_holder.contains('.') {
                    line.set_value(JsonValue::FloatValue(str_holder.parse::<f32>().unwrap()));
                } else {
                    line.set_value(JsonValue::IntValue(str_holder.parse::<u32>().unwrap()));
                }
                str_holder = String::new();
            },
            ' ' => {
                //it.next();
            },
            '{' => {
                stack.push(it.next().unwrap().to_string());
            },
            '}' => {
                if stack.peek() == Some(&String::from("{")){
                    stack.pop();
                } else {
                    return Err(Error::OrphanCurlyBrace);
                }
            },
            '"' => {
                 // If the stack is empty it means this is the first char we are encountering
                 if stack.is_empty() {
                    return Err(Error::NoStartCurlyBrace);
                }
                stack.push("\"".to_string());
            },
            '[' => {
                on_vec = true;
                stack.push("[".to_string());
                //line.set_value(JsonValue::ArrayValue(vec![]));
            },
            ':' => {
                // need to parse what we have so far as the key
            },
            ',' => {
              // println!("Comma {:?}", json); 
            },
            f if f.is_alphanumeric() => {
                // If the stack is empty it means this is the first char we are encountering
                if stack.is_empty() {
                    return Err(Error::InvalidCharacter);
                }


                while it.peek().unwrap().is_alphanumeric()  {  
                    str_holder.push(it.next().unwrap());
                } 
                if it.peek() == Some(&'"') {
                    // we must have pushed a " into the stack. we peek to confirm if so pop, otherwise there is an invalid json
                    if stack.peek() == Some(&String::from("\"")){
                        stack.pop()
                    } else {
                        //Error Parsing THere is a problem somewhere
                        return Err(Error::IncompleteElement); 
                    } 

                    it.next();
                    // remove all the spaces
                    while it.peek() == Some(&' ') || it.peek() == Some(&'\n') {
                        it.next();
                    }
                    // if the next character is a :, then we just got a key
                    if it.peek() == Some(&':'){
                        if json.key_exists(&str_holder) {
                            return Err(Error::DuplicateKey(str_holder))
                        }
                        line.set_key(str_holder);
                        str_holder = String::new();
                        //maybe I should continue here - still undecided
                    }
                    else if it.peek() == Some(&','){
                        //prcntln!("{}", str_holder);
                        if on_vec {
                            vec_holder.push(JsonValue::StringValue(str_holder));
                        } else {
                            line.set_value(JsonValue::StringValue(str_holder));
                        }
                        str_holder = String::new();
                    } 
                    else if it.peek() == Some(&'}'){
                        // we are at the end of a json block
                        str_holder = String::new();
                        continue;
                    } 
                    else if it.peek() == Some(&']'){
                        if stack.peek() == Some(&'['.to_string()) { 
                            stack.pop();
                            vec_holder.push(JsonValue::StringValue(str_holder.clone()));
                            line.set_value(JsonValue::ArrayValue(vec_holder));
                            
                            on_vec = false; // We are through dealing with a vector
                            vec_holder = Vec::new(); // Reset the vec_holder variable since the variable has been used
                            str_holder = String::new();  // Reset the str_holder variable since the variable has been used
                        }
                        else {
                            return Err(Error::InvalidState);
                        }
                    }
                    else { 
                        //println!("{:?}", it.peek());
                        return Err(Error::InvalidCharacter); 
                    }
                    
                } else {
                    // remove all spaces
                    while it.peek() == Some(&' ') {
                        it.next();
                    }
                    if it.peek() == Some(&':'){
                       return Err(Error::IncompleteElement)
                    }
                    else if it.peek() == Some(&','){
                         
                        let mut val = false;
                        if str_holder.to_lowercase() == "true" {
                            val = true;
                        } else  if str_holder.to_lowercase() == "false"  {
                            val = false
                        } else {
                            return Err(Error::InvalidCharacter); 
                        }

                        line.set_value(JsonValue::BoolValue(val));
                        str_holder = String::new();
                    }
                    
                }
            },
            _ => {}
        }
        it.next();
       
        // Push into master
        if line.is_ok() {
            json.lines.push(line);
            line = JsonLine::new();
        }

    }

    if stack.is_empty() {
        Ok(json)
    } else {
        // we can call a function passing the stack data to know what point of orphaning
        println!("STACk {:?}", stack.data);
        return Err(Error::OrphanCurlyBrace);
    }

    
}


#[derive(Clone, Debug)]
enum JsonValue{
    Json,
    ArrayValue(Vec<JsonValue>),
    StringValue(String),
    FloatValue(f32),
    IntValue(u32),
    BoolValue(bool),
}

#[derive(Clone, Debug)]
struct JsonStack {
   data: Vec<String>
}


impl JsonStack {
    fn push(&mut self, d: String){ 
        self.data.push(d);
    }
    fn pop(&mut self) {
        self.data.pop();
    }
    fn peek(&self) -> Option<&String>{
        if !self.data.is_empty() {
            let last = self.data.len()-1;
            Some(&self.data[last])
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
#[derive(Clone, Debug)]
struct JsonLine {
    key: Option<String>,
    value: Option<JsonValue>, 
}

impl JsonLine {
    fn new () -> Self {
        Self { key: None, value: None }
    }

    fn set_key (&mut self, key: String) {
        // Can we confirm that this key does not exist in the json

        self.key = Some(key);
         
    }
    fn set_value (&mut self, value: JsonValue) {
        self.value = Some(value)
    }
    fn is_ok(&self) -> bool {
        if self.key.is_some() && self.value.is_some() {
            return true;
        }
        false
    }
}

#[derive(Debug)]
struct Json {
    lines: Vec<JsonLine>,
    is_valid: bool
}

impl  Json {
    fn key_exists(&self, key: &String) -> bool {
        self.lines.iter()
            .any(|f| f.key == Some(key.to_string()))
    }
}

fn main() { 

    // let input = r#"
    //   {
    //     "Ma" : "THis is wrong",
    //     "int": 4,
    //     "floatVal": 5.4,
    //     "8MKJK": .65,
    //     "boolean" : true,
    //     "anarray": "whatever"
    //   }
    // "#;

    let input = r#"
    {
      "sa": {"a": 2, "c": 3 },
      "Ma" : ["apple", "banana"],
      "int": 4,
      "floatVal": 5.4,
      "MKJK": .65,
      "boolean" : true,
      "anarray": "whatever"
    }
  "#;

    let i = Input {
        file : input.to_string(),
        config: vec![]
    };
    match parse_to_json(i) {
        Err(e) => println!("{}", e),
        Ok(r) => println!("{:?}", r)
    } 

    /*
  
    let args = std::env::args().skip(1);
    let input = match args.len() {
        i if i <= 3 => panic!("invalid arguments passed"),
        _ => { 
            parse_args(args.collect(), 0).unwrap()
        }
    }; 

    */
     
     

}



/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[clap(short, long, value_parser)]
   name: String,

   /// Number of times to greet
   #[clap(short, long, value_parser, default_value_t = 1)]
   count: u8,
}


// possible inputs
// cargo run -f file.json -a name, ssn
// cargo run -f file.json -c conf.json

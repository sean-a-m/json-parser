use value::{Json, JsonObject, JsonArray};
use token::{Enums, Token};

//type TokenIter<'a> = Iterator<Item = &'a Token<'a>>;

fn parse_terminal(token: &Token) -> Box<Json> {
    Box::new(token.val.to_string())
}

fn parse_array<'a, I>(tokens: &mut I) -> Box<Json>
    where I: Iterator<Item = &'a Token<'a>>
{
    let mut json_array = JsonArray::new();

    loop {
        let next_token = tokens.next().unwrap();
        if next_token.enum_type == Enums::RBRACKET {
            break;
        } else if next_token.enum_type == Enums::LBRACKET {
        } else {
            let boxed_json_val = parse_value(&next_token, tokens);
            json_array.push(boxed_json_val);
        }
    }

    Box::new(json_array)
}

fn parse_object<'a, I>(tokens: &mut I) -> Box<Json>
    where I: Iterator<Item = &'a Token<'a>>
{
    let mut json_object = JsonObject::new();
    loop {
        let next_token = tokens.next().unwrap();
        if next_token.enum_type == Enums::RBRACE {
            break;
        } else if next_token.enum_type == Enums::TERMINAL {
            let ref entry_string = next_token.val.trim_matches('"');
            json_object.insert(entry_string.to_string(),
                               parse_value(&tokens.next().unwrap(), tokens));
        }
    }
    //FIXME: Else return error here
    Box::new(json_object)
}

pub fn parse_value<'a, I>(token: &Token, tokens: &mut I) -> Box<Json>
    where I: Iterator<Item = &'a Token<'a>>
{
    //println!("{:?}", token.enum_type);
    match token.enum_type {
        Enums::TERMINAL => parse_terminal(token),
        Enums::LBRACKET => parse_array(tokens),
        Enums::LBRACE => parse_object(tokens),
        _ => {
            //println!("none!");
            parse_object(tokens) //FIXME: This should be an error
        }
    }

}

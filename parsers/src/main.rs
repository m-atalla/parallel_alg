use pest::{Parser, iterators::Pair};
use pest::error::Error;


extern crate pest;

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "json.pest"]
pub struct JSONParser;

#[derive(Debug)]
pub enum JSONValue<'a> {
    Object(Vec<(&'a str, JSONValue<'a>)>),
    Array(Vec<JSONValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null
}

impl <'a>JSONValue<'a> {
    fn to_string(ast: &'a JSONValue) -> String {
        return match ast {
            Self::Null => "null".to_string(),
            Self::Boolean(inner) => inner.to_string(),
            Self::Number(num) => num.to_string(),
            Self::String(s) => format!("{:?}", s),
            Self::Array(array) => {
                if array.is_empty() {
                    return "[]".to_string();
                }

                let mut serialized_array = String::from("[");
                for element in array {
                    serialized_array.push_str(
                        &JSONValue::to_string(element)
                    );

                    serialized_array.push(',');
                }

                // handles the extra ',' after the last element
                serialized_array.pop();

                serialized_array.push_str("]");

                serialized_array
            },
            Self::Object(obj) => {
                if obj.is_empty() {
                    return "{}".to_string();
                }

                let mut serialized_obj = String::from("{");

                for (name, element) in obj {
                    serialized_obj.push_str(format!("{:?}:", name).as_str());
                    serialized_obj.push_str(
                        &JSONValue::to_string(element)
                    );
                    serialized_obj.push(',');
                }
                serialized_obj.pop();
                serialized_obj.push_str("}");
                serialized_obj
            }
        };
    }
}

impl std::fmt::Display for JSONValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", JSONValue::to_string(self))
    }
}

fn parse_value(pair: Pair<Rule>) -> JSONValue {
    match pair.as_rule() {
        Rule::object => JSONValue::Object(
            pair.into_inner()
                .map(|pair| {
                    let mut inner_rules = pair.into_inner();

                    let name = inner_rules
                        .next()
                        .unwrap()
                        .into_inner()
                        .next()
                        .unwrap()
                        .as_str();

                    let value = parse_value(inner_rules.next().unwrap());

                    (name, value)
                })
                .collect(),
        ),
        Rule::array => JSONValue::Array(pair.into_inner().map(parse_value).collect()),
        Rule::string => JSONValue::String(pair.into_inner().next().unwrap().as_str()),
        Rule::number => JSONValue::Number(pair.as_str().parse().unwrap()),
        Rule::boolean => JSONValue::Boolean(pair.as_str().trim().parse().unwrap()),
        Rule::null => JSONValue::Null,
        Rule::json | Rule::char | Rule::EOI |
        Rule::pair | Rule::value | Rule::WHITESPACE | Rule::inner_str => unreachable!(),
    }
}

fn parse_json_file(file: &str) -> Result<JSONValue, Error<Rule>>{
    let json = JSONParser::parse(Rule::json, file)?.next().unwrap();
    Ok(parse_value(json))
}

fn main() {
    let unparsed_file = std::fs::read_to_string("./tests/test.json")
        .expect("cannot read file");

    let json = parse_json_file(&unparsed_file).unwrap();

    println!("{}", json);
}

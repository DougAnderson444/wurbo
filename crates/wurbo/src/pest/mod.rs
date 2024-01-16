//! With Wurbo, we need the Context details from the Wasm Interface Types (WIT) file in order to
//! automatically generate the helper functions.
//!
//! To parse the WIT file, we use the [pest](https://pest.rs/) parser generator.
//!
//! We want to parse the WIT into our AST Struct, so we can use the AST sturct in our minijina
//! code.
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pest/wit.pest"] // relative to src
pub struct WitParser;

/// Context Tree: We need a way to keep track and iterate over the context of the WIT file.
/// The render(ctx: context) function gets passed context as an argument. We use this code below
/// to build the context tree. This tree will be used to generate the helper functions. The fields
/// need to be iterables so that we can loop over them and generate the helper functions.
///
/// A tree may look something like this:
///
/// - Context is a Variant with one or more cases. We need the name of the cases.
/// - Each context case may also have a type of data associate with that case. If so, we need the
///
///
#[derive(Debug)]
pub enum ContextValue {
    File(Vec<ContextValue>),
    Package(String),
    /// An interface is a list of records and variants.
    Interface(Vec<ContextValue>),
    /// A variant is a list of cases.
    Variant(Vec<ContextValue>),
    /// A case is named and a type.
    Case((String, Box<ContextValue>)),
    /// A record has a name and type
    Record((Box<ContextValue>, Box<ContextValue>)),
    /// A field has a name and a type.
    Name(String),
    /// A type is a string.
    Field {
        name: Box<ContextValue>,
        ty: Box<ContextValue>,
    },
    /// A type is a string.
    Type(String),
}

pub fn parse_wit(file: &str) -> Result<ContextValue, Error<Rule>> {
    let pairs = WitParser::parse(Rule::file, file)?.next().unwrap();
    let pair_vec = pairs.into_inner().map(parse_pair).collect();
    Ok(ContextValue::File(pair_vec))
}

/// Recursively parse the WIT file into a ContextValue tree.
/// Rules used are: interface, variant, record, name, wit_type
fn parse_pair(pair: Pair<Rule>) -> ContextValue {
    match pair.as_rule() {
        Rule::interface => {
            println!("\n*** interface: {:?}", pair);
            ContextValue::Interface(pair.into_inner().map(parse_pair).collect())
        }
        Rule::variant => {
            println!("\n*** variant: {:?}", pair);
            ContextValue::Variant(pair.into_inner().map(parse_pair).collect())
        }
        Rule::record => {
            println!("\n*** record: {:?}", pair);
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let field = inner.next().unwrap();
            ContextValue::Record((
                Box::new(ContextValue::Name(name)),
                Box::new(parse_pair(field)),
            ))
        }
        Rule::name => {
            println!("\n*** name: {:?}", pair);
            ContextValue::Name(pair.as_str().to_string())
        }
        Rule::wit_type => {
            println!("\n*** wit_type: {:?}", pair);
            ContextValue::Type(pair.as_str().to_string())
        }
        Rule::package => {
            println!("\n*** package: {:?}", pair);
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            ContextValue::Package(name)
        }
        Rule::file => {
            println!("\n*** file: {:?}", pair);
            ContextValue::File(pair.into_inner().map(parse_pair).collect())
        }
        Rule::record_item => {
            println!("\n*** record_item: {:?}", pair);
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let wit_type = inner.next().unwrap().as_str().to_string();
            ContextValue::Field {
                name: Box::new(ContextValue::Name(name)),
                ty: Box::new(ContextValue::Type(wit_type)),
            }
        }
        Rule::variant_item => {
            println!("\n*** variant_item: {:?}", pair);
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let wit_type = inner.next().unwrap();
            ContextValue::Case((name, Box::new(parse_pair(wit_type))))
        }
        Rule::EOI => {
            println!("\n*** EOI: {:?}", pair);
            ContextValue::File(vec![])
        }
        _ => {
            println!("\n*** unreachable: {:?}", pair);
            unreachable!()
        }
    }
}

#[cfg(test)]
mod test_pest_parser {
    use super::*;
    use pest::Parser;
    #[test]
    fn smoke() {
        let unparsed_file =
            std::fs::read_to_string("./tests/fixtures/basic.wit").expect("Cannot read file");
        let successful_parse = WitParser::parse(Rule::file, &unparsed_file)
            .expect("to be able to parse file")
            .next()
            .expect("to be able to get first rule");
        println!("{:?}", successful_parse);
    }

    #[test]
    fn test_parse_pairs() {
        let unparsed_file =
            std::fs::read_to_string("./tests/fixtures/basic.wit").expect("Cannot read file");
        // use parse_wit
        let context = parse_wit(&unparsed_file).expect("to be able to parse file");
        println!("\n### Entire Context: \n{:?}", context);

        // should be able to do something like:
        // get the arg type of the func named "render" in the "wurbo-out" interface
        // take that arg type, and get the names of the variant cases
        // for each variant case, get the type of the associate data (if any)
        // for the assoc data, get the field names and types (recrusively for the types)
    }
}

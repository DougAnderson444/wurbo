use pest::Parser;
use wurbo::pest::Rule;
use wurbo::pest::WitParser;

#[test]
pub fn basic_test() {
    // load file from pest/fixtures/basic.wit
    let unparsed_file =
        std::fs::read_to_string("./tests/fixtures/basic.wit").expect("Cannot read file");

    // parse file
    let file = WitParser::parse(Rule::file, &unparsed_file)
        .expect("Should be able to parse file")
        .next()
        .expect("Unable to get first rule");

    for block in file.into_inner() {
        match block.as_rule() {
            Rule::package => {
                println!("\npackage: {:?}\n", block);
            }
            Rule::interface => {
                for item in block.into_inner() {
                    match item.as_rule() {
                        Rule::name => {
                            println!("interface {:?}\n", item.as_str());
                        }
                        Rule::record => {
                            // println!("iface record: {:?}\n", item.as_str());
                            for r in item.into_inner() {
                                match r.as_rule() {
                                    Rule::name => {
                                        println!("iface record name: {:?}\n", r.as_str());
                                    }
                                    Rule::record_item => {
                                        println!("iface record field: {:?}\n", r.as_str());
                                    }
                                    _ => {
                                        println!("iface record other: {:?}\n", r.as_str());
                                    }
                                }
                            }
                        }
                        Rule::variant => {
                            for v in item.into_inner() {
                                match v.as_rule() {
                                    Rule::name => {
                                        println!("name: {:?}", v.as_str());
                                    }
                                    Rule::variant_item => {
                                        for f in v.into_inner() {
                                            match f.as_rule() {
                                                Rule::name => {
                                                    print!("{}", f.as_str());
                                                }
                                                Rule::wit_type => {
                                                    println!(" type: {:?}", f.as_str());
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {
                            println!("iface other: {:?}\n", item.as_str());
                        }
                    }
                }
            }
            // Rule::world => {
            //     println!("world: {:?}", block);
            // }
            _ => {
                println!("other: {:?}\n", block);
            }
        }
    }

    assert!(true);
}

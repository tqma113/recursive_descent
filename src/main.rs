extern crate recursive_descent;

use recursive_descent::{cf_grammar, parser};
use parser::Parser;

fn main() {
    let grammar = cf_grammar!{
        Start("S");
        NonTerminals [
            "S", "A", "B", "C", "D"
        ];
        Terminals [
            "a", "b", "c"
        ];
        Rules [
            "S" => [
                ["A", "B"],
                ["D", "C"]
            ],
            "A" => [
                ["a"],
                ["a", "A"]
            ],
            "B" => [
                ["b", "c"],
                ["b", "B", "c"]
            ],
            "D" => [
                ["a", "b"],
                ["a", "D", "b"]
            ],
            "C" => [
                ["c"],
                ["c", "C"]
            ]
        ]
    };
    println!("{:?}", &grammar);
    let mut parser = Parser::new(&grammar);
    match parser.parse("aabc") {
        Ok(result) => {
            for (symbol, index) in result {
                print!("({}, {})", symbol, index)
            }
        }
        Err(unknown) => println!("Err: {}", unknown),
    }
}

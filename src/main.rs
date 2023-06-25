extern crate recursive_descent;

use parser::Parser;
use recursive_descent::{cf_grammar, parser};

fn main() {
    /*
    Detail in 6.6 of Parsing Technique.
    S -> DC | AB
    A -> a | aA
    B -> bc | bBc
    D -> ab | aDb
    C -> c | cC
     */
    let grammar = cf_grammar! {
        Start("S");
        NonTerminals [
            "S", "A", "B", "C", "D"
        ];
        Terminals [
            "a", "b", "c"
        ];
        Rules [
            "S" => [
                ["D", "C"],
                ["A", "B"]
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
        true => {
            for (symbol, index) in parser.analysis_stack {
                print!("({}, {})", symbol, index)
            }
        }
        false => {
            for diagnostic in parser.diagnostics {
                println!("Err: {}", diagnostic)
            }
        }
    }
}

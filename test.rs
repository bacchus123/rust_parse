#![feature(unboxed_closures)]
#![feature(unboxed_closure_sugar)]
#![feature(globs)]

use combinators::*;
use combinators::parser::*;
use combinators::parser::parse_stream::*;

mod combinators;

fn main(){
    let parser : Parser<&str>= box |&: mut x: Stream|{
            if x.skip("Hello"){
                (x, Ok("Hello", std::vec::Vec::new()))
            }else{
            let mut error_vec = std::vec::Vec::new();
                error_vec.push(String::from_str("Nope"));
                (x, Error(error_vec))
            }
        };
    let per_parse : Parser<&str> =  box |&: mut x: Stream| {
            if x.skip("."){
                (x, Ok(".", std::vec::Vec::new()))
            }else{
                (x, Error( vec!(String::from_str("Expected ."))))
            }
        };
    let hello_sent  = combinators::first(&parser, &per_parse);
    run(parser, Stream::new("Hello."));
    run(per_parse, Stream::new("asdf."));
    run(hello_sent, Stream::new("Hello "));
}

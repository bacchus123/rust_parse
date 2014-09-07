#![feature(unboxed_closures)]
#![feature(globs)]
use combinators::*;
use combinators::parser::*;
use combinators::parser::parse_stream::*;


mod combinators;
fn main(){
    let parser: Parser<&str> = Parser{
        parse :  box |&: mut x: Stream|{
            if x.skip("Hello"){
                (x, Ok("Hello", std::vec::Vec::new()))
            }else{
            let mut error_vec = std::vec::Vec::new();
                error_vec.push(String::from_str("Nope"));
                (x, Error(error_vec))
            }
        }
    };
    let per_parse : Parser<&str> = Parser{
        parse : box |&: mut x: Stream| {
            if x.skip("."){
                (x, parser::Ok(".", std::vec::Vec::new()))
            }else{
                (x, parser::Error( vec!()))
            }
        }
    };
    let hello_sent : Parser<&str> = combinators::first(parser, per_parse);
    hello_sent.run(Stream::new("Hello."));
    hello_sent.run(Stream::new("Hello "));
}

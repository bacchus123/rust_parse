use std::fmt::Show;

pub mod parse_stream;

pub enum ParseResult<T> {
    Ok(T, Vec<String>),
    Error(Vec<String>),
}
pub struct Parser<'a, O> {
    pub parse:  Box<Fn<(parse_stream::Stream,),(parse_stream::Stream, ParseResult<O>)> + 'a>
}
impl<'a,O:Show> Parser<'a,O> {
    pub fn run(& self, input : parse_stream::Stream){
        match self.parse.call((input,)){
            (_, Ok(x,err)) => println!("Result: {}, error list: {}", x, err),
            (_, Error(err)) => println!("Error: {}", err),
        }
    }
}




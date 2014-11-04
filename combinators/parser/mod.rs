use std::fmt::Show;

pub mod parse_stream;

pub enum ParseResult<T> {
    Ok(T, Vec<String>),
    Error(Vec<String>),
}

pub type Parser<'a, O> = Box<Fn<(parse_stream::Stream,), (parse_stream::Stream,ParseResult<O>)> + 'a>;

pub fn run<O:Show>(parser : Parser<O>,input :parse_stream::Stream){    
    match parser.call((input,)){
        (_, Ok(x,err))  => println!("Result: {}, error list: {}", x, err),
        (_, Error(err)) => println!("Error: {}", err),
    }
}

#![feature(unboxed_closures)]

pub mod parser;

pub fn first<'b,'a,'c,O,V> (p1 : &'b parser::Parser<'a,O>, p2 :& 'b parser::Parser<'a,V>)->  parser::Parser<'a, O>{
    box |&: x : parser::parse_stream::Stream|->(parser::parse_stream::Stream, parser::ParseResult<O>){
        match p1.call((x,)){
            (i, parser::Error(err)) => (i, parser::Error(err)),
            (i, parser::Ok(res, mut err)) => {
                match p2.call((i,)){
                    (stream, parser::Error(p2err)) => {
                        err.push_all(p2err.as_slice());
                        (stream, parser::Error(err))
                    },
                    (stream, parser::Ok(_, p2err)) => {
                        err.push_all(p2err.as_slice());
                        (stream,parser::Ok(res, err))
                    }
                }
            }
        }
    }
}


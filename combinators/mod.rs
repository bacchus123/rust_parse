pub mod parser;

pub fn first<'a,'b,  O, V> (p1 : &'b parser::Parser<'a,O>,p2 :& 'b  parser::Parser<'a,V>)-> Fn<(parser::parse_stream::Stream,),(parser::parse_stream::Stream,parser::ParseResult<O>)> + 'a 
{            
    |&: x : parser::parse_stream::Stream|->(parser::parse_stream::Stream, parser::ParseResult<O>)
    {
        match p1.call((x,)){
            (i, parser::Error(err)) => (i, parser::Error(err)),
            (i, parser::Ok(res, err)) => {
                match p2.call((i,)){
                    (stream , parser::Error(p2err)) => (stream, parser::Error(err.append(p2err.as_slice()))),
                    (stream, parser::Ok(_, p2err)) => (stream, parser::Ok(res, err.append(p2err.as_slice())))
                }
            }
        }
    } as  Fn<(parser::parse_stream::Stream,),(parser::parse_stream::Stream,parser::ParseResult<O>)>
}


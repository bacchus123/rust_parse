#![feature(unboxed_closures)]
pub mod parser;

pub fn first<'a, O, V> (p1 : parser::Parser<'a,O>, p2 : parser::Parser<'a,V>)->parser::Parser<'a,O>{
        parser::Parser{
            parse : box |&: mut x:parser::parse_stream::Stream|{
                let first = p1.parse.call((x,));
                match first{
                    (x, parser::Ok(r, opt_errs))=>{
                        let second = p2.parse.call((x,));
                        match second
                        {
                            (x, parser::Error(p2_errs))=> {
                                (x, parser::Error(opt_errs.append(p2_errs.as_slice())))
                            }
                            (x, parser::Ok(_, p2_opt_errs)) =>{
                                (x, parser::Ok(r, opt_errs.append(p2_opt_errs.as_slice())))
                            }
                        }
                    }
                    _ => first
                }
            }
        }
        
    }

use std::str;

pub trait ParseStream{
    fn tag(&self)-> u64;
    fn next_tag(&mut self);
    fn newline(&mut self);   
}

impl ParseStream for Stream{
    fn tag(&self)->u64{
        self.state_tag
    }
    fn next_tag(&mut self){
        self.state_tag = self.state_tag + 1;
    }
    fn newline(&mut self){
        self.line = self.line + 1;
        self.line_begin = self.index;
        self.next_tag();
    }
}

pub struct Stream{
    stream : String,
    index : u64,
    line_begin : u64,
    line : u64,
    state_tag: u64,
}

impl Stream{
    pub fn new(source : &str)->Stream{
        Stream{
            stream : String::from_str(source),
            index : 0,
            line_begin : 0,
            line : 1,
            state_tag : 0,
        }
    }
    pub fn skip(&mut self, string :&str)->bool{
        let len = string.len();
        if self.stream.len() < len {
            false
        }else{
            let sliced_string = self.stream.as_slice().slice_chars(self.index.to_uint().unwrap(),string.len());
            if str::eq_slice(sliced_string, string) {
                self.index += len.to_u64().unwrap(); 
                return true;
            }
            false
        }
    }
}


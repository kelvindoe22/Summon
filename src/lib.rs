mod get;
use std::collections::HashMap;


pub struct _Response {
    _status_code: usize, 
    _content: String,
    _headers: HashMap<String, String>,
}
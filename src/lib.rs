mod get;
use std::collections::HashMap;


pub struct _Response {
    status_code: usize, 
    content: String,
    headers: HashMap<String, String>,
}
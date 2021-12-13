use std::collections::HashMap;

// a=1&b=2&c&d=&e===&d=7&d=abc\
#[derive(Debug)]
pub struct QueryString<'buffer> {
    data : HashMap<&'buffer str, Value<'buffer>>
}

#[derive(Debug)]
pub enum Value<'buffer> {
    Single(&'buffer str),
    Multiple(Vec<&'buffer str>)
}

impl<'buffer> QueryString<'buffer> {
    pub fn get(&self, key : &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buffer> From<&'buffer str> for QueryString<'buffer> {
    fn from(s : &'buffer str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i+1..];
            }

            data.entry(key)
            .and_modify(|existing : &mut Value| match existing {
                Value::Single(prev_value) => {
                    *existing = Value::Multiple(vec![prev_value, val]);
                },
                Value::Multiple(vector) => vector.push(val)
            })
            .or_insert(Value::Single(val));

        }
        
        QueryString { data }
    }
}
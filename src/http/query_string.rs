use std::collections::HashMap;
pub struct QueryString<'buffer> {
    data: Hashmap<&'buffer str, Value<'buffer>>
}

//enum variant to save either sting or array of strings.
pub enum Value<'buffer> {
    Single(&'buffer str),
    // Array length is specified at compile time,
    // We do not know the amount of values the array will have
    // Vec is heap allocated array that can grow dynamcially.
    Multiple(Vec<&'buffer str>),
}

impl<'buffer> QueryString<'buffer> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buffer> From<&'buffer str> for QueryString<'buffer> {
    fn from(_: &'buffer str) -> Self {
        let mut data = HashMap::new();

        for sub_string in s.split("&") {
            let  mut key = sub_string;
            let  mut value = "";
            if let Some(i) = s.find('=') {
                key = &sub_string[..i];
                value = &sub_string[i + 1..];
            }
        }
        Querystring { data }
    }
}
use std::collections::HashMap;

#[derive(Debug)]
pub struct PriorityMap {
    maps: Vec<HashMap<String, String>>,
}

impl PriorityMap {
    pub fn new(maps: Vec<HashMap<String, String>>) -> Self {
        PriorityMap { maps }
    }

    pub fn maps<'a>(&'a mut self) -> &'a mut Vec<HashMap<String, String>> {
        &mut self.maps
    }

    pub fn get(&self, key: &String) -> Option<&String> {
        self.maps.iter().find_map(|map| map.get(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::assert;

    #[test]
    fn basic() {
        let mut maps = PriorityMap::new(vec![
            HashMap::<String, String>::new(),
            HashMap::<String, String>::new(),
        ]);

        maps.maps()[0].insert(String::from("A"), String::from("a"));
        maps.maps()[0].insert(String::from("B"), String::from("b"));
        maps.maps()[1].insert(String::from("B"), String::from("bb"));
        maps.maps()[1].insert(String::from("C"), String::from("c"));

        assert!(maps.get(&String::from("A")) == Some(&String::from("a")));
        assert!(maps.get(&String::from("B")) == Some(&String::from("b")));
        assert!(maps.get(&String::from("C")) == Some(&String::from("c")));
        assert!(maps.get(&String::from("D")) == None);
    }
}

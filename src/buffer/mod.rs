pub struct Buffer {
    pub content: Vec<String>,
}

impl Buffer {
    pub fn new_from_str(str_to_convert: &str) -> Buffer {
        let content_to_go_in_buff = str_to_convert.split("\n");
        Buffer {
            content: content_to_go_in_buff.map(|c| c.to_string()).collect(),
        }
    }
    pub fn new_from_string(string_to_convert: String) -> Buffer {
        let content_to_go_in_buff = string_to_convert.as_str().split("\n");
        Buffer {
            content: content_to_go_in_buff.map(|c| c.to_string()).collect(),
        }
    }
    pub fn get_grid(&self, width: usize) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for i in self.content.iter() {
            result.append(&mut Buffer::recsplit(i.to_string(), width));
        }
        result
    }

    fn recsplit(thing_to_split: String, width: usize) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        if thing_to_split.as_str().len() > width {
            let tuple_of_strs = thing_to_split.as_str().split_at(width);
            result.push(tuple_of_strs.0.to_string());
            result.append(&mut Buffer::recsplit(tuple_of_strs.1.to_string(), width));
        } else {
            result.push(thing_to_split.to_owned());
        }
        result
    }
    pub fn insert_at(&mut self, place: (usize, usize), char_to_insert: char) {
        self.content[place.1].insert(place.0, char_to_insert);
    }
}

pub struct Buffer {
    pub cursor_pos: (usize, usize),
    pub content: Vec<String>,
}

impl Buffer {
    pub fn new_from_string(str_to_convert: &str) -> Buffer {
        let content_to_go_in_buff = str_to_convert.split("\n");
        Buffer {
            cursor_pos: (0, 0),
            content: content_to_go_in_buff.map(|c| c.to_string()).collect(),
        }
    }
    pub fn get_grid(&self, width: usize) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for i in self.content.iter() {
            if i.as_str().len() > width {
                let tuple_of_strs = i.as_str().split_at(width);
                result.push(tuple_of_strs.0.to_string());
                result.push(tuple_of_strs.1.to_string());
            } else {
                result.push(i.to_owned());
            }
        }
        result
    }

    fn recsplit(thing_to_split: String) -> Vec<String> {
            let mut result: Vec<String> = Vec::new();
            if thing_to_split.as_str().len() > width {
                let tuple_of_strs = i.as_str().split_at(width);
                result.push(tuple_of_strs.0.to_string());
                result.push(tuple_of_strs.1.to_string());
            } else {
                result.push(i.to_owned());
            }
        }
        result


    }
}

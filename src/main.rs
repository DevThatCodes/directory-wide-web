use std::fs;

pub mod rml_parser {

    #[derive(Debug)]
    pub struct RmlElement {
        name: String,
        attributes: Vec<(String, String)>,
        children: Vec<RmlElement>,
    }

    impl RmlElement {
        fn create_element(name: String, attributes: Vec<(String,String)>) -> Self {
            RmlElement {name, attributes, children: vec![]}
        }
        fn get_children(&self) -> &Vec<RmlElement> {
            &self.children
        }
        fn add_child(&mut self, element: RmlElement) {
            self.children.push(element);
        }
        fn tag_name(&self) -> &str {
            &self.name
        }
        fn get_attributes(&self) -> &Vec<(String, String)> {
            &self.attributes
        }
    }

    pub fn parse_to_list(string: &str) -> Vec<String> {
        let mut tag_builder : String = String::from("");
        let mut tag_stack : Vec<String> = vec![];
        let mut inside_tag = false;
        for c in string.chars() {
            if inside_tag {
                if c == ']' {
                    inside_tag = false;
                    tag_stack.push(tag_builder);
                    tag_builder = String::from("");
                } else {
                    tag_builder = tag_builder + &c.to_string();
                }
            }
            // check if inside tag
            if c == '[' {
                inside_tag = true;
            }
        }
        tag_stack
    }

    pub fn parse_to_complex_taglist(taglist: Vec<String>) -> Vec<(String, Vec<String>)> {
        let mut complex_taglist : Vec<(String, Vec<String>)> = vec![];
        for tag in taglist {
            let tag_data_unchanged : Vec<&str> = tag.split_whitespace().collect();
            let mut tag_data : Vec<String> = vec![];
            for string in tag_data_unchanged {
                tag_data.push(string.to_string());
            }
            let mut tag_name = tag_data[0].clone();
            tag_data.remove(0);
            if tag_name[0..1] == *":" {
                tag_data.push(String::from("end-tag"));
                tag_name.remove(0);
            }
            complex_taglist.push((tag_name.to_string(), tag_data));
        }
        complex_taglist
    }

    pub fn parse(complex_taglist: Vec<(String, Vec<String>)>) -> RmlElement {
        RmlElement::create_element(String::from("body"), vec![])
    }
}

fn main() {
    let content = fs::read_to_string("main.info").expect("No file found");
    let doc = rml_parser::parse(rml_parser::parse_to_complex_taglist(rml_parser::parse_to_list(&content)));
    println!("{:#?}", doc);
}

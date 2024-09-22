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

    pub fn parse(taglist: Vec<String>) -> RmlElement {
        let mut complex_taglist : Vec<(String, Vec<(String, String)>)> = vec![];
        let mut complex_tag_attrs : Vec<(String,String)> = vec![];
        for tag in taglist {
            println!("{}", tag);
        }
        RmlElement::create_element(String::from("body"), vec![])
    }
}

fn main() {
    let content = fs::read_to_string("main.info").expect("No file found");
    let doc = rml_parser::parse(rml_parser::parse_to_list(&content));
    println!("{:#?}", doc);
}

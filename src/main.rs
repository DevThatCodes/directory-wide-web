use std::fs;

pub mod rml_parser {
    use std::thread::scope;


    #[derive(Debug)]
    pub struct RmlElement {
        name: String,
        attributes: Vec<(String, String)>,
        children: Vec<RmlElement>,
    }

    impl RmlElement {
        pub fn create_element(name: String, attributes: Vec<(String,String)>) -> Self {
            RmlElement {name, attributes, children: vec![]}
        }
        pub fn get_children(&self) -> &Vec<RmlElement> {
            &self.children
        }
        pub fn add_child(&mut self, element: RmlElement) {
            self.children.push(element);
        }
        pub fn tag_name(&self) -> &str {
            &self.name
        }
        pub fn get_attributes(&self) -> &Vec<(String, String)> {
            &self.attributes
        }
    }

    pub fn parse_to_list(string: &str) -> Vec<String> {
        let mut tag_builder : String = String::from("");
        let mut tag_content_builder : String = String::from("");
        let mut tag_stack : Vec<String> = vec![];
        let mut inside_tag = false;
        let mut after_tag = false;
        for c in string.chars() {
            // do data between tags
            if after_tag && c != '[' {
                tag_content_builder = tag_content_builder + &c.to_string();
                //do some other stuff that is after a tag but doesnt depend on ^
            }

            // do inside tag data
            if inside_tag {
                if c == ']' {
                    inside_tag = false;
                    after_tag = true;
                    tag_stack.push(tag_builder);
                    tag_builder = String::from("");
                } else {
                    tag_builder = tag_builder + &c.to_string();
                }
            }

            // check if inside tag
            if c == '[' {
                if after_tag && !tag_content_builder.replace("\n", "").replace(" ", "").is_empty() {
                    tag_content_builder = String::from("~") + &tag_content_builder;
                    tag_stack.push(tag_content_builder);
                }
                tag_content_builder = String::from("");
                after_tag = false;
                inside_tag = true;
            }
        }
        tag_stack
    }

    pub fn parse_to_complex_taglist(taglist: Vec<String>) -> Vec<(String, Vec<String>)> {
        let mut complex_taglist : Vec<(String, Vec<String>)> = vec![];
        for tag in taglist {
            if tag[0..1] == *"~" {
                complex_taglist.push((tag[1..tag.len()].to_string(), vec![String::from("not-tag")]));
            } else {
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
                } else {
                    tag_data.push(String::from("start-tag"));
                }
                complex_taglist.push((tag_name.to_string(), tag_data));
            }
        }
        complex_taglist
    }

    pub fn parse(complex_taglist: Vec<(String, Vec<String>)>) -> RmlElement {
        let mut scope : Vec<String> = vec![];
        let mut children : Vec<RmlElement> = vec![];
        let mut content : Vec<String> = vec![];
        let mut supercomplex_tag_attrs : Vec<(String, String)> = vec![];

        for (complex_tag, complex_tag_attrs) in complex_taglist {

            // TODO: remove clones

            if complex_tag_attrs.contains(&String::from("start-tag")) {
                for complex_tag_attr in complex_tag_attrs.clone() {
                    let ctad : Vec<&str> = complex_tag_attr.split("=").collect();// complex_tag_attr_data
                    if ctad.len() > 1 {
                        supercomplex_tag_attrs.push((ctad[0].to_string(), ctad[1].to_string()))
                    }
                }
                scope.push(complex_tag.clone());
            }
            if complex_tag_attrs.contains(&String::from("not-tag")) {
                content.push(complex_tag.clone());
            }
            if complex_tag_attrs.contains(&String::from("end-tag")) {
                scope.pop();
                let mut element = RmlElement::create_element(complex_tag.clone(), supercomplex_tag_attrs.clone());
                if !scope.is_empty() {
                    children.push(element);
                } else {
                    for child in children {
                        element.add_child(child);   
                    }
                }
                supercomplex_tag_attrs.clear();
            }
        }
        
        println!("{:?}", children);
        RmlElement::create_element(String::from("body"), vec![])
    }
}

fn main() {
    let content = fs::read_to_string("main.info").expect("No file found");
    let doc = rml_parser::parse(rml_parser::parse_to_complex_taglist(rml_parser::parse_to_list(&content)));
    println!("doc: {:#?}", doc);

}

pub fn insert_after_element(xml: &str, content_to_insert: &str, element: &str) -> String {
    let ending_element = format!("</{}>", element);
    if let Some(pos) = xml.find(&ending_element) {
        let insert_position = pos + ending_element.len();
        let mut new_xml = String::new();
        new_xml.push_str(&xml[..insert_position]); // Add everything up to and including </Documento>
        new_xml.push('\n');
        new_xml.push_str(content_to_insert); // Add your new content
        new_xml.push_str(&xml[insert_position..]); // Add the rest of the original XML
        new_xml
    } else {
        // If </Documento> not found, return the original string (or handle error as needed)
        xml.to_string()
    }
}

pub fn insert_before_element(xml: &str, content_to_insert: &str, element: &str) -> String {
    let starting_element = format!("<{}", element);
    if let Some(pos) = xml.find(&starting_element) {
        let mut new_xml = String::new();
        new_xml.push_str(&xml[..pos]); // Add everything up to the start of the element
        new_xml.push('\n');
        new_xml.push_str(content_to_insert); // Add your new content
        new_xml.push_str(&xml[pos..]); // Add the rest of the original XML
        new_xml
    } else {
        // If <element> not found, return the original string (or handle error as needed)
        xml.to_string()
    }
}
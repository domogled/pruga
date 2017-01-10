
pub mod elm_sink;
pub mod from_file;
pub mod from_string;

pub use html2elm::from_file::from_file;
pub use html2elm::from_string::from_string;

// fn walk(indent: usize, handle: Handle) -> Result<String, String> {
//
// let mut output = String::new();
//
// let node = handle.borrow();
// FIXME: don't allocate
// print!("{}", repeat(" ").take(indent).collect::<String>());
//
// let indent_space = format!("{}", repeat(" ").take(indent).collect::<String>());
//
// output.push_str(&indent_space);
//
// match node.node {
// Document => println!("\n---------------------------\n#Document"),
//
// Doctype(ref name, ref public, ref system) => {
// println!("<!DOCTYPE {} \"{}\" \"{}\">", *name, *public, *system)
// }
//
// Text(ref text) => {
// println!("#text: {}", text.trim())//, // println!("#text: {}", escape_default(text)),
// let code = format!("text \"{}\"", text.trim());
// output.push_str(&code)
// }
//
// Comment(ref text) => {
// println!("<!-- {} -->", escape_default(text)),
// let code = format!("{{- {} -}}", text);
// output.push_str(&code)
// }
//
// Element(ref name, _, ref attrs) => {
// assert!(name.ns == ns!(html));
// print!("<{}", name.local);
//
// match name.local.as_ref() {
// "html" => print!("IGNORE {}", name.local),
//
// _ => {
// let code = parse_element()
// let code = format!("{}\n{}[", name.local, indent_space);
// output.push_str(&code)
// }
// }
//
// let code = format!("{}\n{}[", name.local, indent_space);
// output.push_str(&code);
//
// for attr in attrs.iter() {
//     assert!(attr.name.ns == ns!());
//     // print!(" {}=\"{}\"", attr.name.local, attr.value);
//     let code = format!(" {} \"{}\"", attr.name.local, attr.value);
//     output.push_str(&code);
// }
// println!(">");
// output.push_str("]");
// }
// }
//
// output.push_str(&format!("\n{}[", indent_space));
// for child in node.children.iter() {
// let result = walk(indent + 4, child.clone());
// match result {
// Ok(code) => {
// output.push_str(&code);
// }
//
// Err(err) => println!("Error walk {} ", err),
// }
// }
// output.push_str("]\n");
//
// Ok(output)
// }

// FIXME: Copy of str::escape_default from std, which is currently unstable
// pub fn escape_default(s: &str) -> String {
// s.chars().flat_map(|c| c.escape_default()).collect()
// }


// #[cfg(test)]
// mod tests {
// use super::*;
//
// #[test]
// fn kompiluji() {
// assert_eq!("<div>", escape_default("<div>"));
//
// let paths = fs::read_dir("./examples").unwrap();
//
// for path in paths {
// println!("Name: {}", path.unwrap().path().display())
// }
// }
// }
//
extern crate tendril;
extern crate html5ever;

use html5ever::tokenizer::{TokenSink, Token, ParseError, TokenSinkResult};
use html5ever::tokenizer::{DoctypeToken, CommentToken, CharacterTokens, NullCharacterToken,
                           TagToken, StartTag, EndTag, Tag, EOFToken, Attribute};

use std::iter::repeat;
// use std::collections::HashSet;
// use std::collections::HashMap;

static SPACE: usize = 4;

static ELM_HTML_TAGS: &'static [&'static str] = &["h1",
                                                  "h2",
                                                  "h3",
                                                  "h4",
                                                  "h5",
                                                  "h6",
                                                  "div",
                                                  "p",
                                                  "hr",
                                                  "pre",
                                                  "blockquote",
                                                  "span",
                                                  "a",
                                                  "code",
                                                  "em",
                                                  "strong",
                                                  "i",
                                                  "b",
                                                  "u",
                                                  "sub",
                                                  "sup",
                                                  "br",
                                                  "ol",
                                                  "ul",
                                                  "li",
                                                  "dl",
                                                  "dt",
                                                  "dd",
                                                  "img",
                                                  "iframe",
                                                  "canvas",
                                                  "math",
                                                  "form",
                                                  "input",
                                                  "textarea",
                                                  "button",
                                                  "select",
                                                  "option",
                                                  "section",
                                                  "nav",
                                                  "article",
                                                  "aside",
                                                  "header",
                                                  "footer",
                                                  "address",
                                                  "main_",
                                                  "body",
                                                  "figure",
                                                  "figcaption",
                                                  "table",
                                                  "caption",
                                                  "colgroup",
                                                  "col",
                                                  "tbody",
                                                  "thead",
                                                  "tfoot",
                                                  "tr",
                                                  "td",
                                                  "th",
                                                  "fieldset",
                                                  "legend",
                                                  "label",
                                                  "datalist",
                                                  "optgroup",
                                                  "keygen",
                                                  "output",
                                                  "progress",
                                                  "meter",
                                                  "audio",
                                                  "video",
                                                  "source",
                                                  "track",
                                                  "embed",
                                                  "object",
                                                  "param",
                                                  "ins",
                                                  "del",
                                                  "small",
                                                  "cite",
                                                  "dfn",
                                                  "abbr",
                                                  "time",
                                                  "var",
                                                  "samp",
                                                  "kbd",
                                                  "s",
                                                  "q",
                                                  "mark",
                                                  "ruby",
                                                  "rt",
                                                  "rp",
                                                  "bdi",
                                                  "bdo",
                                                  "wbr",
                                                  "details",
                                                  "summary",
                                                  "menuitem",
                                                  "menu"];

// pub enum Indenter {
// Space,
// Tab,
// }

// #[derive(Copy, Clone)]
pub struct ElmSink {
    // pub in_char_run: bool,
    elm_code: Vec<String>,
    depth: usize,
    indent: String,
    indent_size: usize,
    imported_html_tags: Vec<String>,
    imported_attributes: Vec<String>,
    imported_events: Vec<String>,
    context: Vec<bool>,
}



impl ElmSink {
    pub fn new(indent_size: usize) -> ElmSink {

        // let indenter = match indenter {
        // Indenter::Space => " ",
        // Indenter::Tab => "\t",
        // };

        ElmSink {
            // in_char_run: _in_char_run,
            // code: String::new(),
            elm_code: Vec::new(),
            indent_size: indent_size,
            depth: 0,
            indent: String::new(),
            imported_html_tags: Vec::new(),
            imported_attributes: Vec::new(),
            imported_events: Vec::new(),
            context: Vec::new(),
        }
    }

    pub fn generate_code(&mut self) -> String {

        if self.elm_code.len() == 0 {
            return String::new();
        }

        let mut code: Vec<String> = Vec::new();

        code.push("module View exposing(view)".to_string());
        code.push(String::new());

        code.push("import Html exposing (Html)".to_string());

        self.imported_html_tags.sort();
        self.imported_html_tags.dedup();

        let imports = self.imported_html_tags.join(", ");
        code.push(format!("import Html exposing ({})", imports));

        self.imported_attributes.sort();
        self.imported_attributes.dedup();
        let imports = self.imported_attributes.join(", ");
        code.push(format!("import Html.Attributes exposing ({})", imports));
        code.push(String::new());

        // @TODO: cannot move out of borrowed content
        // let mut imported_html_tags = self.imported_html_tags;
        // let x = self.generate_import(&self.imported_html_tags, "Html".to_string());

        // code.append();
        // code.append(&mut self.elm_code);

        code.push("view : Model -> Html Msg".to_string());
        let def_func = "view model =".to_string();
        let len_func = def_func.len();
        let indent_func = format!("{}", repeat(" ").take(len_func).collect::<String>());
        let ref elm_code = self.elm_code;
        code.push(def_func);

        // code.append(&mut self.elm_code);
        {
            for line in elm_code {
                code.push(format!("{}{}", indent_func, line));
            }
        }

        code.join("\n")
    }

    // pub fn generate_import(self, items: &Vec<String>, from: String) -> Vec<String> {
    // let output = Vec::new();
    //
    // output
    // }

    fn add_indent(&mut self) {

        self.depth += 1;

        self.indentize();

    }

    fn drop_indent(&mut self) {

        if (self.depth > 0) {
            self.depth -= 1;
            self.indentize();
        }

    }

    fn indentize(&mut self) {
        let indent_size = self.depth * SPACE;

        self.indent = format!("{}", repeat(" ").take(indent_size).collect::<String>());
    }


    fn elm_html_tag(&mut self, tag: &Tag) {

        match tag.kind {
            StartTag => {
                // seznam importovaných elementů
                let name = format!("{}", tag.name);
                self.imported_html_tags.push(name);

                // odsazení tagu
                // je prvním elementem ve větvi?
                let start_code = match (self.depth, self.context.get(self.depth)) {
                    // root element a je první
                    (0, None) => {
                        self.context.push(true);
                        format!("{}{}", self.indent, tag.name)
                    }
                    // root element jako druhý v pořadí - chyba, vše musí být v jednom jediném kořenovém elementu
                    (0, Some(_)) => panic!("Pouze jeden kořenový element může být definován"),
                    // potomek, první ve větvi
                    (_, None) => {
                        self.context.push(true);
                        format!("{}[ {}", self.indent, tag.name)
                    }
                    // potomek, další ve větvi
                    (_, Some(_)) => format!("{}, {}", self.indent, tag.name),
                };


                self.add_indent();

                self.attributes_to_elm(start_code, &tag.attrs);


            }
            EndTag => {

                match self.context.get(self.depth) {

                    // potomek, první ve větvi
                    None => {
                        self.elm_code.push(format!("{}[]", self.indent));
                    }
                    // potomek, další ve větvi
                    Some(_) => {
                        self.context.pop();
                        self.elm_code.push(format!("{}]", self.indent));
                    }
                };

                self.drop_indent();

            }
        }



    }

    fn attributes_to_elm(&mut self, start_code: String, attrs: &Vec<Attribute>) {

        match attrs.len() {
            0 => self.elm_code.push(format!("{} []", start_code)),
            1 => {
                let attr = self.parse_attr(attrs.first().unwrap());
                self.elm_code
                    .push(format!("{} [{}]", start_code, attr))
            }
            _ => {
                self.elm_code.push(start_code);
                let (first, others) = attrs.split_first().unwrap();

                let attr = self.parse_attr(first);
                self.elm_code.push(format!("{}[ {}", self.indent, attr));

                for attr in others {
                    let attr = self.parse_attr(attr);
                    self.elm_code.push(format!("{}, {}", self.indent, attr));
                }
                // let attrs = string_list.join(&format!("\n{}, ", indent));
                // format!("[ {}\n{}]\n{}", attrs, indent, indent)
                self.elm_code.push(format!("{}]", self.indent));
            }
        }
    }

    fn parse_attr(&mut self, attr: &Attribute) -> String {
        let name = format!("{}", attr.name.local);
        self.imported_attributes.push(name);
        format!("{} \"{}\"", attr.name.local, attr.value)
    }

    fn unknown_tag(&mut self, tag: &Tag) {
        match tag.kind {
            StartTag => print!("UNKNOWN TAG: <\x1b[32m{}\x1b[0m", tag.name),

            EndTag => print!("UNKNOWN TAG: <\x1b[31m/{}\x1b[0m", tag.name),
        }
        for attr in tag.attrs.iter() {
            print!(" \x1b[36m{}\x1b[0m='\x1b[34m{}\x1b[0m'",
                   attr.name.local,
                   attr.value);
        }
        if tag.self_closing {
            print!(" \x1b[31m/\x1b[0m");
        }
        println!(">");
    }
}

impl TokenSink for ElmSink {
    type Handle = ();

    fn process_token(&mut self,
                     token: Token /* , _line_number: u64 */)
                     -> TokenSinkResult<()> {

        match token {
            DoctypeToken(_) => {}

            CommentToken(comment) => {
                self.elm_code.push(format!("{{-{}-}}", comment));
            }

            CharacterTokens(b) => {
                // for c in b.chars() {
                //     self.do_char(c);
                // }
                let text = b.trim();

                // println!("text \"{}\"", text);

                if !text.is_empty() {

                    // odsazení tagu
                    // je prvním elementem ve větvi?
                    let start_code = match self.context.get(self.depth) {

                        // potomek, první ve větvi
                        None => {
                            self.context.push(true);
                            format!("{}[", self.indent)
                        }
                        // potomek, další ve větvi
                        Some(_) => format!("{},", self.indent),
                    };

                    self.elm_code.push(format!("{} text \"{}\"", start_code, text));
                    self.imported_html_tags.push("text".to_string());
                }

            }
            // NullCharacterToken => self.do_char('\0'),
            TagToken(tag) => {

                let name = tag.name.chars().as_str();

                match name {
                    name if ELM_HTML_TAGS.contains(&name) => self.elm_html_tag(&tag),
                    _ => self.unknown_tag(&tag),
                }


            }

            ParseError(err) => {
                // self.is_char(false);
                panic!("ERROR: {}", err);
            }

            EOFToken => {
                println!("EOFToken: {:?}", token);
            }

            NullCharacterToken => {}
        }
        TokenSinkResult::Continue
    }
}

use pest::Parser;

#[derive(Parser)]
#[grammar = "wofi.pest"]
pub struct WofiParser;

use crate::color::wofi_colors::{CSSProp, Selector, WofiColor};
use pest::iterators::{Pair, Pairs};

impl WofiColor {
    pub(crate) fn from_css(content: &str) -> WofiColor {
        let file = WofiParser::parse(Rule::file, content)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let mut wofi_style = WofiColor::default();

        for pair in file.into_inner() {
            match pair.as_rule() {
                Rule::selector => {
                    let selector: Pairs<Rule> = pair.into_inner();
                    selector.into_iter().for_each(|pair| match pair.as_rule() {
                        Rule::window_selector => wofi_style.window = parse_prop(pair),
                        Rule::input_selector => wofi_style.input = parse_prop(pair),
                        Rule::input_focus_selector => wofi_style.input_focused = parse_prop(pair),
                        Rule::inner_box_selector => wofi_style.inner_box = parse_prop(pair),
                        Rule::inner_box_flowbox_child_selector => {
                            wofi_style.inner_box_flowchild = parse_prop(pair)
                        }
                        Rule::outer_box_selector => wofi_style.outer_box = parse_prop(pair),
                        Rule::scroll_selector => wofi_style.scroll = parse_prop(pair),
                        Rule::text_selector => wofi_style.text = parse_prop(pair),
                        _ => eprintln!("unknown prop {:?}", pair),
                    });
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }
        wofi_style
    }
}

fn parse_prop(pair: Pair<Rule>) -> Selector {
    let css_props = pair.into_inner().next().unwrap().into_inner();

    let mut props = vec![];

    css_props.into_iter().for_each(|prop| {
        let mut key_value = prop.into_inner().next().unwrap().into_inner().into_iter();

        let key = key_value.next().unwrap().as_str().to_string();
        let mut value = key_value.next().unwrap().as_str().to_string();

        if let Some(more_value) = key_value.next() {
            value.push_str(&format!(" {}", more_value.as_str()))
        }

        props.push(CSSProp { key, value })
    });

    Selector { props }
}

impl Default for WofiColor {
    fn default() -> Self {
        WofiColor {
            window: Default::default(),
            input: Default::default(),
            input_focused: Default::default(),
            inner_box: Default::default(),
            inner_box_flowchild: Default::default(),
            scroll: Default::default(),
            outer_box: Default::default(),
            text: Default::default(),
        }
    }
}

impl ToString for WofiColor {
    fn to_string(&self) -> String {
        let mut output = String::new();
        output += "window";
        output += &self.window.to_css();
        output += "#input";
        output += &self.input.to_css();
        output += "#input:focus";
        output += &self.input_focused.to_css();
        output += "#inner-box";
        output += &self.inner_box.to_css();
        output += "#inner-box flowboxchild:focus";
        output += &self.inner_box_flowchild.to_css();
        output += "#outer-box";
        output += &self.outer_box.to_css();
        output += "#text";
        output += &self.text.to_css();
        output += "#scroll";
        output += &self.scroll.to_css();
        output
    }
}

impl ToString for Selector {
    fn to_string(&self) -> String {
        let mut output = String::new();
        output += "\n";
        self.props.iter().for_each(|prop| {
            output += &prop.key;
            output += ": ";
            output += &prop.value;
            output += ";\n";
        });
        output
    }
}

impl Selector {
    fn to_css(&self) -> String {
        format!(" {{{}}}\n", self.to_string())
    }

    pub(crate) fn set_bg_color(&mut self, value: &str) {
        if let Some(prop) = self
            .props
            .iter_mut()
            .find(|prop| prop.key == "background-color")
        {
            prop.value = value.to_string();
        } else {
            panic!("No such CSS property : 'background-color'")
        }
    }

    pub(crate) fn set_color(&mut self, value: &str) {
        if let Some(prop) = self.props.iter_mut().find(|prop| prop.key == "color") {
            prop.value = value.to_string();
        } else {
            panic!("No such CSS property 'color'")
        }
    }

    pub(crate) fn set_border_color(&mut self, value: &str) {
        if let Some(prop) = self.props.iter_mut().find(|prop| prop.key == "border") {
            let split: Vec<&str> = prop.value.split(" ").collect();

            prop.value = format!("{} {} {}", split[0], split[1], value);
        } else {
            panic!("No such CSS property : 'border'")
        }
    }
}

impl Default for Selector {
    fn default() -> Self {
        Selector { props: vec![] }
    }
}

#[cfg(test)]
mod test {
    use crate::color::wofi_colors::WofiColor;
    use std::ops::Not;

    #[test]
    fn parse_wofi_config_ok() {
        let content = std::fs::read_to_string("tests/wofi_styles.css").unwrap();
        let style = WofiColor::from_css(&content);

        assert!(style.outer_box.props.is_empty().not());
        assert!(style.scroll.props.is_empty().not());
        assert!(style.text.props.is_empty().not());
        assert!(style.inner_box_flowchild.props.is_empty().not());
        assert!(style.inner_box.props.is_empty().not());
        assert!(style.input_focused.props.is_empty().not());
        assert!(style.input.props.is_empty().not());
    }
}

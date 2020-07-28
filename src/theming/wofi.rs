#[derive(Clone, Debug)]
pub struct Wofi {
    pub window: Selector,
    pub input: Selector,
    pub input_focused: Selector,
    pub inner_box: Selector,
    pub inner_box_flowchild: Selector,
    pub scroll: Selector,
    pub outer_box: Selector,
    pub text: Selector,
}

#[derive(Clone, Debug)]
pub struct CSSProp {
    pub(crate) key: String,
    pub(crate) value: String,
}

#[derive(Clone, Debug)]
pub struct Selector {
    pub(crate) props: Vec<CSSProp>,
}

impl Default for Wofi {
    fn default() -> Self {
        Wofi {
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

impl Default for Selector {
    fn default() -> Self {
        Selector { props: vec![] }
    }
}

#[derive(Debug, Serialize, Clone)]
pub(crate) struct SwayColor {
    pub(crate) colors: Vec<String>,
}

impl ToString for SwayColor {
    fn to_string(&self) -> String {
        self.colors.join("\n")
    }
}

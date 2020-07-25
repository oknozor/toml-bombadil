use crate::color::{ToConfig, Theme};
use anyhow::Result;
struct WofiColor {
  window_background: String,
  input_background: String,
  input_primary: String,
  input_border: String,
  input_focused_background: String,
  input_focused_primary: String,
  input_focused_border: String,
  inner_box_background: String,
  inner_box_focused_background: String,
  outer_box_background: String,
  scroll: String,
  text: String,
  text_background: String,
}

/*
window {
margin: 5px;
padding: 5px;
background-color:  #434c5e;
}

#input {
background-color:  #434c5e;
color: #eceff4;
border: 0px solid  #434c5e;
}

#input:focus {
background-color: #5e81ac;
color: #eceff4;
border: 0px solid   #434c5e;
}

#inner-box {
background-color:  #434c5e;
}

#inner-box flowboxchild:focus {
background-color: #5e81ac
}

#outer-box {
background-color:  #434c5e;
}

#scroll {
background-color: #434c5e;
}

#text {
margin: 2px;
background-color: rgba(0,0,0,0);
color: #eceff4;
}
*/
impl ToConfig for WofiColor {
  fn write() -> Result<()> {
    unimplemented!()
  }

  fn from_theme(theme: Theme) -> Self {
    unimplemented!()
  }
}

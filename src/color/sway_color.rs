/*
set $black    #3b4252
set $gray     #d8dee9
set $lred     #bf616a
set $blue     #5e81ac
set $lblue    #88c0d0
set $white    #eceff4
set $yellow   #ebcb8b

# Color                 border    background text    indicator  child_border
client.focused          $yellow   $blue      $gray   $yellow     $yellow
client.focused_inactive $white    $black     $gray   $gray      $gray
client.unfocused        $black    $black     $gray   $black     $blue
client.urgent           $lred     $black     $gray   $lred      $lblue
*/

struct SwayColor {
  focused_border: String,
  focused_background: String,
  focused_text: String,
  focused_indicator: String,
  focused_child_border: String,

  focused_inactive_border: String,
  focused_inactive_background: String,
  focused_inactive_text: String,
  focused_inactive_child_border: String,

  focused_inactive_indicator: String,
  unfocused_border: String,
  unfocused_background: String,
  unfocused_text: String,
  unfocused_indicator: String,
  unfocused_child_border: String,

  urgent_border: String,
  urgent_background: String,
  urgent_text: String,
  urgent_indicator: String,
  urgent_child_border: String,
}
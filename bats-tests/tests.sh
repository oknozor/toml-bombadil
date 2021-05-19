#!/usr/bin/env bats

load 'bats-support/load'
load 'bats-assert/load'
load 'bats-file/load'

@test "Print version" {
  run bombadil --version
  assert_success
}

@test "Print help" {
  run bombadil --help
  assert_success
}

@test "Symlink bombadil config" {
  run bombadil install -c "$HOME/dotfiles/bombadil.toml"
  assert_success
  assert_symlink_to "$HOME/dotfiles/bombadil.toml" "$HOME/.config/bombadil.toml"
}

@test "Links dots" {
  run bombadil link
  assert_success
  assert_file_exist "$HOME/dotfiles/.dots/dummy.dot"
  assert_file_exist "$HOME/dotfiles/.dots/maven/settings.xml"
  assert_symlink_to "$HOME/dotfiles/.dots/dummy.dot" "$HOME/.config/dummy.dot"
  assert_symlink_to "$HOME/dotfiles/.dots/maven/settings.xml" "$HOME/.m2/settings.xml"
}

@test "Vars injected" {
  run bombadil link
  assert_success
  assert_file_exist "$HOME/.config/dummy.dot"
  assert_file_contains "$HOME/.config/dummy.dot" "red is #de1f1f"
  assert_file_contains "$HOME/.config/dummy.dot" "green is #41de1f"
}

@test "Link profile" {
  run bombadil link -p i3
  assert_success
  assert_file_exist "$HOME/.config/i3/config"
  assert_symlink_to "$HOME/dotfiles/.dots/i3" "$HOME/.config/i3"
}

@test "Unlink works" {
  run bombadil unlink
  assert_success
  assert_not_symlink_to "$HOME/dotfiles/.dots/i3" "$HOME/.config/i3"
  assert_not_symlink_to "$HOME/dotfiles/.dots/dummy.dot" "$HOME/.config/dummy.dot"
}

@test "Link multiple profile" {
  run bombadil link -p i3 corporate
  assert_success
  assert_file_exist "$HOME/.config/i3/config"
  assert_symlink_to "$HOME/dotfiles/.dots/i3" "$HOME/.config/i3"
  assert_symlink_to "$HOME/dotfiles/.dots/maven/corporate.settings.xml" "$HOME/.m2/settings.xml"
}


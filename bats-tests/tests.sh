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
  assert_symlink_to "$HOME/dotfiles/.dots/dummy.dot" "$HOME/.config/dummy.dot"
}

@test "Vars injected" {
  run bombadil link
  assert_success
  assert_file_exist "$HOME/.config/dummy.dot"
  assert_file_contains "$HOME/.config/dummy.dot" "red is #de1f1f"
  assert_file_contains "$HOME/.config/dummy.dot" "green is #41de1f"
}


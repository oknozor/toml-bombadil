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
}

@test "Links dots" {
  run bombadil link
  assert_success
  assert_file_exist "$HOME/.config/dummy.dot"
}

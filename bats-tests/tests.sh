#!/usr/bin/env bats

load 'bats-support/load'
load 'bats-assert/load'
load 'bats-file/load'

# FIXME : should use setup_file() here
setup() {
    GPG_KEY=$HOME/gpg_keys
    gpg --import "$GPG_KEY"/private.gpg
    gpg --import "$GPG_KEY"/public.gpg
    echo -e "5\ny\n" | gpg --no-tty --command-fd 0 --expert --edit-key test@toml.bombadil.org trust
}

@test "Print version" {
  run bombadil --version
  assert_success
}

@test "Print help" {
  run bombadil --help
  assert_success
}

## FIXME: This seems to timeout and return status 1, is it bats timing out or something else ?
# @test "Install from remote" {
#   run bombadil clone https://github.com/oknozor/dotfiles.git -t oknozor_dotfiles
#   assert_success
#   assert_symlink_to "$HOME/oknozor_dotfiles/bombadil.toml" "$HOME/.config/bombadil.toml"
#   assert_file_exist "$HOME/.config/i3"
#   assert_symlink_to "$HOME/oknozor_dotfiles/.dots/i3/wm" "$HOME/.config/i3"
# }

@test "Symlink bombadil config" {
  run bombadil install "$HOME/dotfiles"
  assert_success
  assert_symlink_to "$HOME/dotfiles/bombadil.toml" "$HOME/.config/bombadil.toml"
}

@test "Should add secret" {
  run bombadil add-secret -k "server_password" -v "hunter2" -f "$HOME/dotfiles/vars.toml"
  assert_output --partial 'Added server_password : hunter2'

  run bombadil get secrets
  assert_output --partial '"server_password": "hunter2"'

  run bombadil link
  assert_success

  assert_file_contains "$HOME/.config/dummy.dot" "secret is hunter2"
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

@test "Hooks works" {
  run bombadil link
  assert_output --partial "Hello from bombadil"
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

@test "Link meta profile" {
  run bombadil link -p corporate-sway
  assert_success
  assert_symlink_to "$HOME/dotfiles/.dots/sway" "$HOME/.config/sway"
  assert_file_exist "$HOME/.config/sway/config"
  assert_symlink_to "$HOME/dotfiles/.dots/maven/corporate.settings.xml" "$HOME/.m2/settings.xml"
}


@test "Profile should override default vars" {
  run bombadil link
  assert_success
  assert_file_exist "$HOME/.bashrc"
  assert_symlink_to "$HOME/dotfiles/.dots/bashrc" "$HOME/.bashrc"
  assert_file_contains "$HOME/.bashrc" "export JAVA_HOME=/etc/java11-openjdk"

  run bombadil link -p java-16
  assert_success
  assert_file_contains "$HOME/.bashrc" "export JAVA_HOME=/etc/java16-openjdk"
}

@test "Profile should override scoped vars" {
  run bombadil link
  assert_success
  assert_file_exist "$HOME/.bashrc"
  assert_symlink_to "$HOME/dotfiles/.dots/bashrc" "$HOME/.bashrc"
  assert_file_contains "$HOME/.bashrc" "export JAVA_HOME=/etc/java11-openjdk"

  run bombadil link -p java-17
  assert_success
  assert_file_contains "$HOME/.bashrc" "export JAVA_HOME=/etc/java17-openjdk"
}


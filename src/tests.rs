// use crate::add;
use assert_cmd::Command;
use std::env;
use std::path::PathBuf;

fn test_env() -> PathBuf {
  let mut tempesta_config =
    PathBuf::from(env::var("HOME").expect("HOME environment variable not set"));
  tempesta_config.push(".config/tempesta/test.toml");
  env::set_var("TEMPESTA_CONFIG", tempesta_config);
  PathBuf::from(env::var("HOME").expect("HOME environment variable not set"))
}

#[test]
fn tempesta_init() {
  let home = test_env();
  let output = "Where do you want to store the bookmarks? [~/.bookmark-store]: Do you want to use Git for tracking bookmarks? (Y/n): Tempesta initialized successfully: HOME/.config/tempesta/test.toml\n"
        .replace("HOME", home.to_str().expect("Unable to convert HOME dir to str"));
  Command::cargo_bin("tempesta")
    .unwrap()
    .arg("init")
    .write_stdin("~/.bookmark-store-test\nno\n")
    .assert()
    .success()
    .stdout(output);
  //TODO: assert file is created and looks as expected
}

#[test]
fn tempesta_add_overwrite_move_remove() {
  let home = test_env();

  // add
  let output_add = "Bookmark file stored at HOME/.bookmark-store-test/bookmark-title-test.toml\nBookmark added successfully as bookmark-title-test\n"
        .replace("HOME", home.to_str().expect("Unable to convert HOME dir to str"));
  Command::cargo_bin("tempesta")
    .unwrap()
    .args([
      "add",
      "bookmark-title-test",
      "https://url-test.local",
      "test-tag",
    ])
    .assert()
    .success()
    .stdout(output_add);

  // add (again but this time overwrite)
  let output_add_overwrite = "Bookmark already exists at HOME/.bookmark-store-test/bookmark-title-test.toml. Overwrite? (y/N): Overwriting file...\nBookmark file stored at HOME/.bookmark-store-test/bookmark-title-test.toml\nBookmark added successfully as bookmark-title-test\n"
        .replace("HOME", home.to_str().expect("Unable to convert HOME dir to str"));
  Command::cargo_bin("tempesta")
    .unwrap()
    .args([
      "add",
      "bookmark-title-test",
      "https://www.google.com/",
      "test-tag",
    ])
    .write_stdin("y\n")
    .assert()
    .success()
    .stdout(output_add_overwrite);

  //open, should the output
  let output_open = "Browser opened\n".replace(
    "HOME",
    home.to_str().expect("Unable to convert HOME dir to str"),
  );
  Command::cargo_bin("tempesta")
    .unwrap()
    .args(["open", "bookmark-title-test"])
    .assert()
    .success()
    .stdout(output_open);

  // move
  let output_move =
    "Bookmark moved successfully from bookmark-title-test to move/test\n";
  Command::cargo_bin("tempesta")
    .unwrap()
    .args(["move", "bookmark-title-test", "move/test"])
    .assert()
    .success()
    .stdout(output_move);

  //list
  let output_list = "move/test :: https://www.google.com/\n";
  Command::cargo_bin("tempesta")
    .unwrap()
    .args(["list"])
    .assert()
    .success()
    .stdout(output_list);

  // remove (removing the last entry in the bookmark-store-test removes it completely)
  let output_remove = "Bookmark removed successfully as move/test\n";
  Command::cargo_bin("tempesta")
    .unwrap()
    .args(["remove", "move/test"])
    .assert()
    .success()
    .stdout(output_remove);
  // TODO: cleanup ~/.config/tempesta/tempesta.toml

  //TODO: if panick remove files
}

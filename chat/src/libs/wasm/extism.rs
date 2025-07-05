use extism::*;
use anyhow::Result;

pub fn run_wasm() -> Result<()> {
  let url = Wasm::url(
    "https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm"
  );
  let manifest = Manifest::new([url]);
  let mut plugin = Plugin::new(&manifest, [], true).unwrap();
  let res = plugin.call::<&str, &str>("count_vowels", "Hello, world!").unwrap();
  println!("{}", res);
  Ok(())
}

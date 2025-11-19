use anyhow::{Ok, Result};
use minijinja::Environment;
use std::ops::Deref;
use std::path::Path;

#[derive(Debug)]
pub struct Tmpls<'s> {
    pub env: Environment<'s>,
}

impl<'s> Tmpls<'s> {
    pub fn new<T: AsRef<Path> + ?Sized>(path: &'s T) -> Result<Tmpls<'s>> {
        let mut env = Environment::new();
        let entries = std::fs::read_dir(path.as_ref())?;
        for entry in entries {
            let entry = entry?;
            let name = entry
                .file_name()
                .to_str()
                .expect("Invalid file name")
                .to_owned();
            let path = entry.path();
            let content = std::fs::read_to_string(&path)?;
            env.add_template_owned(name, content)?;
        }
        Ok(Tmpls { env })
    }
}

impl<'s> Deref for Tmpls<'s> {
    type Target = Environment<'s>;
    fn deref(&self) -> &Self::Target {
        &self.env
    }
}

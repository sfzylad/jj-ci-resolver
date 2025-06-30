use std::path::PathBuf;

use anyhow::{Error, Ok, Result};
use jj_lib::config::{ConfigFile, ConfigLayer, ConfigSource, ConfigValue};

#[derive(Debug)]
pub struct Revsets {
    layer: ConfigLayer,
}

#[derive(Debug)]
pub enum Alias {
    Success,
    Failures,
    Pending,
    Canceled,
}

impl Revsets {
    pub fn new(source: ConfigSource, path: PathBuf) -> Result<Self, Error> {
        let res = Self {
            layer: ConfigLayer::load_from_file(source, path)?,
        };

        Ok(res)
    }

    pub fn clean(&mut self) -> Result<Option<ConfigValue>, Error> {
        // NOTE: Using `dummy` value here because the JJ template will fail if
        // this value is an empty string.
        self.layer
            .set_value(r#"revset-aliases."ci_failures""#, "dummy")?;
        self.layer
            .set_value(r#"revset-aliases."ci_success""#, "dummy")?;
        self.layer
            .set_value(r#"revset-aliases."ci_pending""#, "dummy")?;
        self.layer
            .set_value(r#"revset-aliases."ci_canceled""#, "dummy")?;
        Ok(None)
    }

    pub fn update_alias(&mut self, refs: Vec<String>, alias: Alias) -> Result<(), Error> {
        let r: Vec<String> = refs.iter().map(|x| format!("present({x})")).collect();
        let value = r.join("|");

        match alias {
            Alias::Success => {
                let _ = self
                    .layer
                    .set_value(r#"revset-aliases."ci_success""#, value)?;
            }
            Alias::Failures => {
                let _ = self
                    .layer
                    .set_value(r#"revset-aliases."ci_failures""#, value)?;
            }
            Alias::Pending => {
                let _ = self
                    .layer
                    .set_value(r#"revset-aliases."ci_pending""#, value)?;
            }
            Alias::Canceled => {
                let _ = self
                    .layer
                    .set_value(r#"revset-aliases."ci_canceled""#, value)?;
            }
        }

        let _cfg = ConfigFile::from_layer(self.layer.clone().into()).unwrap();
        _cfg.save()?;

        Ok(())
    }
}

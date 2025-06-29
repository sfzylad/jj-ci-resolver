use std::path::PathBuf;

use anyhow::{Error, Result};
use jj_lib::config::{ConfigFile, ConfigLayer, ConfigLoadError, ConfigSource};

#[derive(Debug)]
pub struct Revsets {
    layer: ConfigLayer,
}

impl Revsets {
    pub fn new(source: ConfigSource, path: PathBuf) -> Result<Self, ConfigLoadError> {
        let res = Self {
            layer: ConfigLayer::load_from_file(source, path)?,
        };

        Ok(res)
    }

    pub fn set_ci_success(&mut self, refs: Vec<String>) -> Result<(), Error> {
        let r: Vec<String> = refs.iter().map(|x| format!("present({x})")).collect();
        let value = r.join("|");

        let _ = self
            .layer
            .set_value(r#"revset-aliases."ci_success""#, value)
            .unwrap();

        let _cfg = ConfigFile::from_layer(self.layer.clone().into()).unwrap();
        _cfg.save()?;

        Ok(())
    }

    pub fn set_ci_failures(&mut self, refs: Vec<String>) -> Result<(), Error> {
        let r: Vec<String> = refs.iter().map(|x| format!("present({x})")).collect();
        let value = r.join("|");

        let _ = self
            .layer
            .set_value(r#"revset-aliases."ci_failures""#, value)
            .unwrap();

        let _cfg = ConfigFile::from_layer(self.layer.clone().into()).unwrap();
        _cfg.save()?;

        Ok(())
    }

    pub fn set_ci_pending(&mut self, refs: Vec<String>) -> Result<(), Error> {
        let r: Vec<String> = refs.iter().map(|x| format!("present({x})")).collect();
        let value = r.join("|");

        let _ = self
            .layer
            .set_value(r#"revset-aliases."ci_pending""#, value)
            .unwrap();

        let _cfg = ConfigFile::from_layer(self.layer.clone().into()).unwrap();
        _cfg.save()?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_set_ci_success() {
        todo!();
    }
}

use resext::ResExt;

ResExt! {
    pub enum We {
        Wo(std::io::Error),
        Other,
    }
}

#[test]
#[should_panic]
fn wrapper() {
    fn temp() -> Res<()> {
        let path = "non_existent";
        let _ = std::fs::read(path)
            .context("Test")
            .with_context(|| format!("Failed to read file: {}", path))?;
        Ok(())
    }
    temp().unwrap();
}

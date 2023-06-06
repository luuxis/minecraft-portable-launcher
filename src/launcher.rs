pub struct Launcher {
    pub program: String,
    pub arguments: Vec<String>,
}

impl Launcher {
    pub fn execute(&self) -> Result<(), std::io::Error> {
        let mut command = std::process::Command::new(std::env::current_dir().unwrap().join(&self.program).to_str().unwrap());
        if !self.arguments.is_empty() {
            command.args(&self.arguments);
        }
        command.spawn()?;
        Ok(())
    }
}

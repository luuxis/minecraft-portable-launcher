use std::time::Duration;

mod detail_env {
    use crate::launcher::Launcher;
    pub const URL: &str = "https://launcher.mojang.com/download/Minecraft.exe";
    pub const PATH: &str = "Minecraft/Minecraft Launcher/Minecraft.exe";

    lazy_static::lazy_static! {
        pub static ref LAUNCHER: Option<Launcher> = Some(Launcher {
            program: String::from(PATH),
            arguments: vec![
                "--workDir".into(),
                "../.minecraft".into(),
                "--tmpDir".into(),
                "./tmp".into(),
                "--user-data-dir".into(),
                "../data user".into()
            ],
        });
    }
}

pub const DELAY_WINDOW: Duration = Duration::from_millis(500);

pub use detail_env::*;
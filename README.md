# Minecraft Portable Launcher
## Prérequis

Pour compiler le programme, il est nécessaire d'avoir MSVC et l'environnement Rust. 

Pour MSVC, installez [Build Tools pour Visual Studio 2022](https://aka.ms/vs/17/release/vs_BuildTools.exe) ou [Visual Studio Community 2022](https://visualstudio.microsoft.com/fr/thank-you-downloading-visual-studio/?sku=Community&channel=Release&version=VS2022&source=VSLandingPage&cid=2030&passive=false)

Pour l'environnement Rust, veuillez télécharger [rustup-init](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe) qui installera rustup et vous demandera de télécharger une *toolchain* (aka le compilateur sous une configuration spécifique pour votre ordinateur). Lorsqu'il vous le sera demandé, prenez celle par défaut. Celle-ci devrait ressembler à `stable-x86_64-pc-windows-msvc`

 
 Plateforme | Build debug | Build release |
| --------- | ----------- | ------------- |
| Windows x64 | `cargo build` | `cargo build --release` |
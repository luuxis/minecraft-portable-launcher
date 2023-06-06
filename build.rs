extern crate winres;

fn main() {
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    let manifest = std::fs::read_to_string("assets/Minecraft.manifest").expect("Failed to read manifest");
    res
      .set_icon("assets/images/icon.ico")
      .set_manifest(&manifest);
    res.compile().unwrap();
  }
}
use std::{
    fs::{create_dir, read_to_string, File},
    io::Write,
    process::Command,
};

use catppuccin::PALETTE;

fn main() {
    let svg = read_to_string("wallpaper.svg").unwrap();
    for flavor in &PALETTE {
        flavor.iter().for_each(|x| {
            if x.accent {
                let _ = create_dir("svgs");
                let _ = create_dir("pngs");

                let file_name = format!(
                    "svgs/error-wallpaper-{}-{}.svg",
                    flavor.name.to_string().to_lowercase(),
                    x.name.to_string().to_lowercase()
                );

                let mut file = File::create(&file_name).unwrap();

                let svg = svg
                    .clone()
                    .replace("#cdd6f4", &flavor.colors.text.hex.to_string())
                    .replace("#45475a", &flavor.colors.surface1.hex.to_string())
                    .replace("#cba6f7", &x.hex.to_string());
                file.write_all(svg.as_bytes()).unwrap();

                let mut cmd = Command::new("inkscape")
                    .arg(&file_name)
                    .arg("-b")
                    .arg(flavor.colors.crust.hex.to_string())
                    .arg("-y")
                    .arg("255")
                    .arg("-o")
                    .arg(file_name.replace(".svg", ".png").replace("svgs", "pngs"))
                    .spawn()
                    .unwrap();

                let _ = cmd.wait();
            }
        });
    }
}

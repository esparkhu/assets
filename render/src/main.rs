use std::env::current_dir;
use std::ffi::OsString;
use std::fs::{File, create_dir_all, read_dir};
use std::error::Error;
use std::path::PathBuf;
use tiny_skia::Pixmap;
use xmltree::Element;


const AWARD_COLOURS: [(&str, &str); 3] = [
    ("gold", "#c9b037"),
    ("silver", "#d7d7d7"),
    ("bronze", "#ad8a56")
];


fn load_xml(file: &PathBuf) -> Element {
    let f = File::open(file)
        .expect("Failed to open SVG file.");
    Element::parse(f)
        .expect("Failed to parse SVG file.")
}

fn change_colour(xml: &mut Element, colour: &str) {
    xml.attributes.insert("fill".to_string(), colour.to_string());
}

fn save_xml(xml: &Element, file: &PathBuf) -> Result<(), Box<dyn Error>> {
    create_dir_all(file.parent().unwrap())?;
    let f = File::create(file)?;
    xml.write(f)?;
    Ok(())
}

fn render_png(svg_file: &PathBuf, file: PathBuf) {
    let opt: usvg::Options = Default::default();
    let tree = usvg::Tree::from_file(svg_file, &opt)
        .expect("Failed to open created SVG file.");
    let mut pixmap = Pixmap::new(128, 128).unwrap();
    resvg::render(&tree, usvg::FitTo::Original, pixmap.as_mut());
    create_dir_all(file.parent().unwrap())
        .expect("Failed to create directory for PNG file.");
    pixmap.save_png(file)
        .expect("Failed to save PNG file.");
}

fn get_in_files(in_dir: PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    read_dir(in_dir)?
        .map(|entry| { Ok(entry?.path()) })
        .collect()
}

fn render_event_logo(
    base_dir: &PathBuf, in_file: &PathBuf, file_name: OsString
) {
    let mut svg_file_name = file_name.clone();
    svg_file_name.push(".svg");
    let mut png_file_name = file_name.clone();
    png_file_name.push(".png");
    let mut svg_base_dir = base_dir.clone();
    svg_base_dir.push("svg");
    let mut png_base_dir = base_dir.clone();
    png_base_dir.push("png");
    let mut xml = load_xml(in_file);
    for (colour_name, colour) in AWARD_COLOURS {
        let mut svg_out_file: PathBuf = svg_base_dir.clone();
        svg_out_file.push(colour_name);
        svg_out_file.push(&svg_file_name);
        let mut png_out_file: PathBuf = png_base_dir.clone();
        png_out_file.push(colour_name);
        png_out_file.push(&png_file_name);
        change_colour(&mut xml, colour);
        save_xml(&xml, &svg_out_file)
            .expect("Failed to write SVG file.");
        render_png(&svg_out_file, png_out_file);
    }
}

fn render_event_logos() {
    let mut base_dir = current_dir().unwrap();
    base_dir.push("event-logos");
    let mut source_dir = base_dir.clone();
    source_dir.push("source");
    let in_files = get_in_files(source_dir)
        .expect("Failed to open event logo directory");
    for in_file in in_files {
        match in_file.file_stem() {
            Some(file_name) => {
                render_event_logo(&base_dir, &in_file, file_name.to_owned());
            }
            None => { continue }
        };
    };
}

fn main() {
    render_event_logos();
}

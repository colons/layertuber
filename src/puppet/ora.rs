use serde::Deserialize;
use serde_xml_rs::de::from_str;
use std::io;
use std::io::{Read, Seek};
use zip::read::ZipArchive;

#[derive(Clone, Debug, Deserialize)]
pub struct Layer {
    pub name: String,
    pub src: String,
    pub x: i32,
    pub y: i32,

    #[serde(skip)]
    pub parent_names: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Stack {
    #[serde(rename = "$value")]
    entries: Vec<StackEntry>,
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum StackEntry {
    Layer(Layer),
    Stack(Stack),
}

#[derive(Debug, Deserialize)]
struct Image {
    #[serde(rename = "$value")]
    entries: Vec<StackEntry>,

    #[serde(rename = "w")]
    width: u32,

    #[serde(rename = "h")]
    height: u32,
}

fn layers_from_stack(parent_names: Vec<String>, entries: Vec<StackEntry>) -> Vec<Layer> {
    let mut layers: Vec<Layer> = Vec::new();

    for entry in entries {
        match entry {
            StackEntry::Layer(mut l) => {
                l.parent_names = parent_names.clone();
                layers.push(l)
            }
            StackEntry::Stack(s) => layers.extend_from_slice(&layers_from_stack(
                [parent_names.as_slice(), &[s.name]].concat(),
                s.entries,
            )),
        }
    }

    layers
}

pub fn read(ora: &mut ZipArchive<impl Read + Seek>) -> io::Result<(u32, u32, Vec<Layer>)> {
    let mut mimetype = String::new();
    ora.by_name("mimetype")?.read_to_string(&mut mimetype)?;

    if mimetype != "image/openraster" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "this file does not appear to be an OpenRaster file",
        ));
    }

    let mut stack_xml = String::new();
    ora.by_name("stack.xml")?.read_to_string(&mut stack_xml)?;
    let image: Image = from_str(&stack_xml).expect("this should probably be returned as an error");

    let mut layers = Vec::new();

    for layer in layers_from_stack(vec![], image.entries) {
        layers.push(layer);
    }

    layers.reverse();

    Ok((image.width, image.height, layers))
}

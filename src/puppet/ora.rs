use serde::Deserialize;
use serde_xml_rs::de::from_str;
use std::io;
use std::io::{Read, Seek};
use zip::read::ZipArchive;

#[derive(Debug, Deserialize)]
struct Layer {
    src: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum StackEntry {
    Layer(Layer),
    Stack(Stack),
}

#[derive(Debug, Deserialize)]
struct Stack {
    #[serde(rename = "$value")]
    entries: Vec<StackEntry>,
}

fn paths_from_stack(stack: Stack) -> Vec<String> {
    let mut paths = Vec::new();

    for entry in stack.entries {
        match entry {
            StackEntry::Layer(l) => paths.push(l.src),
            StackEntry::Stack(s) => paths.extend_from_slice(&paths_from_stack(s)),
        }
    }

    paths
}

pub fn layer_names(ora: &mut ZipArchive<impl Read + Seek>) -> io::Result<Vec<String>> {
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
    let stack: Stack = from_str(&stack_xml).expect("this should probably be returned as an error");

    let mut layer_names = Vec::new();

    for path in paths_from_stack(stack) {
        layer_names.push(path);
    }

    layer_names.reverse();

    Ok(layer_names)
}

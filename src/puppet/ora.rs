use serde_xml_rs::de::from_str;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use zip::read::ZipArchive;
use serde::Deserialize;

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

pub fn layer_names(ora_path: &Path) -> io::Result<Vec<String>> {
    let mut ora = ZipArchive::new(File::open(ora_path)?)?;

    let mut mimetype = String::new();
    ora.by_name("mimetype")?.read_to_string(&mut mimetype)?;

    if mimetype != "image/openraster" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "{} does not appear to be an OpenRaster file",
                ora_path.display()
            ),
        ));
    }

    let mut stack_xml = String::new();
    ora.by_name("stack.xml")?.read_to_string(&mut stack_xml)?;
    let stack: Stack = from_str(&stack_xml).expect("this should probably be returned as an error");
    dbg!(&stack);

    Ok(Vec::new())
}

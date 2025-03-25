use instant_xml::from_str;
use zip::ZipArchive;

use crate::core::model::Model;
use crate::io::error::Error;

use std::io::{self, Read};

/// Read all models from a 3MF reader
pub fn read<R: Read + io::Seek>(reader: R) -> Result<Vec<Model>, Error> {
    let mut zip = ZipArchive::new(reader)?;
    let mut models = Vec::new();

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        if file.name().ends_with(".model") {
            let mut buf: String = String::new();
            let _ = file.read_to_string(&mut buf)?;
            let model = from_str::<Model>(&buf)?;
            models.push(model);
        }
    }

    Ok(models)
}

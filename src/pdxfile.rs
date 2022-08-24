use anyhow::Result;
use std::fs::read_to_string;
use std::path::Path;

use crate::block::Block;
use crate::errorkey::ErrorKey;
use crate::errors::warn;
use crate::fileset::FileKind;
use crate::pdxfile::parse::parse_pdx;

mod parse;

pub struct PdxFile;

impl PdxFile {
    pub fn read_no_bom(pathname: &Path, kind: FileKind, fullpath: &Path) -> Result<Block> {
        let contents = read_to_string(fullpath)?;
        parse_pdx(pathname, kind, &contents)
    }

    pub fn read(pathname: &Path, kind: FileKind, fullpath: &Path) -> Result<Block> {
        let contents = read_to_string(fullpath)?;
        if let Some(bomless) = contents.strip_prefix('\u{feff}') {
            parse_pdx(pathname, kind, bomless)
        } else {
            warn(
                (pathname, kind),
                ErrorKey::Encoding,
                "file must start with a UTF-8 BOM",
            );
            parse_pdx(pathname, kind, &contents)
        }
    }
}

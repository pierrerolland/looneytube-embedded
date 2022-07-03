use crate::errors::Error;
use crate::helpers::base_dir::base_dir;
use serde::Serialize;
use slugify::slugify;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct Category {
    pub name: String,
    pub slug: String,
    pub picture: Option<String>,
}

impl Category {
    pub fn find_single(slug: String) -> Result<Category, crate::errors::Error> {
        match Category::find_all().into_iter().find(|c| c.slug == slug) {
            Some(c) => Ok(c),
            None => Err(Error::CategoryNotFound),
        }
    }

    pub fn find_all() -> Vec<Category> {
        let directories = fs::read_dir(base_dir()).unwrap();

        directories
            .into_iter()
            .map(|d| d.unwrap().into())
            .collect::<Vec<Category>>()
    }

    fn find_picture_filename(dir_name: &str) -> Option<String> {
        let picture_filename = format!("{}/{}/thumb.png", base_dir(), dir_name);

        match Path::new(&picture_filename).exists() {
            false => None,
            true => Some(format!("/d/{}/thumb.png", urlencoding::encode(dir_name))),
        }
    }
}

impl From<DirEntry> for Category {
    fn from(dir_entry: DirEntry) -> Self {
        let dir_name = String::from(dir_entry.file_name().to_str().unwrap());

        Category {
            name: String::from(&dir_name),
            slug: slugify!(&dir_name),
            picture: Category::find_picture_filename(&dir_name),
        }
    }
}

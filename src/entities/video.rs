use crate::helpers::base_dir::base_dir;
use crate::Category;
use serde::Serialize;
use slugify::slugify;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct Video {
    pub name: String,
    pub path: String,
    pub slug: String,
    pub full_slug: String,
    pub picture: Option<String>,
}

impl Video {
    pub fn find_all(category: &Category) -> Vec<Video> {
        let files = fs::read_dir(format!("{}/{}", base_dir(), &category.name)).unwrap();

        let mut videos = files
            .into_iter()
            .map(|d| d.unwrap())
            .filter(Video::is_video_file)
            .map(|d| Video::from_dir_entry(d, category))
            .collect::<Vec<Video>>();

        videos.sort_by(|a, b| a.name.cmp(&b.name));

        videos
    }

    fn find_picture_filename(filename: &str, category: &Category) -> Option<String> {
        let picture_filename = format!("{}/{}/thumbs/{}.jpg", base_dir(), &category.name, filename);

        match Path::new(&picture_filename).exists() {
            false => None,
            true => Some(format!(
                "/d/{}/thumbs/{}.jpg",
                urlencoding::encode(&category.name),
                urlencoding::encode(filename)
            )),
        }
    }

    fn is_video_file(dir_entry: &DirEntry) -> bool {
        match dir_entry.path().extension() {
            None => false,
            Some(ext) => ext == "mp4",
        }
    }

    fn from_dir_entry(dir_entry: DirEntry, category: &Category) -> Video {
        let filename = String::from(dir_entry.path().file_stem().unwrap().to_str().unwrap());

        Video {
            name: String::from(&filename),
            slug: slugify!(&filename),
            full_slug: format!("{}/{}", &category.slug, slugify!(&filename)),
            picture: Video::find_picture_filename(&filename, category),
            path: format!(
                "/d/{}/{}.mp4",
                urlencoding::encode(&category.name),
                urlencoding::encode(&filename)
            ),
        }
    }
}

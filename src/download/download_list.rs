use serde::Deserialize;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use sha1::Sha1;
use tokio::fs;
use std::path::{Path, PathBuf};
use super::{file_to_hash, DownloadError};

#[derive(Deserialize, Debug)]
pub struct File {
    pub url: String,
    pub size: u64,
    pub hash: String,
    pub path: String,
}

impl File {
    pub async fn download(&self, root: &str) -> Result<(), DownloadError> {
        let resp = reqwest::get(&self.url).await.map_err(DownloadError::Reqwest)?;
        let bytes = resp.bytes().await.map_err(DownloadError::Reqwest)?;
        let path = std::path::PathBuf::from(root).join(&self.path);
        self.create_folder(root).await?;
        let mut file = fs::File::create(path).await.map_err(DownloadError::Io)?;
        file.write_all(&bytes).await.map_err(DownloadError::Io)?;
        Ok(())
    }

    pub async fn verify(&self, root: &str) -> Result<bool, DownloadError> {
        let path = std::path::PathBuf::from(root).join(&self.path);
        let mut file = fs::File::open(path).await.map_err(DownloadError::Io)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).await.map_err(DownloadError::Io)?;
        let hash = self.file_hash(root)?;
        Ok(hash == self.hash)
    }

    fn file_hash(&self, root: &str) -> Result<String, DownloadError> {
        let path = self.fullpath(root);
        let hash = file_to_hash::<Sha1, _>(path)?;
        Ok(hex::encode(&hash))
    }

    //create folder if doesn't exists
    async fn create_folder(&self, root: &str) -> Result<(), DownloadError> {
        let path = self.fullpath(root);
        let parent = match path.parent() {
            Some(p) => p,
            None => return Ok(()),
        };
        if !parent.exists() {
            fs::create_dir_all(parent).await.map_err(DownloadError::Io)?;
        }
        Ok(())
    }

    fn fullpath(&self, root: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(root).join(&self.path)
    }
}
#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct Files{
    pub files: Vec<File>,
}
// type Files = Vec<File>;

impl Files {
    
    pub async fn from_url(url: &str) -> Result<Files, reqwest::Error> {
        Self::from_response(reqwest::get(url).await?).await
    }
    pub async fn from_response(resp: reqwest::Response) -> Result<Files, reqwest::Error> {
        Ok(resp.json::<Files>().await?)
    }
    pub fn iter(&self) -> std::slice::Iter<File> {
        self.files.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<File> {
        self.files.iter_mut()
    }
    pub fn into_iter(self) -> std::vec::IntoIter<File> {
        self.files.into_iter()
    }
    fn make_list_files(parent: &Path) -> Vec<PathBuf> {
        let mut v = Vec::new();
        if !parent.exists() {
            return v;
        }
        for entry in std::fs::read_dir(parent).unwrap() {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    println!("Error while reading dir {}: {}", parent.to_str().unwrap_or("{unwknown}"), e);
                    continue;
                },
            };
            let path = entry.path();
            if path.is_dir() {
                let new_path = parent.join(path.file_name().expect("Error while getting directory name"));
                v.extend(Self::make_list_files(&new_path));
            } else {
                v.push(path);
            }
        }
        v
    }
    pub fn remove_old_files<P: AsRef<Path>>(&self, root: P) {
        let files_distant = {
            let mut v = self.files.iter().map(|f| root.as_ref().join(PathBuf::from(f.path.as_str()))).collect::<Vec<_>>();
            v.sort();
            v
        };
        let files_local = {
            let mut v = Self::make_list_files(root.as_ref());
            v.sort();
            v
        };
        println!("files_distant: {:#?}", files_distant);
        println!("files_local: {:#?}", files_local);
        // return;
        let (mut file_distant_iter, mut file_local_iter) = (files_distant.into_iter(), files_local.into_iter());
        let (mut file_distant, mut file_local) = (file_distant_iter.next(), file_local_iter.next());
        loop {
            match (&mut file_distant, &mut file_local) {
                (Some(d), Some(l)) => {
                    if d != l {
                        println!("Remove file: {}", l.to_str().unwrap_or("{unknown}"));
                        std::fs::remove_file(l).unwrap();
                    } else {
                        file_distant = file_distant_iter.next();
                    }
                    file_local = file_local_iter.next();
                }
                (None, Some(l)) => {
                    println!("Remove file: {}", l.to_str().unwrap_or("{unknown}"));
                    std::fs::remove_file(l).unwrap();
                    file_local = file_local_iter.next();
                }
                _ => break,
            }
        }
    }
}

impl From<Files> for Vec<File> {
    fn from(f: Files) -> Self {
        f.files
    }
}




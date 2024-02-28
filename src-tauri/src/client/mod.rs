use std::{fs, io::Write, path::Path};

use reqwest::Response;

use crate::{
    error::Result,
    model::{Course, File, Folder, ProgressPayload},
};
const BASE_URL: &'static str = "https://oc.sjtu.edu.cn";

pub struct Client {
    cli: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            cli: reqwest::Client::new(),
        }
    }

    async fn get_request(
        &self,
        url: &str,
        query: Option<&Vec<(String, String)>>,
        token: &str,
    ) -> Result<Response> {
        let mut req = self
            .cli
            .get(url)
            .header("Authorization".to_owned(), format!("Bearer {}", token));

        if let Some(query) = query {
            req = req.query(query)
        }

        let res = req.send().await?;
        Ok(res)
    }

    pub async fn list_courses(&self, token: &str) -> Result<Vec<Course>> {
        let url = format!("{}/api/v1/courses", BASE_URL);
        let res = self.get_request(&url, None, token).await?;

        let courses = serde_json::from_slice(&res.bytes().await?)?;
        Ok(courses)
    }

    pub async fn download_file<F: Fn(ProgressPayload) + Send>(
        &self,
        file: &File,
        token: &str,
        save_path: &str,
        progress_handler: F,
    ) -> Result<()> {
        let mut response = self
            .get_request(&file.url, None, token)
            .await?
            .error_for_status()?;

        let mut payload = ProgressPayload {
            uuid: file.uuid.clone(),
            downloaded: 0,
            total: file.size,
        };
        let path = Path::new(save_path).join(&file.display_name);
        let mut file = fs::File::create(path.to_str().unwrap())?;
        while let Some(chunk) = response.chunk().await? {
            payload.downloaded += chunk.len() as u64;
            progress_handler(payload.clone());
            file.write_all(&chunk)?;
        }

        tracing::info!("File downloaded successfully!");
        Ok(())
    }

    pub async fn list_files_with_page(
        &self,
        url: &str,
        token: &str,
        page: u64,
    ) -> Result<Vec<File>> {
        let res = self
            .get_request(
                &url,
                Some(&vec![("page".to_owned(), page.to_string())]),
                token,
            )
            .await?
            .error_for_status()?;

        let files = serde_json::from_slice(&res.bytes().await?)?;
        Ok(files)
    }

    pub async fn list_files(&self, url: &str, token: &str) -> Result<Vec<File>> {
        let mut all_files = vec![];
        let mut page = 1;

        loop {
            let files = self.list_files_with_page(&url, token, page).await?;
            if files.is_empty() {
                break;
            }
            page += 1;
            all_files.extend(files);
        }
        Ok(all_files)
    }

    pub async fn list_course_files(&self, course_id: i32, token: &str) -> Result<Vec<File>> {
        let url = format!("{}/api/v1/courses/{}/files", BASE_URL, course_id);
        self.list_files(&url, token).await
    }

    pub async fn list_folder_files(&self, folder_id: i32, token: &str) -> Result<Vec<File>> {
        let url = format!("{}/api/v1/folders/{}/files", BASE_URL, folder_id);
        self.list_files(&url, token).await
    }

    pub async fn list_folders_with_page(
        &self,
        course_id: i32,
        token: &str,
        page: u64,
    ) -> Result<Vec<Folder>> {
        let url = format!("{}/api/v1/courses/{}/folders", BASE_URL, course_id);
        let res = self
            .get_request(
                &url,
                Some(&vec![("page".to_owned(), page.to_string())]),
                token,
            )
            .await?
            .error_for_status()?;

        let files = serde_json::from_slice(&res.bytes().await?)?;
        Ok(files)
    }

    pub async fn list_folders(&self, course_id: i32, token: &str) -> Result<Vec<Folder>> {
        let mut all_folders = vec![];
        let mut page = 1;
        loop {
            let folders = self.list_folders_with_page(course_id, token, page).await?;
            if folders.is_empty() {
                break;
            }
            page += 1;
            all_folders.extend(folders);
        }
        Ok(all_folders)
    }
}

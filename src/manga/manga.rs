pub struct Manga {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub description: String,
    pub genres: Vec<String>,
    pub chapters: Vec<Chapter>,
    pub url: String,
    pub cover_image: String,
}

impl Manga {
    pub fn new(
        id: String,
        title: String,
        authors: Vec<String>,
        genres: Vec<String>,
        url: String,
    ) -> Self {
    }
}

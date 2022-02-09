use std::path::{Path, PathBuf};
use std::time::Duration;
use serde::{Serialize, Deserialize};


// Structure de données pour le stockage des métadonnées d'un fichier mp3
// Nous avons fait le choix de stocker ces données que nous trouvons pertinentes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicFile {
   pub path: PathBuf,
   pub file_size: u64,
   pub title: String,
   pub author: String,
   pub duration: Duration,
   pub album: String,
   pub year: u16,
   pub genre: String,
}


// Constructeur de la structure
impl MusicFile {
    pub fn new(path: &Path) -> MusicFile {
        // Initialisation des champs avec des valeurs 
        let music_file: MusicFile = MusicFile {
            path: path.to_path_buf(),
            file_size: 0,
            title: String::new(),
            author: String::new(),
            duration: Duration::new(0, 0),
            album: String::new(),
            year:0,
            genre: String::new(),
        };
        music_file
    }

    // Getter du champ path
    pub fn get_file_path(&self) -> PathBuf {
        self.path.to_path_buf()
    }
}

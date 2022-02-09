use std::path::Path;
use std::fs::metadata;
use mp3_metadata::Genre;
use walkdir::{DirEntry, WalkDir};
use crate::musicfile::MusicFile;


const SUPPORTED_EXTENSIONS: [&str; 1] = ["mp3"];

fn is_supported(entry: &DirEntry) -> bool {
    entry.path().is_file() &&
    SUPPORTED_EXTENSIONS.contains(&entry.path().extension().unwrap().to_str().unwrap())
}

pub fn scan(path: &Path) -> Vec<MusicFile> {
    let mut music_files: Vec<MusicFile> = Vec::new();
    let walker = WalkDir::new(path).into_iter();
    for entry in walker {
        match entry {
            Ok(entry) => {
                if is_supported(&entry) {
                    // Initialisation de la structure pour chanque fichier de musique
                    music_files.push(MusicFile::new(entry.path()));
                }
            }
            // Gestion d'erreur
            Err(e) => panic!("Error while scaning music file : {:?}\n{}", e.path(), e),
        };
    }
    // Remplissage des métadonnées
    music_files = fill_files_metadata(music_files);
    music_files 
}


// Clone la liste des médias en  les rajoutant les Tags nécessaires pour les fichiers mp3
pub fn fill_files_metadata(music_files: Vec<MusicFile>) -> Vec<MusicFile> {
    let mut result: Vec<MusicFile> = Vec::new(); // Nouveau vecteur résultat
    music_files.into_iter().for_each(|mut music_file| { // Parcourt de l'ancien vecteur
        // Assignation de la taille du fichier
        match metadata(music_file.get_file_path()) {
            Ok(meta) => music_file.file_size = meta.len(),
            Err(e) => panic!("Error when collecting music files metadata : {}", e)
        }

        // Récupération puis assignation des matadonnées mp3
        match mp3_metadata::read_from_file(music_file.get_file_path()) {

            Ok(mp3_metadata) => {

                music_file.duration = mp3_metadata.duration;
                
                if let Some(audio_tag) = mp3_metadata.tag {	
                    music_file.author = audio_tag.artist.replace(" ", "_").trim_matches(char::from(0)).to_string();
                    music_file.title = audio_tag.title.replace(" ", "_").trim_matches(char::from(0)).to_string();
                    music_file.album = audio_tag.album.replace(" ", "_").trim_matches(char::from(0)).to_string();
                    music_file.year = audio_tag.year;
                    music_file.genre = get_media_genre(audio_tag.genre);
                }
            },

            Err(e) => panic!("Error when collecting music files metadata : {}", e)
        }

        result.push(music_file)
    });
    result
}


// Prend en entrée un genre musical et renvoie le genre supporté sous forme de caractère
// Si le genre n'est pas supporté, renvoie "Unknown"
pub fn get_media_genre(genre: Genre) -> String {
    match genre {
        Genre::Blues => "Blues".to_string(),
        Genre::Country => "Country".to_string(),
        Genre::Disco => "Disco".to_string(),
        Genre::HipHop => "HipHop".to_string(),
        Genre::Jazz => "Jazz".to_string(),
        Genre::Metal => "Metal".to_string(),
        Genre::NewAge => "NewAge".to_string(),
        Genre::Oldies => "Oldies".to_string(),
        Genre::Pop => "Pop".to_string(),
        Genre::RAndB => "RAndB".to_string(),
        Genre::Rap => "Rap".to_string(),
        Genre::Reggae => "Reggae".to_string(),
        Genre::Rock => "Rock".to_string(),
        Genre::DeathMetal => "DeathMetal".to_string(),
        Genre::Classical => "Classical".to_string(),
        Genre::Instrumental => "Instrumental".to_string(),
        Genre::Soul => "Soul".to_string(),
        Genre::Punk => "Punk".to_string(),
        Genre::Electronic => "Electronic".to_string(),
        Genre::Opera => "Opera".to_string(),
        Genre::Symphony => "Symphony".to_string(),
        Genre::Samba => "Samba".to_string(),
        Genre::ACapela => "ACapela".to_string(),
        Genre::DanceHall => "DanceHall".to_string(),
        _ => "Unknown".to_string(),
    }
}
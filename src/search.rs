use crate::musicfile::MusicFile;
extern crate parse_duration;
use parse_duration::parse;

// Représente les champs possibles dans une requête de recherche sur les données gérées
#[derive(Debug, Clone)]
pub enum SearchField {
    Path,
    Size,
    Title,
    Author,
    Duration, 
    Album,
    Year,
    Genre,
    Undefined
}

// Prend en entrée une chaîne de caractère qui correpond au champ de la recherche et renvoie
// l'objet SearchType correspondant
pub fn str_to_search_field(str_field: &str) -> SearchField {
    match str_field {
        "path" => SearchField::Path,
        "size" => SearchField::Size,
        "title" => SearchField::Title,
        "author" => SearchField::Author,
        "duration" => SearchField::Duration,
        "album" => SearchField::Album,
        "year" => SearchField::Year,
        "genre" => SearchField::Genre,
        _ => SearchField::Undefined
    }
}


// Prend en entrée une chaine de caractères qui correspond à la requête de la recherche 
// et renvoie une liste de paire (clé de la requête, valeur), correspondant aux sous requêtes.
// Le format d'une recherche doit être le suivant : champ1:valeur1 champ2:valeur2 ...
// La valeur à rechercher doit être séparer du champ par ":" sans espace. Chaque sous requête doit
// cependant être séparé du suivant par un espace.
// Exemple : title:Titre1 author:Auteur1 duration:20 genre:champ avec espaces
pub fn parse_request(request: &str) -> Vec<(SearchField, String)> {
    let mut result: Vec<(SearchField, String)> = Vec::new();
    // On split la chaine de caractère avec des espaces pour récupérer les sous requêtes dans un tableau
    let subrequests: Vec<&str> = request.split_whitespace().collect(); 
    // Pour chaque sous requête
    for subrequest in subrequests {
      // On récupère le champ et la valeur (split de la chaine avec ":"")
        let fields: Vec<&str> = subrequest.split(":").collect();
        // Si la taille du vecteur récupéré est < 2 i.e le champ ou la valeur de la requête est manquante
        // Alors il y a erreur, on ignore simplement cette sous requête et on passe à la suivante 
        // (en affichant un message d'erreur)
        if fields.len() < 2 {
          // Affichage du message d'erreur sur la sortie standard d'erreur
          eprintln!("Err : Incorrect format of request : Subrequest is : {}, The full request is : {}", subrequest, request)
        } else {
          // Sinon, on converti le champ en SearchField et on ajoute la paire à la liste résultat
            result.push((str_to_search_field(fields[0]), fields[1].to_string()))
        }
    }
    result
}

// Prend en entrée la liste des médias et le vecteur contenant les champs de la recherche
// et les valeurs associées, et renvoie les médias correspondant à la recherche
pub fn search(music_files: Vec<MusicFile>, vec_req: Vec<(SearchField, String)>) -> Vec<MusicFile> {
  // Liste des médias résultat
  let mut result_files: Vec<MusicFile> = Vec::new();
  // Pour chaque média,
  music_files.into_iter().for_each(|music_file| { 

    // Parcoure les sous requêtes de la recherche
    vec_req.clone().into_iter().for_each(|subrequest| { 
      // Vérifie le champ de la recherche
      match subrequest.0 {
        // Si le champ est le chemin
        SearchField::Path => 
          // Conversion du Path en String puis comparaison à la valeur
          // (on peut utiliser unwrap ici car into_string empaque <String, OsString>, unwrap renvoie la partie String)
          if music_file.get_file_path().into_os_string().into_string().unwrap() == subrequest.1 {
            // Si la valeur correspond au chemin du média
            // Alors on ajoute le média à la liste résultat
            result_files.push(music_file.clone());
          },

        // Si le champ est la taille
        SearchField::Size =>
          // On converti la taille spécifée en entier
          match subrequest.1.parse::<u64>() {
            // Si la conversion réussi et que la valeur correspond à la taille du média,
            // alors on ajoute le média à la liste résultat
            Ok (val) => if music_file.file_size == val { result_files.push(music_file.clone()) },
            // Sinon, on affiche un message d'erreur et on ignore la sous requête
            Err(_) => eprintln!("Mauvais format de la sous requête : {:?} : {} n'est pas un entier positif", subrequest, subrequest.1),
          },

        // Si le champ est le titre et que la valeur correspond au titre du média
        SearchField::Title =>
          if music_file.title == subrequest.1 {
            // Alors on ajoute le média à la liste résultat
            result_files.push(music_file.clone());
          },        

        // Si le champ est l'auteur et que la valeur correspond à l'auteur du média
        SearchField::Author =>
          if music_file.author == subrequest.1 {
            // Alors on ajoute le média à la liste résultat
            result_files.push(music_file.clone());
          },

        // Si le champ est la durée
        SearchField::Duration =>
          // On converti la durée spécifée en Duration
          match parse(subrequest.1.clone().as_mut_str()) {
            // Si la conversion réussi et que la valeur correspond à la durée du média,
            // alors on ajoute le média à la liste résultat
            Ok (d) => if music_file.duration == d { result_files.push(music_file.clone()) },
            // Sinon, on affiche un message d'erreur et on ignore la sous requête
            Err(_) => eprintln!("Mauvais format de la sous requête : {:?}, {:?} durée incorrecte", subrequest, subrequest.1),
          },
     
        // Si le champ est l'album et que la valeur correspond à l'album du média
        SearchField::Album =>
          if music_file.album == subrequest.1 {
            // Alors on ajoute le média à la liste résultat
            result_files.push(music_file.clone());
          },

        // Si le champ est l'année
        SearchField::Year =>
          // On converti l'année spécifée en entier
          match subrequest.1.parse::<u16>() {
            // Si la conversion réussi et que la valeur correspond à l'année du média,
            // alors on ajoute le média à la liste résultat
            Ok (val) => if music_file.year == val { result_files.push(music_file.clone()) },
            // Sinon, on affiche un message d'erreur et on ignore la sous requête
            Err(_) => eprintln!("Mauvais format de la sous requête : {:?}, {} n'est pas un entier", subrequest, subrequest.1),
          },

        // Si le champ est le genre et que la valeur correspond au genre  du média
        SearchField::Genre =>
        if music_file.genre == subrequest.1 {
          // Alors on ajoute le média à la liste résultat
          result_files.push(music_file.clone());
        },


        // Si le champ est indéfini, alors la sous requête n'est pas valide. On affiche un message d'erreur 
        // et on ignore la sous requête
        SearchField::Undefined => eprintln!("Mauvais format de la sous requête : {:?}", subrequest),
        }
      });
  });

  result_files
}
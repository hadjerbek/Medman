use medman::cli::CliArguments;
use medman::musicfile::MusicFile;
use medman::scan::scan;
use medman::search::{parse_request, search};
use std::env;
use std::io;
use std::path::Path;
use std::fs::File;
use markdown_gen::markdown::{Markdown, AsMarkdown};


// Message d'aide pour l'utilisation du programme
fn help() {
    println!("\n-- MEDMAN : GESTION DE COLLECTION DE FICHIERS MULTIMEDIA --");
    println!();
    println!("DESCRIPTION");
    println!("Ce programme propose une double interface utilisateur :");
    println!("* Un mode en ligne de commande : les arguments sont passés en paramètre de l’exécutable.");
    println!("* Un mode interactif : si le programme est lancé sans argument, ce mode vous est proposé.");
    println!();
    println!();

    println!("MODE LIGNE DE COMMANDE");
    println!("MODE D'EMPLOI :");
    println!("medman <command> <arguments>");
    println!();
    println!("COMMANDES ET ARGUMENTS :");
    println!("    scan <path>                       Analyse récursivement le répertoire ayant pour chemin 'path' afin de collecter les fichiers supportés (l’analyse extrait les métadonnées du fichier)");
    println!("    search \"<path> champ1:valeur1      Effectue une recherche sur les données gérées dans les médias de 'path'. Le format de la requete est inspirée d’une partie de la syntaxe");
    println!("           champ2:valeur2 ...\"         de l'outil de recherche par mots-clé Apache Lucene. Les sous requêtes sont séparées par des espace.");
    println!("                                      Toutefois, les champs et valeurs des sous requetes NE DOIVENT PAS COMPORTER D'ESPACE. REMPLACER LES ESPACES PAR DES UNDESCORE (_)");
    println!("                                      L'ensemble des champs de recherche possible sont : path, size, title, author, duration, album, year, genre.");
    println!("    write2md <path> \"<filename>        Génère un fichier Markdown contenant le résultat de la dernière requête éffectuée.");
    println!("             search champ1:valeur1 ...\"");
    println!();
    println!("EXEMPLES");
    println!("    scan      ->   scan /tmp/music_files/");
    println!("    search    ->   search \"/tmp/music_files/ title:MyFavMusic duration:2min45s\"");
    println!("    write2md  ->   write2md /tmp/music_files/ \"my_research_result.md search title:MyFavMusic duration:2min45s\"");

    println!();
    println!();

    println!("MODE INTERACTIF");
    println!("COMMANDES");
    println!("    scan <path>                   Analyse récursivement le répertoire ayant pour chemin 'path' afin de collecter les fichiers supportés (l’analyse extrait les métadonnées du fichier)");
    println!("    search champ1:valeur1         Effectue une recherche sur les données gérées. Le format de la requete est inspirée d’une partie de la syntaxe de l'outil de recherche par mots-clé Apache Lucene.");
    println!("           champ2:valeur2         Chaque sous requête doit donc être séparée d'un espace.");
    println!("           ...                    L'ensemble des champs de recherche possible sont : path, size, title, author, duration, album, year, genre.");
    println!("    write2md <filename>           Génère un fichier Markdown contenant le résultat de la dernière requête éffectuée.");
    println!("    help                          Affiche le message d'aide.");
    println!("    quit                          Met fin au programmme.");
    println!();
    println!("EXEMPLES");
    println!("    scan      ->   scan /tmp/music_files/");
    println!("    search    ->   search path:/tmp/music_files/music1.mp3 title:MyFavMusic duration:2min45s");
    println!("    write2md  ->   write2md my_research_result.md");
    println!();

}


// Affiche un message d'erreur puis l'aide
pub fn err_help() {
    eprintln!("Commande non supportée. Les commandes supportées sont : scan, search et write2md");
    eprintln!();
    help();
    panic!("ERREUR");

}


// Génère un fichier Markdown contenant le résultat d’une requête
fn to_markdown(results: Vec<MusicFile>, file_path: &str, request:&str) {
    // Création du fichier résultat
    let file = File::create(file_path).unwrap();
    let mut md = Markdown::new(file);

    md.write("RESULTS OF YOUR REQUESTS".heading(1)).unwrap();
    md.write("Summary:".heading(2)).unwrap();
    md.write("Request:".bold()).unwrap();
    md.write(request.code()).unwrap();
    md.write(format!("Number of results: {}", results.len()).bold()).unwrap();

    md.write("Results:".heading(2)).unwrap();
    
    // Ecrit chaque élément du résultat dans le fichier
    for result in results {
        match serde_json::to_string_pretty(&result) {
            Ok(js) => md.write(js.as_str().quote()).unwrap(),
            Err(e) => panic!("Could'nt write music file in md format : {}", e),
        }
    }
    println!("La requête a été exportée avec succès vers {}.", file_path);

}


fn main() {

    let cli_args: Vec<String> = env::args().collect();

    if cli_args.len() > 1 { // Mode ligne de commande
        // Récupération des arguments
        let args = CliArguments::new();
        println!();
        match args.get_command().as_str() {
            "scan" => // La commande à exécuter est le scan
            {
                // Scan du répertoire
                let music_files = scan(args.path());
                // Affichange des médias scannés
                println!("Fichiers scannés :\n");
                for music_file in music_files.clone() {
                    println!("{:?}", music_file);
                }        
            },

            "search" => // La commande à exécuter est le search
            {
                // Scan du répertoire
                let music_files = scan(args.path());
                // Conversion des arguments (qui représentent la requête)
                match args.get_arguments() {
                    Some(string_args) => 
                    {
                        let vec_req = parse_request(string_args.as_str());
                        // Recherche dans la liste des fichiers scannés
                        let req_results = search(music_files.clone(), vec_req);
                        // Affichage du résultat
                        println!("Résultats de votre requête : ");
                        for mf in req_results.clone() {
                            println!("{:#?}", mf);
                        }        
                    },
                    None => err_help(),
                }
            },

            "write2md" => // La commande à exécuter est le write2md
            {
                // Scan du répertoire
                let music_files = scan(args.path());
                match args.get_arguments() {
                    Some(string_args) => 
                    {
                        // Récupération des différents champs de la commande (notamment search et ses arguments)
                        let mut fields = string_args.splitn(3, " ");
                        let file_name = fields.next();
                        let search_key = fields.next();
                        let search_args = fields.next();
                        match (file_name, search_key, search_args) {
                            (Some(file_path), Some("search"), Some(search_args)) =>
                            {
                                // Conversion des arguments de search en requête
                                let vec_req = parse_request(search_args);
                                // Recherche dans la liste des fichiers scannés
                                let req_results = search(music_files.clone(), vec_req);
                                // Génération du résultat au format markdown
                                to_markdown(req_results, file_path, ("search ".to_string()+search_args).as_str());

                            },
                            _ => err_help(),
                        }
                    },
                    None => err_help(),
                }

            },

            _ => err_help(),
        }


    } else { // Appel du programme sans argument : Mode interactif
        help();
        println!();

        let mut scanned_files : Vec<MusicFile> = Vec::new(); // Fichiers scannés
        let mut prec_request: String = String::new(); // Requête précédente
        let mut req_results: Vec<MusicFile> = Vec::new(); // Résultats d'une requête

        loop {
            // Lecture de la commande
            println!();
            println!();
            println!("Entrez la commande à exécuter:");
            let mut buffer = String::new(); // Entrée standard de l'utilisateur
            match io::stdin().read_line(&mut buffer) {
                Ok(_) => 
                {
                    // Récupération de la commande et de l'argument de la requête
                    let mut fields = buffer.trim().splitn(2, " ");
                    let cmd = fields.next();
                    let args = fields.next();
                    // Si la commande saisie est scan
                    if cmd == Some("scan") {
                        // Désempaque l'argument et Scanne le répertoire passé en paramètre
                        match args {
                            Some(args) => {
                                scanned_files = scan(Path::new(args));
                                println!("Fichiers scannés :\n");
                                for music_file in scanned_files.clone() {
                                    println!("{:#?}", music_file);
                                }
                            },

                            None => eprintln!("Requête incorrect !"),
                        }
    
                    } else if cmd == Some("search")  { // Sinon s'il s'agit de search, lance la recherche
                        match scanned_files.clone().len() {
                            0 => eprintln!("Aucun répertoire scanné. Vous devez scanner un répertoire au préalable."),
                            _ => {
                                match args {
                                    Some(args) => {
                                        prec_request = buffer.clone();
                                        let vec_req = parse_request(args);
                                        req_results = search(scanned_files.clone(), vec_req);
                                        println!("Résultats de votre requête : ");
                                        for mf in req_results.clone() {
                                            println!("{:#?}", mf);
                                        }
                                    },
                                    None => eprintln!("Requête incorrect !"),
                                }
                            },
                        }
                    } else if cmd == Some("write2md") { // Write2md
                        // Vérifie si une recherche a déjà été effectuée
                        match req_results.len() {
                            0 => eprintln!("Aucune requête à extraire sous forme md. Veuillez exécuter une requête au préalable"),
                            _ => match args {
                                    Some(args) => to_markdown(req_results.clone(), args, &prec_request),
                                    None => eprintln!("Requête incorrect !"),
                            }
                        }

                    } else if cmd == Some("help") { // Message d'aide
                        help();

                    } else if cmd == Some("quit") { // Met fin au programme
                        break;
                    }
                    else {
                        eprintln!("Mot clé incorrect/non pris en charge.");
                    }
                },
                Err(_) => eprintln!("Mauvaise sasie !"),
            }
   
        }
    } 

    println!();
    println!();
}

use structopt::StructOpt;

/// Représente les arguments en paramètres de ligne de commande
#[derive(Debug)]
#[derive(StructOpt)]
pub struct CliArguments {
    /// Commande à exécuter
    command: String,

    /// Chemin où trouver les fichiers à analyser
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    /// Le reste des arguments (à passer aux commandes search et write2md).
    /// Pour plus d'info sur le mode d'emploi, exécutez le programme sans argument
    arguments: Option<String>,
}

impl CliArguments {
    pub fn new() -> CliArguments {
        CliArguments::from_args()
    }

    pub fn path(&self) -> &std::path::Path {
        self.path.as_path()
    }

    pub fn get_command(&self) -> String
    {
        self.command.clone()
    }

    pub fn get_arguments(&self) -> Option<String>
    {
        self.arguments.clone()
    }

}

use std::io;

use anyhow::Result;
use clap::{CommandFactory, Parser};

use crate::cli::SeaplaneArgs;
use crate::Ctx;

// @TODO @SIZE this str is ~3.5kb, it can be stored compressed at around 1.4kb. However that would
// require a code to do the compression/decompression which is larger than the 2.1kb savings. There
// are other locations in the code may benefit as well; if the uncompressed sum of those becomes
// greater than code required to do the compression, we may look at compressing these large strings
// to keep the binary size minimal.
static COMPLETION_HELP: &str = "DISCUSSION:
    Enabling shell completion scripts depends on the shell you're using, the 
    operating system (or distribution of operating system), or even your 
    individual setup. Consult your shell or operating system documentation for 
    full details. 

    This guide covers several common setups.

    BASH:

    Completion scripts are often stored either system wide in 
    `/etc/bash_completion.d/` or individually by user in 
    `$HOME/.local/share/bash-completion/completions/`. In either case, the file 
    is typically the name of the binary who we are trying to complete, in our 
    case that is `seaplane`.

    e.g. to configure completions for just our user:

      $ mkdir -p ~/.local/share/bash-completion/completions/
      $ seaplane shell-completion bash > ~/.local/share/bash-completion/completions/seaplane
    
    Alternative, it is common to eval the completions during shell startup. To 
    do so, one needs only place the following command in their `.bashrc` or 
    similar:
    
      eval \"$(seaplane shell-completion bash)\"
    
    Whichever method you choose, you may need to close and re-open your terminal 
    for the changes to take affect.

    ZSH:

    ZSH completions are commonly stored in the directories pointed to by your 
    `$fpath` variable. To use the completions generated by seaplane you must 
    either add the completion script to one of the existing directories, or add 
    your custom directory to the `$fpath` list. A common directory to either 
    create, or use if it exists is `~/.zfunc`

      $ mkdir -p ~/.zfunc

    Then in your `.zshrc` file, either add:

      compinit
      fpath+=~/.zfunc

    Note, if your `.zshrc` file already had a line `compinit`, just ensure the 
    `fpath+=~/.zfunc` comes afterwards.

    ZSH looks for files beginning with an underscore and the name of the binary 
    to complete, in our case that would be `_seaplane`:

      $ seaplane shell-completion zsh > ~/.zfunc/_seaplane

    Like BASH, you could alternatively use an `eval` command in your `.zshrc`:

      eval \"$(seaplane shell-completion zsh)\"

    Ensure you close and open your terminal to utilize the completions.

    FISH:

    Completion scripts are commonly stored in `$HOME/.config/fish/completions/` 
    using the file name of the binary to complete with a `.fish` extension:

      $ seaplane shell-completion fish > ~/.config/fish/completions/seaplane.fish

    Ensure you close and open your terminal to utilize the completions.

    POWERSHELL:

    These completion scripts require PowerShell v5.0 or newer. Windows 10 and 11 
    already have a new enough version, but on Windows 7 you will need to download
    and update it manually which is out of scope for this guide.

    The completions are loaded from a 'profile.' You check if a profile already exists using the command:
    
      PS C:\\> Test-Path $profile

    If this returns `False`, you must first create a profile:

      PS C:\\> New-Item -path $profile -type file -force

    This creates a file at 
    `${env:USERPROFILE}\\Documents\\WindowsPowerShell\\Microsoft.PowerShell_profile.ps1`.

    Inside this profile file, we can either place the completion script inline, 
    or `source` a separate file (our completion script). This guide will demo 
    placing the completion script inline:
    
      PS C:\\> seaplane shell-completion powershell \\
        >> ${env:USERPROFILE}\\Documents\\WindowsPowerShell\\Microsoft.PowerShell_profile.ps1
";

/// Generate shell completion script files for seaplane
#[derive(Parser)]
#[clap(after_help = COMPLETION_HELP)]
pub struct SeaplaneShellCompletionArgs {
    /// The shell to generate completion scripts for
    #[clap(arg_enum)]
    shell: clap_complete::Shell,
}

impl SeaplaneShellCompletionArgs {
    pub fn run(&self, _ctx: &mut Ctx) -> Result<()> {
        let mut app = SeaplaneArgs::command();

        clap_complete::generate(self.shell, &mut app, "seaplane", &mut io::stdout());

        Ok(())
    }
}

Short help:

```console
$ seaplane restrict list -h
seaplane-restrict-list [..]
List restrictions in an API, or across all APIs

USAGE:
    seaplane restrict list [API] [OPTIONS]

ARGS:
    <API>    The API to list the restrictions from

OPTIONS:
    -A, --api-key <STRING>    The API key associated with a Seaplane account used to access Seaplane API endpoints [env: SEAPLANE_API_KEY]
    -B, --base64              The directory is already encoded in URL safe Base64
        --color <COLOR>       Should the output include color? [default: auto] [possible values: always, ansi, auto, never]
    -D, --decode              Decode the directories before printing them
        --format <FORMAT>     Change the output format [default: table] [possible values: table, json]
    -h, --help                Print help information
    -H, --no-header           Omit the header when printing with `--format=table` [aliases: no-heading, no-headers]
        --no-color            Do not color output (alias for --color=never)
        --no-decode           Print directories without decoding them
    -q, --quiet               Suppress output at a specific level and below
    -S, --stateless           Ignore local state files, do not read from or write to them
    -v, --verbose             Display more verbose output
    -V, --version             Print version information

```

Long help:

```console
$ seaplane restrict list --help
seaplane-restrict-list [..]
List restrictions in an API, or across all APIs

Directory will be displayed in base64 encoded format by default because they may contain
arbitrary binary data. Use --decode to output the decoded values instead.

USAGE:
    seaplane restrict list [API] [OPTIONS]

ARGS:
    <API>
            The API to list the restrictions from

OPTIONS:
    -A, --api-key <STRING>
            The API key associated with a Seaplane account used to access Seaplane API endpoints
            
            The value provided here will override any provided in any configuration files.
            A CLI provided value also overrides any environment variables.
            One can use a special value of '-' to signal the value should be read from STDIN.
            
            [env: SEAPLANE_API_KEY]

    -B, --base64
            The directory is already encoded in URL safe Base64

        --color <COLOR>
            Should the output include color?
            
            [default: auto]
            [possible values: always, ansi, auto, never]

    -D, --decode
            Decode the directories before printing them
            
            Binary values will be written directly to standard output (which may do strange
            things to your terminal)

        --format <FORMAT>
            Change the output format
            
            [default: table]
            [possible values: table, json]

    -h, --help
            Print help information

    -H, --no-header
            Omit the header when printing with `--format=table`
            
            [aliases: no-heading, no-headers]

        --no-color
            Do not color output (alias for --color=never)

        --no-decode
            Print directories without decoding them

    -q, --quiet
            Suppress output at a specific level and below
            
            More uses suppresses higher levels of output
                -q:   Only display WARN messages and above
                -qq:  Only display ERROR messages
                -qqq: Suppress all output

    -S, --stateless
            Ignore local state files, do not read from or write to them

    -v, --verbose
            Display more verbose output
            
            More uses displays more verbose output
                -v:  Display debug info
                -vv: Display trace info

    -V, --version
            Print version information

```
# Bible CLI

The purpose of this tool is to be able to read, search, export bible data. Right
now there's only a parser for the Zefania XML format. Bible Translations
in the Zefania XML format can be downloaded from
[strongs-de/zefania-xml-bibles](https://github.com/strongs-de/zefania-xml-bibles)
or [SourceForge](https://sourceforge.net/projects/zefania-sharp/files/Bibles/).

```
USAGE:
    bible-cli [OPTIONS] <BIBLE> [SUBCOMMAND]

ARGS:
    <BIBLE>    Sets the bible xml file to use

OPTIONS:
    -h, --help       Print help information
    -v, --verbose    Sets the level of verbosity
    -V, --version    Print version information

SUBCOMMANDS:
    export    Exports the bible into static json files
    help      Print this message or the help of the given subcommand(s)
    search    searches in the bible
    serve     serves the bible REST api
```

## Export command
This command can be used to export a bible file into many json files. These files
can be used to display the new [strongs-v2](https://github.com/strongs-de/strongs-v2)
web ui.

```
USAGE:
    bible-cli <BIBLE> export [OPTIONS]

OPTIONS:
    -h, --help      Print help information
    -o, --outdir    Output directory
```

Examples:

```bash
# Export a single bible
bible-cli bibles/GER_LUTH1912.xml export

# Export all bible files in a folder
bible-cli "bibles/*.xml" export
```

## Search command

You can use `bible-cli` to search for a word or phrase in a bible translation, using the search command.

```
USAGE:
    bible-cli <BIBLE> search [OPTIONS] [--] [TERM]

ARGS:
    <TERM>    search term

OPTIONS:
    -h, --help                 Print help information
    -t, --times [<time>...]    Execute search given times
```

Examples:

```bash
# Search for a word
bible-cli bibles/GER_LUTH1912.xml search Abraham
```

## Serve command
You can use `bible-cli` to provide a rudimentary REST Api for a chosen bible translation.

```
USAGE:
    bible-cli <BIBLE> serve [OPTIONS]

OPTIONS:
    -h, --help                Print help information
    -p, --port [<port>...]    Port to host the API (default: 8000)
```

Examples:

```bash
bible-cli bibles/GER_ELB1905_STRONG.xml serve
```

The endpoints available are:

```bash
# Get info of the chosen bible translation
curl http://localhost:8000/info

    {
    "identifier": "ELB1905STR",
    "name": "Elberfelder 1905"
    }


# Return a bible chapter
curl http://localhost:8000/{book}/{chapter}

    {
    "chapter": 1,
    "verses": [
        {
        "verse": 0,
        "chunks": [
            {
            "text": "Darauf",
            "strong": {
                "number": 1899,
                "grammar": null
            }
            }
        ]
        }
    ]
    }

# Search in the chosen bible
curl http://localhost:8000/{search_term}

    [
    "0_16_4",
    "0_16_8",
    "0_16_14",
    "0_16_16",
    "0_16_17",
    "0_16_21"
    ]

# Hint: The result is a list of strings in the format "{book_nr}_{chapter_nr}_{verse_nr}"
```

# Run with docker

You can use `bible-cli` with docker:

```bash
docker run --rm -it -v `pwd`:/data mirhec/bible-cli "/data/bibles/*.xml" export
```

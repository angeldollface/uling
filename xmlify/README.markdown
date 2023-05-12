# XMLIFY :scroll: :ribbon:

![GitHub CI](https://github.com/angeldollface/uling/actions/workflows/xmlify.yml/badge.svg)

***Serialize speech transcripts as XML. :scroll: :ribbon:***

## ABOUT :books:

Since I need to serialize and annotate some speech transcripts into XML, I thought I'd write this small tool.

## INSTALLATION :inbox_tray:

You should have the following tools installed and available from the command line:

- [Rust](https://rust-lang.org)
- [Git](https://git-scm.org)

To install ***XMLIFY*** from source, run this command from a terminal:

```bash
cargo install --git https://github.com/angeldollface/uling xmlify
```

This should make the `xmlify` binary available from the command line. The command above will also compile and install the latest cutting-edge version of ***XMLIFY***.

## USAGE :hammer:

### COMMAND LINE AFTER INSTALLATION

Once you have correctly installed ***XMLIFY***, you should have the `xmlify` binary available from the command line.
You can use this tool in the following ways:

- Serialize a transcript text file:

```bash
xmlify --inf ./your_transcript_file.txt
# OR
xmlify -i ./your_transcript_file.txt
```

- Get some version info:

```bash
xmlify --version
# OR
xmlify -v
```

- Get some help info:

```bash
xmlify --help
# OR
xmlify -h
```

## CHANGELOG :black_nib:

### Version 1.0.0

- Initial release.
- Upload to GitHub.

## NOTE :scroll:

- *XMLIFY :scroll: :ribbon:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.

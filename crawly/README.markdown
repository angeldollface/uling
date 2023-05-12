# CRAWLY :scroll: :mag_right:

![GitHub CI](https://github.com/angeldollface/uling/actions/workflows/crawly.yml/badge.svg)

***Search for specific words and surrounding text in a dataset of text (files). :scroll: :mag_right:***

## ABOUT :books:

Since I am writing my bachelor thesis this semester and we're building a corpus in the seminar, I needed a tool that searches for words and gives me various types of information about the search results back. ***Crawly*** is that tool. Enjoy. :heart:

## INSTALLATION :inbox_tray:

You should have the following tools installed and available from the command line:

- [Rust](https://rust-lang.org)
- [Git](https://git-scm.org)

To install ***Crawly*** from source, run this command from a terminal:

```bash
cargo install --git https://github.com/angeldollface/uling -p crawly
```

This should make the `crawly` binary available from the command line. The command above will also compile and install the latest cutting-edge version of ***Crawly***.

## USAGE :hammer:

### COMMAND LINE AFTER INSTALLATION

Once you have correctly installed ***Crawly***, you should have the `crawly` binary available from the command line.
You can use this text-crawler in the following ways:

- Crawl some files from a config file:

```bash
crawly --inf ./your_text_file.txt --wrd "word"
# OR
crawly -i ./your_text_file.txtn -w "word"
```

- Get some version info:

```bash
crawly --version
# OR
crawly -v
```

- Get some help info:

```bash
crawly --help
# OR
crawly -h
```

## CHANGELOG :black_nib:

### Version 1.0.0

- Initial release.
- Upload to GitHub.

## NOTE :scroll:

- *Crawly.rs :scroll: :mag_right:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.

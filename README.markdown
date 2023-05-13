# ULING :mag_right: :scroll:

![GitHub CI](https://github.com/angeldollface/uling/actions/workflows/xmlify.yml/badge.svg)

![GitHub CI](https://github.com/angeldollface/uling/actions/workflows/tately.yml/badge.svg)

![GitHub CI](https://github.com/angeldollface/uling/actions/workflows/crawly.yml/badge.svg)

***A suite of tools for linguistic research. :mag_right: :scroll:***

## ABOUT :books:

Since I needed some tools for some linguistic research this semester, I thought I'd write the tools I need myself in Rust. This Cargo workspace contains these tools. For installation instructions for a certain tool, click on the link to the tool below.

- [XMLIFY](xmlify/README.markdown): Serialize speech transcripts as XML.
- [TATELY](tately/README.markdown): Annotate speech with tags you define.
- [CRAWLY](crawly/README.markdown): Search for specific words and surrounding text in a dataset of text (files).

## INSTALL THE WHOLE SUITE OF BINARIES :inbox_tray:

To install all binaries in this Cargo workspace, make sure you have the following tools installed and available from the command line:

- [Rust](https://rust-lang.org)
- [Git](https://git-scm.org)
- [Cargo](https://crates.io) (Usually comes with Rust.)

To install all binaries, run this command from a terminal session:

```bash
cargo install --git https://github.com/angeldollface/uling xmlify crawly tately
```

## SAMPLE WORKFLOW :test_tube:

Assuming your trasncribed speech file looks something like this and has the name `speech.txt`:

```Text
Speaker 1: My name is John.
Speaker 2: My name is Jane.
```

A sample workflow might look something like this:

- We serialize the speech file using `xmlify`:

```bash
xmlify -i speech.txt
# OR
xmlify --inf speech.txt
```

- We annotate the serialized speech using `tately` (The configuration file for the annotation can be found below the commands.):

```bash
tately -s speech.xml -a annotation.xml
# OR
tately --sld speech.xml --ano annotation.xml
```

```XML
<!--annotation.xml-->
<Annotation word_type="noun">name</Annotation>
<Annotation word_type="verb">is</Annotation>
```

- We then search for occurences of the word type `noun` in the annotated speech using `crawly`:

```bash
crawly -i speech_annotated.xml -w "noun"
# OR
crawly --inf speech_annotated.xml --wrd "noun"
```

## NOTE :scroll:

- *ULING :mag_right: :scroll:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.

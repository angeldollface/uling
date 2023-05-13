# TATELY :black_nib: :test_tube:

![GitHub CI](https://github.com/angeldollface/uling/actions/workflows/tately.yml/badge.svg)

***Annotate speech with tags you define. :black_nib: :test_tube:***

## ABOUT :books:

Since I not only needed to transcribe, serialize, and search spoken speech but also annotate it, I needed a tool to do that for me. This is that tool. Enjoy. :heart:

## INSTALLATION :inbox_tray:

You should have the following tools installed and available from the command line:

- [Rust](https://rust-lang.org)
- [Git](https://git-scm.org)

To install ***Tately*** from source, run this command from a terminal:

```bash
cargo install --git https://github.com/angeldollface/uling tately
```

This should make the `tately` binary available from the command line. The command above will also compile and install the latest cutting-edge version of ***Tately***.

## USAGE :hammer:

### COMMANDS

Once you have correctly installed ***Tately***, you should have the `tately` binary available from the command line.
You can use this tool in the following ways:

- Serialize a transcript text file:

```bash
tately --sld your_serialized_speech.xml --ano your_annotation_database.xml
# OR
tately -s your_serialized_speech.xml -a your_annotation_database.xml
```

- Get some version info:

```bash
tately --version
# OR
tately -v
```

- Get some help info:

```bash
tately --help
# OR
tately -h
```

### FILE FORMATTING

In the first command from the previous section, we had two files: `your_serialized_speech.xml`and `your_annotation_database.xml`.
Both of these files need to follow a strict pattern of formatting and naming. View the examples below to understand the pattern for each file.

- `your_serialized_speech.xml`: This file contains the serialized speech. It ***MUST*** have the same structure as outlined below.

```XML
<Utterance speaker="Host" id="2"> Hey, guess what?</Utterance>
<Utterance speaker="Host" id="3"> Everyone on today's show is completely honest.</Utterance>
<Utterance speaker="Speaker 4" id="7"> All I'm saying is, I can give you my word.</Utterance>
<Utterance speaker="Speaker 4" id="8"> I've been honest throughout the whole game.</Utterance>
<Utterance speaker="Speaker 4" id="9"> I'm an honest guy.</Utterance>
```

- `your_annotation_database.xml`: This file contains the "list" of word types you would like to annotate in the serialized speech. It ***MUST*** have the same structure as outlined below.

```XML
<Annotation word_type="Name">Jane</Annotation>
<Annotation word_type="DM1">I'm thinking</Annotation>
<Annotation word_type="DM1">Don't worry</Annotation>
<Annotation word_type="Con1">If you</Annotation>
<Annotation word_type="Name">Peter</Annotation>
<Annotation word_type="Name">Jasper</Annotation>
```

## CHANGELOG :black_nib:

### Version 1.0.0

- Initial release.
- Upload to GitHub.

## NOTE :scroll:

- *Tately :black_nib: :test_tube:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.

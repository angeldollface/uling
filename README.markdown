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
Speaker 1: What do you do, Jane?
Speaker 2: I'm a support worker.
Speaker 2: You?
Speaker 1: I'm an Astronaut.
Speaker 2: Wow!
```

A sample workflow might look something like this:

- We serialize the speech file using `xmlify`:

```bash
xmlify -i ./speech.txt
# OR
xmlify --inf ./speech.txt
```

- This will yield an XML file by the name of `speech.xml` that looks like this:

```XML
<Utterance speaker="Speaker 1" id="1"> My name is John.</Utterance>
<Utterance speaker="Speaker 2" id="2"> My name is Jane.</Utterance>
<Utterance speaker="Speaker 1" id="3"> What do you do, Jane?</Utterance>
<Utterance speaker="Speaker 2" id="4"> I'm a support worker.</Utterance>
<Utterance speaker="Speaker 2" id="5"> You?</Utterance>
<Utterance speaker="Speaker 1" id="6"> I'm an Astronaut.</Utterance>
<Utterance speaker="Speaker 2" id="7"> Wow!</Utterance>
```

- We annotate the serialized speech using `tately`:

```bash
tately -s ./speech.xml -a ./annotation.xml
# OR
tately --sld ./speech.xml --ano ./annotation.xml
```

- Assuming we're looking for "is" and "name", this would be the configuration file, `annotation.xml`:

```XML
<Annotation word_type="noun">name</Annotation>
<Annotation word_type="verb">is</Annotation>
```

- This will yield the annotated, serialized speech in XML format that looks like this in a file called `speech_annotated.xml`:

```XML
<Utterance speaker="Speaker 1" id="1">My <noun>name</noun> <verb>is</verb> John.</Utterance>
<Utterance speaker="Speaker 2" id="2">My <noun>name</noun> <verb>is</verb> Jane.</Utterance>
<Utterance speaker="Speaker 1" id="3">What do you do, Jane?</Utterance>
<Utterance speaker="Speaker 2" id="4">I'm a support worker.</Utterance>
<Utterance speaker="Speaker 2" id="5">You?</Utterance>
<Utterance speaker="Speaker 1" id="6">I'm an Astronaut.</Utterance>
<Utterance speaker="Speaker 2" id="7">Wow!</Utterance>
```

- We then search for occurences of the word type `noun` in the annotated speech using `crawly`:

```bash
crawly -i ./speech_annotated.xml -w "noun"
# OR
crawly --inf ./speech_annotated.xml --wrd "noun"
```

- That command will yield a file called `speech_annotated_noun.xml` with the following contents:

```XML
<Word entity="noun">
<File>./speech_annotated.xml</File>
<Count>2</Count>
<Line number="1"><Utterance speaker="Speaker 2" id="2">My <noun>name</noun> <verb>is</verb> Jane.</Utterance></Line>
<Line number="0"><Utterance speaker="Speaker 1" id="1">My <noun>name</noun> <verb>is</verb> John.</Utterance></Line>
</Word>
```

## NOTE :scroll:

- *ULING :mag_right: :scroll:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.

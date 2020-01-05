# XkcdAscii
"Rust: A xkcd comic fetcher that converts it to ASCII art" Task for Google Code-In 2019

[![asciicast](https://asciinema.org/a/wxCg6rTvgI1ye7VZLsC9kRoWd.svg)](https://asciinema.org/a/wxCg6rTvgI1ye7VZLsC9kRoWd)


## Usage
`$ cargo run -- [--comic_id {id}]`
When the program is run with a `comic_id` variable, it fetches the comic with that id. If the comic doesn't exists it throws and error otherwise it shows the ascii represantation of the comic. Without the `comic_id` parameter, the program fetches a random comic and shows it.

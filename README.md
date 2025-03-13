# Installation
Prebuilt binaries can be found [here](https://github.com/JayanAXHF/curler/releases).
## Cargo

```sh
cargo install curler
```
## Building from source

```sh
git clone https://github.com/JayanAXHF/curler
cd curler
cargo build --release
```

To run, run `cargo run`
## Details
### JSON Mode
1. Create a `.json` file. For ease of use, create a `paths.json` file in the same directory as the executable.
2. The format for the `.json` file is as follows:

```json
{
	"files": [
		{
			"name": "worksheet.pdf",
			"url": "https://sample.gg"
		},
		{
			"name": "worksheet_ak.pdf",
			"url": "https://sample.gg/sample"
		}
		...
	]
}
```

### Text Mode
1. Create a `text` file. The program defaults to `paths.txt` files in the current directory.
2. The format for the `text` file is as follows:
```txt
filename, url
file2, url
.
.
.
```

## The Programme
This program was made to make it easier for me to download school worksheets, but is compatible with all files. The path to the input file is the path to the `.json` or `text` file with the list of files. *Subject* is used to create a directory where all the downloaded files are stored. Defaults to `./`.

Available Options:
`-s, --subject <SUBJECT>` The subject, or directory, to download the files to. [Defaults to `./`]

`-f, --file-path <FILE_PATH>` The path to the JSON/txt file containing the URLs to download

`-m, --max-threads <MAX_THREADS>`  Maximum number of threads to use [default: 4]

`--mode <MODE>`  Possible values:
	- json: Parse `.json` file (default)
	- text: Parse text file

# Installation
Prebuilt binaries can be found [here](https://github.com/JayanAXHF/curler/releases).
## Cargo
```
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

## The Programme
The programme will ask the user for two things, *A path to a JSON file* and for a *subject*. This program was made to make it easier for me to download school worksheets, but is compatible with all files. The path to the JSON file is the path to the `.json` file with the list of files. *Subject* is used to create a directory where all the downloaded files are stored. Defaults to `./`.

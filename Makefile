INPUT_FILE ?= Cover_Letter_Devrel - Google Docs.pdf
OUTPUT_FILE ?= output.gz

run:
	cargo run -- "$(INPUT_FILE)" "$(OUTPUT_FILE)"

build:
	cargo build

exec:
	./target/debug/your_program "$(INPUT_FILE)" "$(OUTPUT_FILE)"

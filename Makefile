build: compile
	cp target/debug/homepage workflow/source/

compile:
	cargo build

release: compile_universal
	lipo -create -output workflow/source/homepage target/aarch64-apple-darwin/release/homepage target/x86_64-apple-darwin/release/homepage
	
package: release
	cd workflow/source; zip -r ../package/homepage-workflow.alfredworkflow *

compile_universal:
	cargo build --release --target x86_64-apple-darwin --target aarch64-apple-darwin

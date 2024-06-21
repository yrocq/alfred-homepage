build: compile
	cp target/debug/homepage workflow/source/

compile:
	cargo build

release: compile_universal
	lipo -create -output workflow/source/homepage target/aarch64-apple-darwin/release/homepage target/x86_64-apple-darwin/release/homepage
	
package: release
	codesign -s ${APPLE_DEVELOPPER_ID} --timestamp --entitlements homepage.entitlements --options=runtime ./workflow/source/homepage
	cd workflow/source; zip -r ../package/homepage-workflow.alfredworkflow *

compile_universal:
	cargo build --release --target x86_64-apple-darwin --target aarch64-apple-darwin

notarize: package
	xcrun notarytool submit ./workflow/package/homepage-workflow.alfredworkflow --wait --keychain-profile "notarytool-password"
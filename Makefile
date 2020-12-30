build:
	rm -rf pkg
	wasm-pack --verbose build --scope codepunkt --target nodejs
publish:
	wasm-pack --verbose publish --access public
test:
	wasm-pack --verbose test --chrome --headless
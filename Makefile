build:
	rm -rf pkg
	wasm-pack --verbose build --scope codepunkt --target nodejs
publish:
	wasm-pack --verbose publish --access public
test:
	wasm-pack --verbose test --chrome --headless
node:
	rm -rf example/results
	node example/index.js
run:
	make build
	make node
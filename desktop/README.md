# weird-tauri

Local-first website generator.

## Building
### Dependencies

- cargo (rust toolchain)
- pnpm

### Steps

```sh
git clone https://github.com/commune-os/weird.git
cd weird/desktop
git submodule update --init zola
# install dependencies
pnpm install
# create a release build
pnpm run tauri build
./src-tauri/target/weird
```

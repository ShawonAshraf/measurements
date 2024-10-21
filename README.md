# Measurements

## Prerequisites

- `wasm-pack` needs to be installed

## Running locally

### Frontend

```bash

# install the angular workspace dependencies
npm install


# build the wasm module
# from the project root, navigate to the wasm module library location
cd ./projects/sampler/src/lib/sampler-lib
# generate wasm bundle
wasm-pack build --target web

## serve the angular app
ng serve

## to create a production ready build
npm run build app --prod

```

### Serving a production build with python

```bash
# via python
python3 -m http.server -d ./dist/app/browser
# or,
python -m http.server -d ./dist/app/browser
```

## Tests
    
```bash
# wasm
cd ./projects/sampler/src/lib/sampler-lib
cargo test

# frontend
# from the project root
npm run test app
```


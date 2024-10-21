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

### Test inputs for frontend

These readings are taken from the instruction document.

```text
{2017-01-03T10:04:45, TEMP, 35.79}
{2017-01-03T10:01:18, SPO2, 98.78}
{2017-01-03T10:09:07, TEMP, 35.01}
{2017-01-03T10:03:34, SPO2, 96.49}
{2017-01-03T10:02:01, TEMP, 35.82}
{2017-01-03T10:05:00, SPO2, 97.17}
{2017-01-03T10:05:01, SPO2, 95.08}
```

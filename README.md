# hy
command line translation tool implemented in Rust


## Usage

```bash
# install `hy`
$> git clone git@github.com:shohi/hy.git
$> cd hy
$> cargo install

# play
$> hy [WORD]

# help
$> hy -h

```

require `rust 1.39+` (async/await)

## TODO

- [x] provide command flag to set timeout for http request
- [ ] improve error handling
- [ ] support search history and do some stats based on it
- [x] output result in async way
- [x] add version info

## Thanks

1. fanyi, <https://github.com/afc163/fanyi>


## References

1. <https://www.dictionaryapi.com/products/api-collegiate-dictionary>
2. <https://github.com/jokermonn/-Api>

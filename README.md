# Crypt

Sift piped strings through various ciphers.

## Usage example

```bash
$ echo "attack at dawn, my kings" | rev | crypt rot13 --encrypt
$ ftavx lz ,ajnq gn xpnggn
```

All commands are listed in help.

## Implemented

* [x] Atbash
* [x] ROT13
* [X] Caesar

## To implement

* [ ] Affine
* [ ] Baconian
* [ ] Polybius Square

...and hopefully much more.

## Future plans

* [ ] Alphabet support (Russian, German, Hebrew, etc.)
* [ ] CL input support

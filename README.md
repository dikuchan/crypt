# Crypt

Encrypt piped strings.

## Usage example

```bash
$ echo "attack at dawn, my kings" | rev | crypt rot13 --encrypt
$ ftavx lz ,ajnq gn xpnggn

$ echo "gffgka gf zgyt, ec awtsq" | crypt affine -d
$ attack at dawn, my kings
```

All commands are listed in help.

## Implemented

* [x] Atbash
* [x] Affine
* [X] Caesar
* [x] Rot13

## To implement

* [ ] Baconian
* [ ] Polybius Square

...and hopefully much more.

## Future plans

* [ ] Alphabet support (Russian, German, Hebrew, etc.)
* [ ] CL input support

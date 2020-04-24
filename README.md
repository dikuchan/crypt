# Crypt

Sift piped strings through various ciphers.

## Usage example

```bash
$ echo "attack at dawn, my kings" | rev | crypt rot13 --encrypt
$ ftavx lz ,ajnq gn xpnggn

$ echo "hello world" | crypt base64 -d aGVsbG8gd29ybGQ=
$ hello world
```

All commands are listed in help.

## Implemented

* [x] Atbash
* [x] Rot13
* [X] Caesar
* [X] Base64

## To implement

* [ ] Affine
* [ ] Baconian
* [ ] Polybius Square

...and hopefully much more.

## Future plans

* [ ] Alphabet support (Russian, German, Hebrew, etc.)
* [ ] CL input support

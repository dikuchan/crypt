# Crypt

Sift piped strings through various ciphers.

## Example usage

```bash
$ echo "Attack at dawn, my kings" | rev | crypt rot13 --encrypt
$ ftavx lz ,ajnq gn xpnggN
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

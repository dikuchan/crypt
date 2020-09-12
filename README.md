# Crypt

Cryptographic command line tool.

The most simple prominent ciphers are implemented and listed down below.

If you think something is missed, I would gladly inspect an issue or a pull request.

## Examples

```bash
$ crypt rot13 --encrypt "hello world"
$ uryyb jbeyq
```

```bash
$ echo "attack at dawn" | crypt affine -e 15 7 | crypt affine -d 15 7
$ attack at dawn
```

```bash
$ crypt caesar -e 10 "truth is 42" | rev | rev | crypt caesar -d 10
$ truth is 42
```

All commands are listed in help.

## Implemented

* [x] Atbash
* [x] Affine
* [x] Baconian
* [X] Caesar
* [x] Rot13
* [ ] RSA

## Customization

Support of custom languages could be provided.

```json
{
  "cyrillic": {
    "upper": ["А", "Б", ..., "Я"],
    "lower": ["а", "б", ..., "я"]
  }
}
```

Usage:

```bash
$ echo "привет" | crypt rot13 --encrypt --lang cyrillic.json
$ ьэхося
```

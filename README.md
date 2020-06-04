# Crypt

Command line tool for encryption and decryption.

## Description

#### Learn

Crypt provides the interface that is convenient when learning cryptography.

With that idea in mind, most common ciphers and attacks are implemented and their execution are demonstrated step by step.

#### Use

Well-known and prominent ciphers are implemented and listed down below.

If you think something is missed, I would gladly inspect an issue or a pull request.

#### Customize

Additional languages could be added via .json file.

## Examples

```bash
$ crypt rot13 -encrypt "hello world"
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
    * [ ] Brute-force
    * [ ] Hill-climbing
* [x] Rot13
* [ ] RSA
    * [ ] Common modulus
    * [ ] Small decryption exponent
    * [ ] Partial disclosure

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

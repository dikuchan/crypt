# Crypt

Encrypt piped strings.

## Usage example

```bash
$ echo "attack at dawn" | crypt affine -e 15 7 | crypt affine -d 15 7
$ attack at dawn

$ echo "cake is a lie" | crypt rot13 --encrypt | crypt atbash -e
$ kmci eu m bei 

$ echo "kmci eu m bei" | crypt atbash --decrypt | crypt rot13 -d
$ cake is a lie

$ echo "truth is 42" | crypt caesar -e 10 | rev | rev | crypt caesar -d 10
$ truth is 42
```

All commands are listed in help.

## Implemented

* [x] Atbash
* [x] Affine
* [x] Baconian
* [X] Caesar
* [x] Rot13

## To implement

* [ ] Polybius Square

...and hopefully much more.

## Future plans

* [ ] Alphabet support (Russian, German, Hebrew, etc.)
* [ ] CL input support

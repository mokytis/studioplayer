# studioplayer

A new playout for digiplay. Written in rust.

## Motivation

The current playout has a broken audio wall that regular crashes and was
written in 2006. It is about time that Radio Warwick had something new.

Rust was chosen as it is:

 - low level (giving us control of audio devices easily)
 - memory safe
 - has a shouty compiler (hopefully less runtime crashes)

## Limitations

Current this project doesn't allow for remote studio unless you are connected
the VPN. In future a new RESTFul API may be added to digiplay to allow for
non-vpn remote playout.


## Ack

UI is based on [studio-play](https://github.com/radiowarwick/studio-play) which
is an unfinished playout project.

# Objectives

Implement a chat server (step by-step)

- add some kind of identification for users (not authentication yet). For now this is just a username and does not check for the uniqueness of it.

## Steps

On a more advanced level:

1. implement graceful shutdown (see the corresponding article on the tokio tutorial)

2. implement  basic level for an admin authentication: if one gives a specific commands and provides the right password, it can control the server (only one person per time must be granted such access)

3. eventually use the "Introduction to Control Theory And Its Application to Computing Systems" paper to implement some loop control on such server


## Developer's guide

### channels and strings

- use Dispatch as data to be passed through channels

- convert Vec<u8> -> String by `String::from_utf8()?` e String -> Vec by `string.as_bytes()`

- data is NOT cleaned from new line characters `\n`

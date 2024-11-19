# Objectives

Implement a chat server (step by-step)

## Steps

1. Build a simple async server which is binded to a specific address and accepts connections and enum commands
2. Expand the server so that it writes smt to the client
3. have shared state which is given to the client and administered by it
4. expand the server so to be able to react to commands


On a more advanced level:
1. implement graceful shutdown (see the corresponding article on the tokio tutorial)
2. implement  basic level for an admin authentication: if one gives a specific commands and provides the right password, it can control the server (only one person per time must be granted such access)
3. eventually use the "Introduction to Control Theory And Its Application to Computing Systems" paper to implement some loop control on such server
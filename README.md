# Bevy Roguelike game

Here I am trying to do a Roguelike game, using Rust and Bevy. I am closely following the amazing book "Hands-on Rust" by Herbert Wolverson. 
The idea is to re-write his code but using Bevy instead of Legion and Bracket-lib. Features that are not directly related 
to Legion or Bracket-lib will be implement using Bevy if it offers a "better" alternative. Why? It is just a learning experience.
If you check this, feel more than free to provide any sort of feedback.

Stuff implemented so far:

**Chapter 5.** This is the first chapter of the book that covers a Dungeon Crawler. Here we implement a player "@" that moves around using the arrow keys.
We also create random maps (rooms and corridors), we also implement a camera following the player. The book adds art at the end of this chapter but I
will keep it as ASCII so far, but you only need to replace the font file and it will get the art instead.
Main differences against chapter 5 is that I fully use the ECS model, as he does later in chapter 6. I also created the rooms as floor and wall, letting the
other cells be nothing, instead of all walls. Every cell is an entity, and the camera fully uses Bevy, so this is quite different to the book.
If you want to see the code, [check this commit](https://github.com/thephet/BevyRoguelike/tree/b9838c1fcaada49dbea27a9e40fa50c48cda512f).

**Chapter 6.** Enemies are added, and collision detection implemented (if the player moves against an enemy, the enemy dissapears).
If you want to see the code, [check this commit](https://github.com/thephet/BevyRoguelike/tree/861c7751ae4f08a533198803338a79fad684c6bd).

**Chapter 7.** Enemies move randomly. The game now goes in turns between player and enemy movement. This is a bit different to the book because 
we use "state" from Bevy. I think this makes it easier. Movement is re-worked to send messages of intent, similar to the book. 
If you want to see the code, [check this commit](https://github.com/thephet/BevyRoguelike/tree/b4bd4cdc4f4eff145ebe2c070fc5eee07a2bef81).

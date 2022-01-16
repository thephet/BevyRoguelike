# Bevy Roguelike game

<p float="left">
  <img src="https://raw.githubusercontent.com/thephet/BevyRoguelike/main/screenshots/title_screen.png" width="45%" />
  <img src="https://raw.githubusercontent.com/thephet/BevyRoguelike/main/screenshots/ingame.png" width="45%" /> 
</p>

Here I am trying to do a Roguelike game, using Rust and Bevy. I am closely following the amazing book "Hands-on Rust" by Herbert Wolverson. 
The idea is to re-write his code but using Bevy instead of Legion ~~and Bracket-lib~~. Features that are not directly related 
to Legion or Bracket-lib will be implement using Bevy if it offers a "better" alternative. Why? It is just a learning experience.
If you check this, feel more than free to provide any sort of feedback.

[Click here to go to Youtube and see the last video of the game](https://www.youtube.com/watch?v=OJuPTUPgVE8)

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

**Chapter 8.** This was a hard one, because a lot of the features implemented here are not natively implemented in Bevy. Starting with UI, it was wholly redone with Bevy. I also changed the design from the book, instead of having a floating top HP bar, I decided to use a more traditional bottom UI panel. The left part shows the log, which is yet to be implemented (now it only shows 3 static messages), the right part shows the HP information, which is updated as the game progresses. I also implemented mouse click tooltips, which was a pain, because again Bevy has no easy way to recognize which entity received the click, so this had to be done manually, and then getting the tooltip to appear or dissapear. Finally combat has been implemented using messages, and in this case Bevy shines. [check this commit](https://github.com/thephet/BevyRoguelike/tree/b06e6582c68ffd0cc8ba2303f074c38a3b0e880a). [check this youtube video to see the game](https://www.youtube.com/watch?v=CJdQXVfgwsU)

**Chapter 9.** This one was not difficult, but it required a lot of boilerplate coding. This chapter is all about path-planning and about the enemies following you. Before I said that I wanted to avoid Bracket-lib, but their pathfinding library is very good, and I am not going to spend time myself writing an A*. I spent some time playing with the [pathfinding library in rust](https://docs.rs/pathfinding/latest/pathfinding/), and it works well, but ultimate I decided to use bracket-lib and follow the book. In the future I might change this. Anyway so now I got the enemies chasing you, and attacking you. Moreover, the gamelog updates when an important action happens (like an attack). There is also an start screen, a victory screen and a game over screen, and the game can be re-started. To implement this Bevy was very good, and it was really easy. [check this commit](https://github.com/thephet/BevyRoguelike/tree/406f4ac4d334703310f6325b6888ea7a21944c94) [check this youtube video to see the game](https://www.youtube.com/watch?v=SDzFxr87X-8)

**Chapter 10.** This chapter implements field of view, one of the key elements of classic roguelike games. This was an easy and quick one. Basically, since I commited to use Bracketlib, their field of view lib is amazing, so I just used that. I got annoyed because it forces me to use the Point stuff (as far as I know, perhaps there's some other way around in Rust) while I do all my position using Position (3d), but it is what it is. The code itself was quite different because in Bevy we don't handle the render (at least I don't), so it's all about playing with the different Sprite structs to make the stuff visible or not, or change the color. [check this commit](https://github.com/thephet/BevyRoguelike/tree/bc0a70f4c1911859c71f9728a1044b4deb394585) [check this youtube video to see the game](https://www.youtube.com/watch?v=OJuPTUPgVE8)
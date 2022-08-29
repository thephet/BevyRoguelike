# Bevy Roguelike game

<p float="left">
  <img src="https://raw.githubusercontent.com/thephet/BevyRoguelike/main/screenshots/title_screen.png" width="24%" />
  <img src="https://raw.githubusercontent.com/thephet/BevyRoguelike/main/screenshots/dungeon.png" width="24%" /> 
  <img src="https://raw.githubusercontent.com/thephet/BevyRoguelike/main/screenshots/forest.png" width="24%" /> 
  <img src="https://raw.githubusercontent.com/thephet/BevyRoguelike/main/screenshots/caves.png" width="24%" /> 
</p>

Here I am trying to do a Roguelike game, using Rust and Bevy. I am closely following the amazing book "Hands-on Rust" by Herbert Wolverson. 
The idea is to re-write his code but using Bevy instead of Legion ~~and Bracket-lib~~. Features that are not directly related 
to Legion or Bracket-lib will be implement using Bevy if it offers a "better" alternative. Why? It is just a learning experience.
If you check this, feel more than free to provide any sort of feedback.

[Click here to go to Youtube and see the last video of the game](https://www.youtube.com/watch?v=OJuPTUPgVE8) [or this one to see the different maps and themes](https://www.youtube.com/watch?v=rF9SKP2W7BY).

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

**Chapter 8.** This was a hard one, because a lot of the features implemented here are not natively implemented in Bevy. Starting with UI, it was wholly redone with Bevy. I also changed the design from the book, instead of having a floating top HP bar, I decided to use a more traditional bottom UI panel. The left part shows the log, which is yet to be implemented (now it only shows 3 static messages), the right part shows the HP information, which is updated as the game progresses. I also implemented mouse click tooltips, which was a pain, because again Bevy has no easy way to recognize which entity received the click, so this had to be done manually, and then getting the tooltip to appear or dissapear. Finally combat has been implemented using messages, and in this case Bevy shines. [check this commit](https://github.com/thephet/BevyRoguelike/tree/b06e6582c68ffd0cc8ba2303f074c38a3b0e880a) and [check this youtube video to see the game](https://www.youtube.com/watch?v=CJdQXVfgwsU).

**Chapter 9.** This one was not difficult, but it required a lot of boilerplate coding. This chapter is all about path-planning and about the enemies following you. Before I said that I wanted to avoid Bracket-lib, but their pathfinding library is very good, and I am not going to spend time myself writing an A*. I spent some time playing with the [pathfinding library in rust](https://docs.rs/pathfinding/latest/pathfinding/), and it works well, but ultimate I decided to use bracket-lib and follow the book. In the future I might change this. Anyway so now I got the enemies chasing you, and attacking you. Moreover, the gamelog updates when an important action happens (like an attack). There is also an start screen, a victory screen and a game over screen, and the game can be re-started. To implement this Bevy was very good, and it was really easy. [check this commit](https://github.com/thephet/BevyRoguelike/tree/406f4ac4d334703310f6325b6888ea7a21944c94) and [check this youtube video to see the game](https://www.youtube.com/watch?v=SDzFxr87X-8).

**Chapter 10.** This chapter implements field of view, one of the key elements of classic roguelike games. This was an easy and quick one. Basically, since I commited to use Bracketlib, their field of view lib is amazing, so I just used that. I got annoyed because it forces me to use the Point stuff (as far as I know, perhaps there's some other way around in Rust) while I do all my position using Position (3d), but it is what it is. The code itself was quite different because in Bevy we don't handle the render (at least I don't), so it's all about playing with the different Sprite structs to make the stuff visible or not, or change the color. [check this commit](https://github.com/thephet/BevyRoguelike/tree/bc0a70f4c1911859c71f9728a1044b4deb394585) and [check this youtube video to see the game](https://www.youtube.com/watch?v=OJuPTUPgVE8).

**Chapter 11.** This was a very fun chapter to read, but not very fun to implement, basically because there's no ECS, so there's no Bevy, and this chapter is mostly a copy paste of the original code. I only had to change a few non-important things so that it works with my game. The only change I did was to create a "prefab architect", instead of having prefab like a function that you can apply to any map. I plan to use this prefab architect for boss levels. [check this commit](https://github.com/thephet/BevyRoguelike/tree/31eca1705da44c6d5a233bd9687651d8a9a50c0a).

**Chapter 12** This chapter is about adding themes. Again, like the previous chapter, there's no ECS involved so it is mostly copy paste. The main difference is that Bevy's render is different, so I had to work around that. [check this commit](https://github.com/thephet/BevyRoguelike/tree/4d196c8faa9d78eb71645e903dc5f874d03642fa) and [check this video to see the game](https://www.youtube.com/watch?v=rF9SKP2W7BY).

**Chapter 13** This chapter is about adding items, like potions, and inventory management. By far the chapter that took me the most time. The backend is very similar to the book, but using Bevy. The frontend is very different. I wanted to implement a more "classic" popup iventory menu, where the user can pick an item to use scrolling through the menu. This was a bit hard with bevy. I think UI stuff is what I like the least. [check this commit](https://github.com/thephet/BevyRoguelike/tree/e403de02421e2c5be7d36c98186e6f554096bd4e) and [check this video to see the game](https://www.youtube.com/watch?v=TO99KHgz4iI).

**Chapter 14** This chapter is about making deeper dungeons, where you arrive to some stairs, and stepping into them brings you to the next level. Since the whole ECS from Bevy is a bit different to the code describe in the book, the implementation here was very different. It was all about playing with states, and swapping between states as the player changes levels, and enabling and disabling different systems. In the book there's a function "next level" that does it all, while here it is spread in different scripts. [check this commit](https://github.com/thephet/BevyRoguelike/tree/e2d4c8c1e5432576416eadb87587841a5597907a) and [check this video to see the game](https://youtu.be/0A54EDGwdZ8).

**Chapter 15** This chapter has two different objectives. The first one is about implementing a data-driven approach where the different entities are fetched from a template file instead of being programatically generated in the code. The idea behind this is to split the development between game design and engine design. The second objective is about adding equipment, such as swords, and variable damage depending on the equipment used.  [check this commit](https://github.com/thephet/BevyRoguelike/commit/8f9b329d6509724f2f6f65770a57ca1db50a8bd6) and check these two youtube videos: [equipment](https://youtu.be/4kOhoIGzTqo) and [data-driven design](https://youtu.be/_kYizf5Ybgc). This chapter also finished the book!
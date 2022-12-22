# Automated Kingdom

Multiplayer top down game written in rust and using macroquad for graphics. Written in Rust and use Macroquad crate for graphics and ws crate for web socket server, also written in Rust. Will target both native as well as wasm, with online matchmaking.

## How the game works

Players spawn in a randomly generated world with up to 3 other players or AI open with natural resources such as trees, iron and gold. Players first build workers with wood and iron which they can then assign to work and build defenses around their homebase, with an electricity system and a heavy focus on traps and using powered items to gather resources quicker.

Players can also train a wide variety of troops, which they can then use to attack other enemy bases or defend their own. Troops will auto attack, but the player can command them to go to certain locations, focus a certain troop or hold their ground. Players can assign troops to certain groups, allowing for easier manipulation. You can upgrade troops and invest in sorcery as well as spy on enemies using drones.

## Tools and ideas to be used

- A* for pathing
- Enemy opponent AI
- Troop AI
- Random world generation
- Web sockets
- WASM
- Macroquad

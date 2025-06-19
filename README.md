# Tron-IO
Basic tron light cycle game (non-copyrighted name pending) to learn Rust gamedev and to experiment with multiplayer protocols

## MVP

- [x] Modify snake-game template to be Tron-game
- [x] Allow any colors and store colors on grid
- [x] Main Menu
- [x] Touch support
- [x] Multiplayer support
- [ ] Redo sockets or SSL multiplayer support :()
- [x] Setup public server
- [x] WASM build
- [ ] Non-copyrighted name
- [x] Boost
- [x] Settings
- [x] Options
- [ ] In-game pause menu
- [ ] Fix virtual gamepad AND/OR fix creating a new render target on resize on web
- [ ] Rotate screen prompt on mobile

## Wishlist
- [x] Teams
- [x] Local multiplayer with controls
- [x] Sounds/music obvy
- [x] Keep playing the game for a few frames after all but one is dead
- [ ] Client-side prediction with rollback (Should be cheap because grid is so over-optimized...)
- [ ] Brakes or jump?
- [ ] Add real lobby
- [ ] Computer sounding names! (For teams and players (CPU, MEMORY, etc...)) (Cute descriptions (Segfault, OOM, etc...))
- [ ] Server browser
- [ ] Explosions
- [ ] Steal armegetron
- [ ] Sprite or diff thing for head
- [ ] Head interpolation
- [ ] Virtual Gamepad drag instead of click
- [ ] Spritesheet with: Simple bike sprite, gamepad arrows/buttons, rotate screen icon
- [ ] Animations: Death explosion and slow fade to black, countdown to start, etc
- [ ] More AI enehancements: Random behaviour (including mistakes based on difficultly), boost
- [ ] More interseting main menu: Large AI battle with random options
- [ ] Colorblind-friendly colors...
- [ ] Real color pallatte... (terminal themed???)

## Ideas
 - Powerups to keep things interesting: Shoot, shield, etc...
 - Alternate disc throwing game wars (boomerang fu)
 - Snake mode or fixed trail length mode or no collide own tail mode or fill the box grid game mode or grow biggest mode
 - Something more visually interesting: Shader, ASCII terminal vibes, 
 - Campaign about hacking into core memory, snarky things said in chat, map overview of progress, OR turn-based overworld?
 - Race through interesting platformer levels?
 - **Interesting maps!** Obstacles, platforms, voids in map OR Side scroller instead
 - Break out of grid?
 - Light sail rider mode? Light sail to get to the next level?
 - Metroidvania LOL
 - Try camera that follows player...

## Graveyard
 - Camera zoomed in on player: felt weird and boring, easy to experiment with again if desired
 - Sounds for every turn: Way too many SFX
 - Snake-game: Way too hard to cut people off, felt boring
 - Shoot-only 'Tank" game: Also way too hard to shoot people and boring
 - Draw player infos in crossed font when dead... (Looked terrible...)

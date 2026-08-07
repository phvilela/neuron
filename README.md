# Neural Network to play pong

Neural Network is not implemented yet

## Game
For now The game is implemented to run in the tty/pts<br>
Classic pong game

* Run:
```sh
cargo run
```

### Controls
- Player 1: `w` and `s` to move the bar up and down
- Player 2: `UpArrow` and `DownArrow` to move the bar up and down
- `Esc` to quit

### Layout

The app size is extracted from the terminal, it first check stdout, then stderr and finally stdin, so it also works with redirection, and it is implemented with a spam mode, so if you can access another tty device you can display your game in another output space and it fits the screen

```
Ball: Vector: x = 56.81543 , y = 42.9797	 A : 2 B : 0

|
|
|                   o                                   |
                                                        |
                                                        |
```
# How to

...actually write a Wii game with this stuff. Be sure to group stuff in modules and to make things as platform independent as possible. This is a Wii specific project, yes, but there's no reason to abandon the old principles:

- KISS : Keep It Simple and Stupid : no fancy one-liners and complex systems where the simple thing will work just fine.
- ETC : Easy To Change, code should be flexible in the parts that will most likely change, and change should be easy to implement (no updating 12 files).
- Low Coupling, High Cohesion!

## The gist

It's probably a bit difficult to take all this code and know how to work with it. The main point is that you should only really be concerned with the following files / folders:

- `app/src/raw_data_store.rs` : this is where you add new data files to be accessed elsewhere. Be sure to put them in `app/data`.
- `app/src/change_provider.rs` and `app/gamelib/src/game_state/changes/controls.rs` : augment these two files to add different control sources to the game.
- `app/gamelib/src/game_state/components` : this is is where you add **components**.
- `app/gamelib/src/game_state/systems` : this is where you add new **systems** that work on different components. Be sure to define a system name in `system_name.rs`
- `app/gamelib/src/game_states` : this is where you add new **game states** that define a given world with **entities**.

### Components and Entities

We use the `hecs` library to implement an Entity Component System (ECS). See [the docs](https://docs.rs/hecs/latest/hecs/) for more info.

### Systems

Although we won't be explaining all of `hecs` for you, it is important to know how we use systems: we define them in modules and register under enum names.
When adding a system be sure to add it to `SystemName` too. You can reference the system name when building a game state.

### Game States

The core addition to `hecs`'s ECS is that we can easily define `GameState`s. One state can move to another state, and multiple states can reference the same systems and components.
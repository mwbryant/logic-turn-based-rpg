# Logic Turn Based RPG

This is a [Bevy](https://bevyengine.org/) game meant to create a turn based rpg in the same design as old Paper Mario Games.  This follows the devlogs at [LogicProjects on Youtube](https://youtu.be/RjY-nyeiwVI).

The game currently features both an overworld and a combat state.  In the overworld you can move with WASD and interact with NPCs by approaching them and pressing E (as their icon indicates).  If you get too close to a wandering enemy it will start a battle with them.

In combat you can select between 2 attacks, a melee and a ranged attack, using A, D, and Space.  If you press Space right when your attack hits an enemy it will Crit and do double damage.  You can also block enemy attacks by pressing Space right when they hit you.

![Example Gif](gifs/rpg_demo.gif)

All code is broken into 3 main modules currently. Art contains effects like the fade in and particles.  Combat contains all systems and components relating to the combat state.  Overworld contains all the systems and components relating to the overworld state.

Internally combat is also represented as a state machine to handle the transitions between players choosing an attack, actually attacking, and the enemy attacking.

Enemies in the overworld are loaded from external ron files in the config folder which describe how they move in the world and link to their assoicated battle.  The battles are also described in ron files that contain what enemies will appear in each slot and their stats.

All code were created by LogicProjects and are free to use in any way without restriction.  
Art is from [Kenney](https://www.kenney.nl/assets?q=3d).

This is meant to be an educational project and you should feel free to use any code in your own projects!

# Usage

```
cargo run
```

# States

## Game State

Currently the top level state is the GameState.  In the future this will handle extra states like Main Menu, Credits, and Minigames.

```rust
pub enum GameState {
    #[default]
    Overworld,
    Combat,
}
```

The transistion_to_overworld system changes the state from Combat to Overworld when the winning fadeout is finished.  The start_combat system changes from Overworld to Combat when the player encounters an enemy and the fade finishes.

Nothing happens on update in either state currently and the game assumes that the other states will be properly maintained to their NotInXXX state.  This results in cleaner scheduling logic because we don't need to double list the state set on every system.

Entering Combat causes the setup_combat and the spawn_enemy_health_ui system to run. Nothing runs on Entering Overworld because overworld has a multiframe LoadingRoom state.

Exiting Combat will cause all entities tagged with CombatEntity will despawn.  On exiting Overworld all tagged with OverworldEntity will also despawn.

## Combat State

The combat state controls the basic flow of combat.  The intent is for the stages to cycle on each turn.  EnemyDying exists to ensure enemies have time to despawn to prevent the player from attacking a dead enemy or a dead enemy attacking the player.  PlayerWins is used during the winning cutscene.

```rust
pub enum CombatState {
    #[default]
    NotInCombat,
    PlayerSelecting,
    PlayerAttacking,
    EnemyAttacking,
    EnemyDying,
    PlayerWins,
}
```

Most state changes happen during the attack_flow system which checks who just finished an attack and if any enemies died due to the attack.  

Input is read in player_select_attack system which can set the state to PlayerAttacking.  

EnemyDying is just used as a holding state and the state is set to EnemyAttacking by wait_for_death after all animations are done.

TODO Add state diagram

## Overworld State

The overworld state mainly handles the loading/restoring of rooms and if the player is free to move or is locked into a dialog.  

The broad flow is loading is preformed for new rooms and then once the assets load the restore state runs for a single frame to give a time for all entities to spawn.  Then the player is left free to roam the world, interacting with an NPC puts you into Dialog and encountering an enemy causes CombatStarting where the combat descriptor spawns and a fadeout starts.

```rust
pub enum OverworldState {
    #[default]
    LoadingRoom,
    RestoreRoom,
    FreeRoam,
    CombatStarting,
    Dialog,
    NotInOverworld,
}
```

Most systems only run during the FreeRoam state.  These include player_movement, camera_follow, enemy_wander, enemy_start_combat, player_interaction, and update_player_translation_in_room.

The enemy_start_combat system checks for the player to be in range of an enemy and sets the state to CombatStarting.

Interacting with an NPC and closing the dialog can change the state to and from Dialog in the npc module.

The room module handles the LoadingRoom and RestoreRoom states.  LoadingRoom should only last as long as there are unloaded assets needed to spawn the room, it sets the state to RestoreRoom as fast as possible which spawns entities based on the CurrentRoom resource.
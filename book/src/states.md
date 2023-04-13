# States

## Game State

Currently the highest level state is the GameState.  In the future this will handle extra states like Main Menu, Credits, and Minigames.

```rust
pub enum GameState {
    #[default]
    Overworld,
    Combat,
}
```

The transistion_to_overworld system changes the state from Combat to Overworld when the winning fadeout is finished.  The start_combat system changes from Overworld to Combat when the player encounters an enemy and the fade finishes.

Nothing happens on update in either state currently and the game assumes that the other states will be properly maintained to their NotInXXX state.  This results in cleaner scheduling logic because we don't need to double list the state set on every system.

Entering Combat causes the setup_combat and the spawn_enemy_health_ui system to run.

Exiting Combat will cause all entities tagged with CombatEntity will despawn.  On exiting Overworld all tagged with OverworldEntity will also despawn.

## Combat State

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

## Overworld State

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
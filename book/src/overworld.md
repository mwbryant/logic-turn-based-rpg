# Overworld

The overworld module is intrinsically tied to [OverworldState](states.md) and focuses on swapping between the player being able to roam around the world and other things such as triggering combats and dialogs.

enemy.rs controls the enemy AI and checks if the player is in range to start combat.

npc.rs handles creating npc dialog boxes, interaction icons, and closing dialogs.

player.rs focuses on moving the player and updating the camera.

room.rs handles everything related to loading a room including checking for assets to be ready and restoring a current room after combat.

start_combat.rs checks for the fade triggered by an enemy to be finished and changes every game state to start the encounter.

walls.rs handles creating hitboxes for obstacles in the room.

# Combat

The combat module is intrinsically tied to [CombatState](states.md) and most systems focus on facilitating the main flow between the states.

animation.rs, ui.rs, and graphic_effects.rs handle things like the player moving toward and enemy, health bars, and spawning particles on attack contacts.

The main system is attack_flow in attack.rs which controls what state combat should change into and checks for deaths and results of attacks.

player_wins.rs handles the final state of combat and plays all the winning effects and eventually spawns the fadeout that will end combat.  In the future this will handle much of the complexity of level ups and battle rewards.

selection.rs focuses on creating the menu for the player to select and lock in their attack and the inputs around this.

start_combat.rs contains the systems responsible for spawning everything when first entering combat.  This includes the enemies and the background art.

turn_based.rs handles many of the core rpg systems such as damage calculations and death checks.  It also tracks if the player has input an action timing for defense or offense.

weapon.rs establishes different attacks and the stages of each attack (walk-up, hit, cooldown, etc).
# Art

The art module is designed as a utility for other core simulation modules to spawn entities from to trigger graphical effects.

Currently it offers functionality to let the programmer change any character or icons sprite by just updating an enum.  This provides a powerful abstraction away from creating sprite bundles and managing indices directly.

The fade effect is used commonly through out the code to mask mass despawning and spawning of entities.  To use a fade the programmer just needs to spawn the fade and optionally tag it with a tracker component.  The fade then triggers the just_finished flag for one frame when the screen is completely covered.  In the future this should support triggering a bevy event.

The particle system is accessed through the create_new_rect_emitter function which takes in a description of the particle to spawn and the properties of the emitter.  This creates 2 entities but the programmer is only responsible for the returned emitter.  The plugin will handle the particle parent.

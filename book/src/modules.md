# Modules

The game is currently organized into a module system.  As a collection of related functionality grows eventually it will be moved into a folder and become a new module.  Modules define a root plugin and all of their public components in the mod file.  Everything else within a module is considered an implementation detail of that subsystem and components should be considered it's interface to the outside game.

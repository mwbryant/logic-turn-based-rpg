# Introduction

## Overview

This is the documentation and design document for an RPG written in the Bevy Game Engine.  The project's development is being documented at [Logic Projects on Youtube](https://www.youtube.com/@logicprojects).

The goal of this project is to create a completely open source RPG game in the style of old Mario RPGs that is released on itch.io and eventually Steam.

## Main Design

The game should take place over 4 main chapters telling a hero story.  The goal is to create a short game polished game that can actually be completed in a few months.

The core strategy of combat is going to be based around a "badge" system where the player can equip a subset of different badges to get strategic advantages and new attacks.

Combat will feature timed inputs to do extra damage and to block incoming damage.  

The overworld should feature rich characters and stores for the player to make meaningful choices during downtime between combat.  Hopefully there will be some engaging minigames and side quests for players to earn extra items if they are struggling.

## Why A Book

The main question I'm expecting about this document is why a book to document the game instead of the built in Rust docs.  I have a couple of reasons for this:

First: Code is intrinsically non-linear but I believe a linear description of the design would be more useful for tracking how the project is structured.  It is more important to understand the spirit behind states and how they interact to produce a game than it is to see a listing of the functions you can call on the states.  This book aims to be a strong starting point into the design of the game where the actual code or docs would not present a solid path towards understanding the codebase.

Second: Comments rot, even projects with strict documentation standards like Bevy experience comments rotting this directly leads to bad documentation (I've reported multiple issues with this).  Books can rot too but I'm hoping the high level spirit of systems is less likely to age compared to function by function documentation.

Third: The act of writing the book is forcing a new way of thought around the code in the project.  Wording why certain things work the way they do has already encouraged many systems to be redesigned for the better.

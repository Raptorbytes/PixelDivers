# PixelDivers - Pixel Art 
## 1. Game Concept & Genre
Genre: 2D pixel art shooter with squad-based mechanics and action-oriented gameplay.
Theme: Players are elite soldiers sent to dangerous planets to complete missions and face overwhelming enemy forces.
Core Mechanics:
Player-controlled squad (either solo or co-op).
Combat-centric gameplay (shooting, dodging, and strategic usage of support tools).
Mission-based: Players complete objectives such as enemy elimination, resource collection, or defense while navigating hostile terrain.
Friendly fire is always on to add chaos and strategic depth.

## 2. Core Features
Game Modes
Solo Mode: Players control a single character with AI companions.
Co-op Mode: Multiplayer (local or online). Players can join forces to complete missions.
Combat System
Real-time combat: Fast-paced with shooting in multiple directions.
Stratagems: Players can call for support (e.g., airstrikes, supply drops, turrets), but only when safe or if you wanna get risky, in combat.
Weapons: Multiple weapon types with various stats (e.g., machine guns, sniper rifles, grenades).
Health & Ammo: Resource management for ammo and health.
Enemy Types
Bug & bot swarms: Overwhelming numbers of basic enemies.
Boss fights: Larger, tougher enemies with unique abilities.
Environmental hazards: deep water, toxic gas zones, active hellbombs, pits, etc.

## 3. Pixel Art & Visual Style
Player & Enemy Designs:


Simple, chunky sprites with a sci-fi, militaristic look.
Player characters and enemies will be 32x32 pixel art (Bile titans and other massive enemies will be 96x96, medium enemies, I.E Hulks, will be 64x64, etc)
Weapons and environmental objects (e.g., crates, destructible objects) will also follow the pixel art style.
World Design:


Tile-based maps: 32x32 tiles to represent environments like caves, jungles, and space bases.
Scrolling maps: As the player moves, the map scrolls.
Dynamic lighting: Dark environments with light sources (e.g., flashlights, explosions).

## 4. Gameplay Features & Mechanics
Movement & Combat
Movement: WASD and mouse for player control.
Shooting: Mouse 360 aiming (e.g., left mouse for shooting, Q for grenades).
Camera: Top-down or isometric view to give a full view of the environment.
Aiming: 360-degree shooting system.
Friendly Fire: Always on— players need to be careful when shooting around teammates.
Stratagem System
Real-time usage: Players can call in support, and it will drop within its time rating
Support Options: Airstrikes, reinforcements, supply drops, support weapons, barricades(?) and turrets.
AI Squadmates
AI-controlled teammates with basic commands (move, attack, defend).
Friendly fire will affect AI teammates too, so players need to be careful.

## 5. Game Progression & Replayability
Mission System
Procedural Generation: Random mission objectives and maps for varied gameplay.
Objectives: Enemy elimination, escort missions, or resource retrieval.
Difficulty Levels: Gradual increase in enemy toughness and mission complexity.
Character & Equipment Progression
Unlockable weapons, armor, and stratagems: Over time, players unlock new weapons and gear.
Leveling system: Players can improve their character’s stats (health, stamina, and shooting accuracy).
Multiplayer
Local Co-op: Split-screen or local hotseat with shared resources.
Online Co-op: TCP or local server-based multiplayer for online play.

## 6. Tech Stack
Game Engine & Libraries
Bevy: Best for 2D games in Rust, offering great flexibility for pixel art and physics-based movement.
Handles 2D rendering, asset loading, and animations.
Pixel-Art Framework: specs (for entity-component-system-based game architecture) can work well with Bevy for handling game states and mechanics.
Sound & Music
Sound Effects: Create or source sounds for gunfire, explosions, and player actions.
Music: Atmospheric, synthwave music to match the sci-fi theme.
Libraries: Use rodio for sound or cpal for audio playback in Rust.

## 7. Development Phases
Phase 1: Core Mechanics & Prototype
Movement & Combat: Implement basic player movement, shooting mechanics, and enemy AI.
Basic Environment: Create a simple, scrollable map with pixel art tiles and basic enemies.
HUD: Display health, ammo, and active stratagems on the screen.
Phase 2: Refinement & Features
Squad AI: Implement AI-controlled teammates with basic commands.
Mission & Progression: Add procedural mission generation and unlockable content.
Stratagem System: Add the support system, with safe zones and strategic play.
Phase 3: Polish & Multiplayer
Co-op Mode: Implement local multiplayer, then move to online multiplayer.
Art Polish: Refine pixel art for players, enemies, and environment.
Audio: Add sound effects and music to enhance the gameplay experience.

## 8. Potential Challenges
AI Behavior: Ensuring that AI squadmates are intelligent enough without getting in the player’s way.
Multiplayer: Handling synchronization and network issues for online play.
Procedural Generation: Ensuring that procedural maps and missions are varied but still fun.
Next Steps
Set up a project in Rust with the Bevy framework.
Start by creating basic movement and combat systems before adding pixel art assets.
Focus on building the core mechanics (movement, shooting, AI) before moving to more advanced features like multiplayer.
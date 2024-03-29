// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Availability } from "./Availability";
import type { DungeonAvailability } from "./DungeonAvailability";
import type { DungeonBoss } from "./DungeonBoss";
import type { DungeonReward } from "./DungeonReward";
import type { LocationPosition } from "./LocationPosition";
import type { Medallion } from "./Medallion";

export interface Dungeon { name: string, dungeonCode: string, hoverText: string, totalChests: number, clearedImage: string, defaultImage: string, hasReward: boolean, position: LocationPosition | null, boss: DungeonBoss | null, foundChests: number, reward: DungeonReward, medallion: Medallion, cleared: boolean, dungeonAvailability: Availability, bossAvailability: Availability, logic: DungeonAvailability | null, }
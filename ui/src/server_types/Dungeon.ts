import type { DungeonReward } from "./DungeonReward";
import type { LocationPosition } from "./LocationPosition";
import type { Medallion } from "./Medallion";
import type { Availability } from "./Availability";
import type { DungeonBoss } from "./DungeonBoss";
import type { DungeonAvailability } from "./DungeonAvailability";

export interface Dungeon { name: string, dungeonCode: string, hoverText: string, totalChests: number, clearedImage: string, defaultImage: string, hasReward: boolean, position: LocationPosition | null, boss: DungeonBoss | null, foundChests: number, reward: DungeonReward, medallion: Medallion, cleared: boolean, dungeonAvailability: Availability, bossAvailability: Availability, logic: DungeonAvailability | null, }
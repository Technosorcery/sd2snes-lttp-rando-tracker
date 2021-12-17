import type { DungeonAvailability } from "./DungeonAvailability";
import type { LocationPosition } from "./LocationPosition";
import type { DungeonBoss } from "./DungeonBoss";
import type { DungeonReward } from "./DungeonReward";
import type { Availability } from "./Availability";
import type { Medallion } from "./Medallion";

export interface Dungeon { name: string, dungeonCode: string, hoverText: string, totalChests: number, clearedImage: string, defaultImage: string, hasReward: boolean, position: LocationPosition | null, boss: DungeonBoss | null, foundChests: number, reward: DungeonReward, medallion: Medallion, cleared: boolean, dungeonAvailability: Availability, bossAvailability: Availability, logic: DungeonAvailability | null, }
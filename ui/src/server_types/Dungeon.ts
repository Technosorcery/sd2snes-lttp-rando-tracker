import type { DungeonAvailability } from "./DungeonAvailability";
import type { Medallion } from "./Medallion";
import type { LocationPosition } from "./LocationPosition";
import type { DungeonReward } from "./DungeonReward";
import type { DungeonBoss } from "./DungeonBoss";
import type { Availability } from "./Availability";

export interface Dungeon { name: string, dungeonCode: string, hoverText: string, totalChests: number, clearedImage: string, defaultImage: string, hasReward: boolean, position: LocationPosition | null, boss: DungeonBoss | null, foundChests: number, reward: DungeonReward, medallion: Medallion, cleared: boolean, dungeonAvailability: Availability, bossAvailability: Availability, logic: DungeonAvailability | null, }
import type { DungeonReward } from "./DungeonReward";
import type { Medallion } from "./Medallion";

export interface DungeonUpdate { foundChests: number | null, reward: DungeonReward | null, medallion: Medallion | null, cleared: boolean | null, }
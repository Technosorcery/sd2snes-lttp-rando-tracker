import type { Medallion } from "./Medallion";
import type { DungeonReward } from "./DungeonReward";

export interface DungeonUpdate { foundChests: number | null, reward: DungeonReward | null, medallion: Medallion | null, cleared: boolean | null, }
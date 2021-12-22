import type { RandoLogic } from "./RandoLogic";
import type { DataSourceType } from "./DataSourceType";

export interface ServerConfigUpdate { dataPollRate: number | null, dataSource: string | null, logic: RandoLogic | null, sourceType: DataSourceType | null, }
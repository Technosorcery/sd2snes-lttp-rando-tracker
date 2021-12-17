import type { DataSource } from "./DataSource";
import type { RandoLogic } from "./RandoLogic";

export interface ServerConfig { dataPollRate: number, dataSource: DataSource, logic: RandoLogic, apiPort: number, }
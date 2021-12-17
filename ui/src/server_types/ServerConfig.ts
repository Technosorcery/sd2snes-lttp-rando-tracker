import type { RandoLogic } from "./RandoLogic";
import type { DataSource } from "./DataSource";

export interface ServerConfig { dataPollRate: number, dataSource: DataSource, logic: RandoLogic, apiPort: number, }
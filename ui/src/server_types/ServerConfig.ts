import type { RandoLogic } from "./RandoLogic";
import type { DataSourceType } from "./DataSourceType";

export interface ServerConfig { apiPort: number, dataPollRate: number, dataSource: string, logic: RandoLogic, qusbDevices: Array<string>, sourceType: DataSourceType, }
import type { Qusb2snesConfig } from "./Qusb2snesConfig";
import type { LocalFileConfig } from "./LocalFileConfig";

export type DataSource = { type: "LocalFile" } & LocalFileConfig | { type: "Qusb2snes" } & Qusb2snesConfig;
import type { LocalFileConfig } from "./LocalFileConfig";
import type { Qusb2snesConfig } from "./Qusb2snesConfig";

export type DataSource = { type: "LocalFile" } & LocalFileConfig | { type: "Qusb2snes" } & Qusb2snesConfig;
import type { LocationPosition } from "./LocationPosition";
import type { Availability } from "./Availability";

export interface Location { name: string, hoverText: string, position: LocationPosition, cleared: boolean, availability: Availability, }
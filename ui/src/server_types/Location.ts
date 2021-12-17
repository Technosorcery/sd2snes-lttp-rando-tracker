import type { Availability } from "./Availability";
import type { LocationPosition } from "./LocationPosition";

export interface Location { name: string, hoverText: string, position: LocationPosition, cleared: boolean, availability: Availability, }
import type { Gloves } from "./Gloves";
import type { Bottle } from "./Bottle";
import type { Shield } from "./Shield";
import type { BigKey } from "./BigKey";
import type { Sword } from "./Sword";
import type { Crystal } from "./Crystal";
import type { Magic } from "./Magic";
import type { Armor } from "./Armor";
import type { Pendant } from "./Pendant";

export interface GameState { bow: boolean, blueBoomerang: boolean, redBoomerang: boolean, hookShot: boolean, bomb: number, mushroom: boolean, powder: boolean, fireRod: boolean, iceRod: boolean, bombosMedallion: boolean, etherMedallion: boolean, quakeMedallion: boolean, lantern: boolean, hammer: boolean, flute: boolean, fluteActivated: boolean, shovel: boolean, net: boolean, book: boolean, bottle: boolean, bottleCount: number, caneSomaria: boolean, caneByrna: boolean, cape: boolean, mirror: boolean, silvers: boolean, gloves: Gloves, boots: boolean, flippers: boolean, moonPearl: boolean, swordLevel: Sword, shieldLevel: Shield, armorLevel: Armor, bottleContent1: Bottle, bottleContent2: Bottle, bottleContent3: Bottle, bottleContent4: Bottle, rupees: number, heartQuarters: number, bombCapacity: number, hearts: number, maxHearts: number, arrows: number, arrowCapacity: number, magicProgression: Magic, smallKeys: number, bigKey: BigKey, pendant: Pendant, crystal: Crystal, }
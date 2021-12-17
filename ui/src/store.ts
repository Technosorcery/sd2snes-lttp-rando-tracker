import { InjectionKey } from 'vue'
import { createStore, useStore as baseUseStore, Store } from 'vuex'

enum ArmorLevel {
  GreenMail = "GreenMail",
  BlueMail = "BlueMail",
  RedMail = "RedMail",
}

enum Gloves {
  None = "None",
  PowerGlove = "PowerGlove",
  TitansMitt = "TitansMitt",
}

enum Sword {
  None = "None",
  FightersSword = "FightersSword",
  MasterSword = "MasterSword",
  TemperedSword = "TemperedSword",
  GoldenSword = "GoldenSword",
}

enum Shield {
  None = "None",
  FightersShield = "FightersShield",
  RedShield = "RedShield",
  MirrorShield = "MirrorShield",
}

enum BottleContent {
  NoBottle = "NoBottle",
  Mushroom = "Mushroom",
  Empty = "Empty",
  RedPotion = "RedPotion",
  GreenPotion = "GreenPotion",
  BluePotion = "BluePotion",
  Fairy = "Fairy",
  Bee = "Bee",
  MagicBee = "MagicBee",
}

enum Magic {
  Normal = "Normal",
  Half = "Half",
  Quarter = "Quarter",
}

export interface State {
  caption: string,
  game?: {
    armorLevel: ArmorLevel,
    arrowCapacity: number,
    arrows: number,
    bigKey: {
      desertPalace: boolean,
      easternPalace: boolean,
      gannonsTower: boolean,
      icePalace: boolean,
      miseryMire: boolean,
      palaceOfDarkness: boolean,
      skullWoods: boolean,
      swampPalace: boolean,
      thievesTown: boolean,
      towerOfHera: boolean,
      turtleRock: boolean,
    },
    blueBoomerang: boolean,
    bomb: number,
    bombCapacity: number,
    bombosMedallion: boolean,
    book: boolean,
    boots: boolean,
    bottle: boolean,
    bottleContent1: BottleContent,
    bottleContent2: BottleContent,
    bottleContent3: BottleContent,
    bottleContent4: BottleContent,
    bottleCount: number,
    bow: boolean,
    caneByrna: boolean
    caneSomaria: boolean,
    cape: boolean,
    crystal: {
      one: boolean,
      two: boolean,
      three: boolean,
      four: boolean,
      five: boolean,
      six: boolean,
      seven: boolean,
    },
    etherMedallion: boolean
    fireRod: boolean,
    flippers: boolean,
    flute: boolean,
    fluteActivated: boolean,
    gloves: Gloves,
    hammer: boolean,
    heartQuarters: number,
    hearts: number,
    hookShot: boolean,
    iceRod: boolean,
    lantern: boolean,
    magicProgression: Magic,
    maxHearts: number,
    mirror: boolean,
    moonPearl: boolean,
    mushroom: boolean,
    net: boolean,
    pendant: {
      blue: boolean,
      green: boolean,
      red: boolean,
    },
    powder: boolean,
    quakeMedallion: boolean,
    redBoomerang: boolean,
    rupees: number,
    shieldLevel: Shield,
    shovel: boolean,
    silvers: boolean,
    smallKeys: number,
    swordLevel: Sword,
  },
  dungeons: {
    name: string,
    position: {
      horizontal: {
        left: number,
        top: number,
      },
      vertical: {
        left: number,
        top: number,
      },
    },
    hoverText: string,
    totalChests: number,
    hasReward: boolean,
    dungeonCode: string,
    defaultImage: string,
    clearedImage: string,
    boss: {
      name: string,
      hoverText: string,
      imageNumber: string,
    },
    logic: string,
  }[],
  locations: {
    name: string,
    hoverText: string,
    position: {
      horizontal: {
        left: number,
        top: number,
      },
      vertical: {
        left: number,
        top: number,
      },
    },
    logic: string,
  }[],
  serverConfig: {
    apiPort: number,
    dataPollRate?: number,
    logic?: string,
    dataSource?: Qusb2snes | LocalFile
  }
}

interface Qusb2snes {
  available_devices: string[],
  selected_device: string,
  type: "Qusb2snes",
}

interface LocalFile {
  source: string,
  type: "LocalFile",
}

export const key: InjectionKey<Store<State>> = Symbol()

export const store = createStore({
  state() {
    return {
      caption: "",
      dungeons: [],
      locations: [],
      serverConfig: {
        apiPort: 8000,
      },
    }
  },
  actions: {
    setItemState({ commit }, state) {
      commit('setItemState', state)
    },
    setDungeonState({ commit }, state) {
      commit('setDungeonState', state)
    },
    setLocationState({ commit }, state) {
      commit('setLocationState', state)
    },
  
    updateDungeonState({ commit }, data) {
      commit('updateDungeonState', data)
    },
  
    updateLocationState({ commit }, data) {
      commit('updateLocationState', data)
    },
    updateCaption({ commit }, data) {
      commit('updateCaption', data)
    }  
  },
  getters: {
    getDungeonByDungeonCode: (state: any) => (code: String) => {
      return state.dungeons.find(function (dungeon: any) {
        return dungeon.dungeonCode === code
      })
    },
    mappableLocations: (state) => {
      return state.locations.filter(function (location: any) {
        return !!location.position
      })
    },
    mappableDungeons: (state) => {
      return state.dungeons.filter(function (dungeon: any) {
        return !!dungeon.position
      })
    }  
  },
  mutations: {
    setItemState(state, serverData) {
      state.game = serverData
    },
  
    setDungeonState(state, dungeonData) {
      state.dungeons = dungeonData
    },
  
    setLocationState(state, locationData) {
      state.locations = locationData
    },
  
    updateCaption(state, data) {
      state.caption = data
    },
  
    async fetchServerConfig(state, apiPort) {
      let host =
        typeof apiPort === 'undefined'
          ? window.location.host
          : window.location.hostname + ':' + apiPort
  
      await fetch('http://' + host + '/api/config', { headers: { 'Content-Type': 'application/json' } })
        .then(response => response.json())
        .then(data => {
          window.console.log('Storing server config: ', data)
          state.serverConfig = data
        })
    }  
  }
})

export function useStore () {
  return baseUseStore(key)
}

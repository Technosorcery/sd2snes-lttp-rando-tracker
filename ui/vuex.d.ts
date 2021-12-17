// vuex.d.ts
import { Store } from 'vuex'

declare module '@vue/runtime-core' {
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
  
  interface State {
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
    }
  }
  
  // provide typings for `this.$store`
  interface ComponentCustomProperties {
    $store: Store<State>
  }
}

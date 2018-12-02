function Availability(glitchless = 'unavailable', owGlitches = 'unavailable', majorGlitches = 'unavailable', inverted = 'unavailable') {
    this._glitchless = glitchless;
    this._owGlitches = owGlitches;
    this._majorGlitches = majorGlitches;
    this._inverted = inverted;

    this.getClassName = function () {
        return this[trackerOptions.mapLogic];
    }
}

Object.defineProperty(Availability.prototype, 'glitchless', {
    get: function () {
        return this._glitchless;
    },
    set: function (value) {
        this._glitchless = value;
        this._owGlitches = value;
        this._majorGlitches = value;
    }
});

Object.defineProperty(Availability.prototype, 'owGlitches', {
    get: function () {
        return this._owGlitches;
    },
    set: function (value) {
        this._owGlitches = value;
        this._majorGlitches = value;
    }
});

Object.defineProperty(Availability.prototype, 'majorGlitches', {
    get: function () {
        return this._majorGlitches;
    },
    set: function (value) {
        this._majorGlitches = value;
    }
});

Object.defineProperty(Availability.prototype, 'inverted', {
    get: function () {
        return this._inverted;
    },
    set: function (value) {
        this._inverted = value;
    }
});

// Helper functions to simplify logic.
function canLiftRocks() {
    return trackerData.items.glove >= 1;
}

function canLiftDarkRocks() {
    return trackerData.items.glove === 2;
}

function canLightTorches() {
    return trackerData.items.firerod || trackerData.items.lantern;
}

function canMeltThings() {
    return trackerData.items.firerod || (trackerData.items.bombos && trackerData.items.sword >= 1);
}

function canFly(logic = 'glitchless') {
    return trackerData.items.flute && (logic !== 'inverted' || (canEnterLightWorld(logic, false, false) && trackerData.items.moonpearl));
}

function canSpinSpeed() {
    return trackerData.items.boots && (trackerData.items.sword >= 1 || trackerData.items.hookshot);
}

function canShootArrows() {
    return trackerData.items.bow;
}

function canBlockLasers() {
    return trackerData.items.shield === 3;
}

function canExtendMagic() {
    return trackerData.items.mpupgrade >= 1 || trackerData.items.bottle >= 1;
}

function glitchedLinkInDarkWorld() {
    return trackerData.items.moonpearl || trackerData.items.bottle >= 1;
}

function canGoBeatAgahnim1(allowOutOfLogicGlitches, logic = 'glitchless') {
    if (logic === 'inverted') {
        return !trackerData.items.agahnim
            && (trackerData.items.lantern || allowOutOfLogicGlitches)
            && trackerData.items.sword >= 1
            && canEnterDarkWorldDeathMountain('inverted', allowOutOfLogicGlitches);
    }
    else {
        return !trackerData.items.agahnim
            && (trackerData.items.lantern || allowOutOfLogicGlitches)
            && (trackerData.items.cape || trackerData.items.sword >= 2)
            && trackerData.items.sword >= 1;
    }
}

function canEnterLightWorld(logic, agahnimCheck, allowOutOfLogicGlitches) {
    return logic !== 'inverted'
            || (trackerData.items.agahnim
                || (agahnimCheck && canGoBeatAgahnim1(allowOutOfLogicGlitches, 'inverted'))
                || ((canLiftDarkRocks() || (trackerData.items.hammer && canLiftRocks())) && trackerData.items.moonpearl));
}

function canEnterNorthEastDarkWorld(logic, agahnimCheck, allowOutOfLogicGlitches) {
    if (logic === 'majorGlitches') {
        return trackerData.items.agahnim
                || (agahnimCheck && canGoBeatAgahnim1(allowOutOfLogicGlitches))
                || (trackerData.items.moonpearl
                        && ((canLiftDarkRocks() && (trackerData.items.boots || trackerData.items.flippers))
                                || (trackerData.items.hammer && canLiftRocks())))
                || (canEnterWestDeathMountain(logic, allowOutOfLogicGlitches)
                        && (trackerData.items.bottle >= 1
                                || (trackerData.items.mirror && canSpinSpeed())
                                || (trackerData.items.moonpearl && (trackerData.items.mirror || trackerData.items.boots))));
    }
    else if (logic === 'owGlitches') {
        return trackerData.items.agahnim
                || (agahnimCheck && canGoBeatAgahnim1(allowOutOfLogicGlitches))
                || (trackerData.items.moonpearl
                        && ((canLiftDarkRocks() && (trackerData.items.boots || trackerData.items.flippers))
                                || (trackerData.items.hammer && canLiftRocks())))
                || (canEnterWestDeathMountain(logic, allowOutOfLogicGlitches)
                        && ((trackerData.items.mirror && canSpinSpeed())
                                || (trackerData.items.moonpearl && (trackerData.items.mirror || trackerData.items.boots))));
    }
    else if (logic === 'glitchless') {
        return trackerData.items.agahnim
                || (agahnimCheck && canGoBeatAgahnim1(allowOutOfLogicGlitches))
                || (trackerData.items.hammer && canLiftRocks() && trackerData.items.moonpearl)
                || (canLiftDarkRocks() && trackerData.items.flippers && trackerData.items.moonpearl);
    }
    else if (logic === 'inverted') {
        return trackerData.items.flippers || trackerData.items.hammer || canFly('inverted')
                || (canEnterLightWorld(logic, agahnimCheck, allowOutOfLogicGlitches) && trackerData.items.mirror)
                || (allowOutOfLogicGlitches && trackerData.items.boots);
    }
}

function canEnterNorthWestDarkWorld(logic, agahnimCheck, allowOutOfLogicGlitches) {
    if (logic === 'majorGlitches') {
        return canEnterWestDeathMountain('majorGlitches', allowOutOfLogicGlitches)
                || (trackerData.items.moonpearl
                        && (canLiftDarkRocks()
                                || (trackerData.items.hammer && canLiftRocks())
                                || ((trackerData.items.agahnim || (agahnimCheck && canGoBeatAgahnim1(allowOutOfLogicGlitches)))
                                        && trackerData.items.hookshot
                                        && (trackerData.items.hammer || canLiftRocks() || trackerData.items.flippers))));
    }
    else if (logic === 'owGlitches') {
        return canEnterWestDeathMountain('owGlitches', allowOutOfLogicGlitches) && (trackerData.items.mirror || (trackerData.items.boots && trackerData.items.moonpearl))
                || (trackerData.items.moonpearl
                        && (canLiftDarkRocks()
                                || (trackerData.items.hammer && canLiftRocks())
                                || ((trackerData.items.agahnim || (agahnimCheck && canGoBeatAgahnim1(allowOutOfLogicGlitches)))
                                        && trackerData.items.hookshot
                                        && (trackerData.items.hammer || canLiftRocks() || trackerData.items.flippers))));
    }
    else if (logic === 'glitchless') {
        return trackerData.items.moonpearl
                && ((canEnterNorthEastDarkWorld('glitchless', agahnimCheck, allowOutOfLogicGlitches) && (trackerData.items.hookshot && (trackerData.items.flippers || canLiftRocks() || trackerData.items.hammer)))
                        || (trackerData.items.hammer && canLiftRocks())
                        || canLiftDarkRocks());
    }
    else if (logic === 'inverted') {
        return true;
    }
}

function canEnterSouthDarkWorld(logic, agahnimCheck, allowOutOfLogicGlitches) {
    if (logic === 'majorGlitches') {
        return canEnterWestDeathMountain(logic, allowOutOfLogicGlitches)
                || (trackerData.items.moonpearl
                        && (canLiftDarkRocks()
                                || (trackerData.items.hammer && canLiftRocks())
                                || ((trackerData.items.agahnim || (agahnimCheck && canGoBeatAgahnim1(allowOutOfLogicGlitches)))
                                        && (trackerData.items.hammer || (trackerData.items.hookshot && (trackerData.items.flippers || canLiftRocks()))))));
    }
    else if (logic === 'owGlitches') {
        return (canEnterWestDeathMountain(logic, allowOutOfLogicGlitches) && (trackerData.items.mirror || (trackerData.items.boots && trackerData.items.moonpearl)))
                || (trackerData.items.moonpearl
                        && (canLiftDarkRocks()
                                || (trackerData.items.hammer && canLiftRocks())
                                || ((trackerData.items.agahnim || (agahnimCheck && canGoBeatAgahnim1(allowOutOfLogicGlitches)))
                                        && (trackerData.items.hammer || (trackerData.items.hookshot && (trackerData.items.flippers || canLiftRocks()))))));
    }
    else if (logic === 'glitchless') {
        return trackerData.items.moonpearl
                && (canLiftDarkRocks()
                        || (trackerData.items.hammer && canLiftRocks())
                        || (canEnterNorthEastDarkWorld('glitchless', agahnimCheck, allowOutOfLogicGlitches)
                                && (trackerData.items.hammer
                                        || (trackerData.items.hookshot && (trackerData.items.flippers || canLiftRocks())))));
    }
    else if (logic === 'inverted') {
        return true;
    }
}

function canEnterMireArea(logic, agahnimCheck, allowOutOfLogicGlitches) {
    if (logic === 'majorGlitches') {
        return (trackerData.items.bottle && canEnterWestDeathMountain('majorGlitches', allowOutOfLogicGlitches))
                || (canLiftDarkRocks() && (canFly() || trackerData.items.bottle || trackerData.items.boots))
                || (glitchedLinkInDarkWorld() && trackerData.items.boots && canEnterSouthDarkWorld('majorGlitches', agahnimCheck, allowOutOfLogicGlitches));
    }
    else if (logic === 'owGlitches') {
        return (canLiftDarkRocks() && (canFly() || trackerData.items.boots))
                || (trackerData.items.moonpearl && trackerData.items.boots && canEnterSouthDarkWorld('owGlitches', agahnimCheck, allowOutOfLogicGlitches));
    }
    else if (logic === 'glitchless') {
        return canFly() && canLiftDarkRocks();
    }
    else if (logic === 'inverted') {
        return canFly('inverted') || (canEnterLightWorld(logic, agahnimCheck, allowOutOfLogicGlitches) && trackerData.items.mirror);
    }
}

function canEnterWestDeathMountain(logic, allowOutOfLogicGlitches) {
    if (logic === 'majorGlitches') {
        return trackerData.items.boots
                || trackerData.items.bottle >= 1
                || canFly()
                || (canLiftRocks() && (trackerData.items.lantern || allowOutOfLogicGlitches));
    }
    else if (logic === 'owGlitches') {
        return trackerData.items.boots
                || canFly()
                || (canLiftRocks() && (trackerData.items.lantern || allowOutOfLogicGlitches));
    }
    else if (logic === 'glitchless') {
        return canFly()
                || (canLiftRocks() && (trackerData.items.lantern || allowOutOfLogicGlitches));
    }
    else if (logic === 'inverted') {
        return canEnterDarkWorldDeathMountain('inverted', allowOutOfLogicGlitches);
    }
}

function canEnterEastDeathMountain(logic, allowOutOfLogicGlitches) {
    if (logic === 'majorGlitches') {
        return trackerData.items.boots
                || (canEnterWestDeathMountain('majorGlitches', allowOutOfLogicGlitches) && (trackerData.items.hookshot || trackerData.items.mirror));
    }
    else if (logic === 'owGlitches') {
        return trackerData.items.boots
                || (canEnterWestDeathMountain('owGlitches', allowOutOfLogicGlitches) && (trackerData.items.hookshot || (trackerData.items.mirror && trackerData.items.hammer)));
    }
    else if (logic === 'glitchless') {
        return canEnterWestDeathMountain('glitchless', allowOutOfLogicGlitches) && (trackerData.items.hookshot || (trackerData.items.mirror && trackerData.items.hammer));
    }
    else if (logic === 'inverted') {
        return (canEnterWestDeathMountain('inverted', allowOutOfLogicGlitches) && trackerData.items.hookshot && trackerData.items.moonpearl)
                || (canEnterDarkWorldDeathMountain('inverted', allowOutOfLogicGlitches) && canLiftDarkRocks());
    }
}

function canEnterDarkWorldDeathMountain(logic, allowOutOfLogicGlitches) {
    if (logic === 'majorGlitches') {
        return trackerData.items.moonpearl
                || (trackerData.items.bottle >= 1 && trackerData.items.boots)
                || ((canLiftDarkRocks() || (trackerData.items.hammer && trackerData.items.boots)) && canEnterEastDeathMountain('majorGlitches', allowOutOfLogicGlitches))
                || (trackerData.items.mirror && canEnterWestDeathMountain('majorGlitches', allowOutOfLogicGlitches));
    }
    else if (logic === 'owGlitches') {
        return (trackerData.items.moonpearl && trackerData.items.boots)
                || ((canLiftDarkRocks() || (trackerData.items.hammer && trackerData.items.boots))
                        && canEnterEastDeathMountain('owGlitches', allowOutOfLogicGlitches));
    }
    else if (logic === 'glitchless') {
        return canLiftDarkRocks() && canEnterEastDeathMountain('glitchless', allowOutOfLogicGlitches);
    }
    else if (logic === 'inverted') {
        return canFly('inverted')
                || (canLiftRocks() && (trackerData.items.lantern || allowOutOfLogicGlitches));
    }
}

// define dungeon chests
const dungeons = [];

dungeons[0] = {
    name: "Eastern Palace",
    label: "EP",
    x: "46.8%",
    y: "38.8%",
    image: "boss02.png",
    canEnter: function (logic, agahnimCheck, allowOutOfLogicGlitches) {
        return logic !== 'inverted' || canEnterLightWorld(logic, agahnimCheck, allowOutOfLogicGlitches);
    },
    isBeatable: function() {
        const availability = new Availability();
        if (trackerData.items.bow) {
            if (trackerData.items.lantern) {
                availability.glitchless = 'available';
            }
            else {
                availability.glitchless = 'glitchavailable';
            }
            if (this.canEnter('inverted', false, true)) {
                if (trackerData.items.moonpearl && this.canEnter('inverted', false, false) && trackerData.items.lantern) {
                    availability.inverted = 'available';
                }
                else {
                    availability.inverted = 'glitchavailable';
                }
            }
            else if (this.canEnter('inverted', true, true)) {
                if (trackerData.items.moonpearl && this.canEnter('inverted', true, false) && trackerData.items.lantern) {
                    availability.inverted = 'agahnim';
                }
                else {
                    availability.inverted = 'glitchagahnim';
                }
            }
        }
        return availability;
    },
    canGetChest: function () {
        const availability = new Availability();
        if (trackerData.items.lantern) {
            if (trackerData.items.bow) {
                availability.glitchless = 'available';
            }
            else if (trackerData.dungeonchests[0] >= 2) {
                availability.glitchless = 'available';
            }
            else {
                availability.glitchless = 'possible';
            }
        }
        else if (trackerData.dungeonchests[0] === 3) {
            availability.glitchless = 'available';
        }
        else {
            availability.glitchless = 'possible';
        }
        if (this.canEnter('inverted', false, true)) {
            if (trackerData.items.moonpearl && this.canEnter('inverted', false, false) && trackerData.items.lantern) {
                if (trackerData.items.bow || trackerData.dungeonchests[0] >= 2) {
                    availability.inverted = 'available';
                }
                else {
                    availability.inverted = 'possible';
                }
            }
            else {
                if (trackerData.items.bow || trackerData.dungeonchests[0] >= 2) {
                    availability.inverted = 'glitchavailable';
                }
                else {
                    availability.inverted = 'glitchpossible';
                }
            }
        }
        else if (this.canEnter('inverted', true, true)) {
            if (trackerData.items.moonpearl && this.canEnter('inverted', true, false) && trackerData.items.lantern) {
                availability.inverted = 'agahnim';
            }
            else {
                availability.inverted = 'glitchagahnim';
            }
        }
        return availability;
    }
};

dungeons[1] = {
    name: "Desert Palace",
    label: "DP",
    x: "3.8%",
    y: "78.4%",
    image: "boss12.png",
    canEnter: function (logic, agahnimCheck, allowOutOfLogicGlitches) {
        if (logic === 'majorGlitches') {
            return trackerData.items.book
                    || trackerData.items.boots
                    || (trackerData.items.mirror && canEnterMireArea('majorGlitches', agahnimCheck, allowOutOfLogicGlitches));
        }
        else if (logic === 'owGlitches') {
            return trackerData.items.book
                    || trackerData.items.boots
                    || (trackerData.items.mirror && canEnterMireArea('owGlitches', agahnimCheck, allowOutOfLogicGlitches));
        }
        else if (logic === 'glitchless') {
            return trackerData.items.book
                    || (trackerData.items.mirror && canLiftDarkRocks() && canFly());
        }
        else if (logic === 'inverted') {
            return trackerData.items.book && canEnterLightWorld('inverted', agahnimCheck, allowOutOfLogicGlitches);
        }
    },
    canHurtBoss: function() {
        return trackerData.items.sword
                || trackerData.items.hammer
                || trackerData.items.bow
                || trackerData.items.firerod
                || trackerData.items.icerod
                || trackerData.items.byrna
                || trackerData.items.somaria
    },
    isBeatable: function () {
        const availability = new Availability();
        if (canLiftRocks() && canLightTorches()) {
            if (this.canEnter('glitchless', false, false)) {
                if (trackerData.items.boots) {
                    if (this.canHurtBoss()) {
                        availability.glitchless = 'available';
                    }
                    else {
                        availability.glitchless = 'glitchavailable';
                    }
                }
                else if (this.canHurtBoss()) {
                    availability.glitchless = 'possible';
                }
                else {
                    availability.glitchless = 'glitchpossible';
                }
            }
            if (this.canEnter('owGlitches', false, false)) {
                if (trackerData.items.boots) {
                    if (this.canHurtBoss()) {
                        availability.owGlitches = 'available';
                    }
                    else {
                        availability.owGlitches = 'glitchavailable';
                    }
                }
                else if (this.canHurtBoss()) {
                    availability.owGlitches = 'possible';
                }
                else {
                    availability.owGlitches = 'glitchpossible';
                }
            }
            else if (this.canEnter('owGlitches', true, false)) {
                if (this.canHurtBoss()) {
                    availability.owGlitches = 'agahnim';
                }
                else {
                    availability.owGlitches = 'glitchagahnim';
                }
            }
            else if (this.canEnter('owGlitches', true, false)) {
                availability.owGlitches = 'glitchagahnim';
            }
            if (this.canEnter('majorGlitches', false, false)) {
                if (trackerData.items.boots) {
                    if (this.canHurtBoss()) {
                        availability.majorGlitches = 'available';
                    }
                    else {
                        availability.majorGlitches = 'glitchavailable';
                    }
                }
                else if (this.canHurtBoss()) {
                    availability.majorGlitches = 'possible';
                }
                else {
                    availability.majorGlitches = 'glitchpossible';
                }
            }
            else if (this.canEnter('majorGlitches', true, false)) {
                if (this.canHurtBoss()) {
                    availability.majorGlitches = 'agahnim';
                }
                else {
                    availability.majorGlitches = 'glitchagahnim';
                }
            }
            else if (this.canEnter('majorGlitches', true, false)) {
                availability.majorGlitches = 'glitchagahnim';
            }
            if (this.canEnter('inverted', false, true) && trackerData.items.moonpearl) {
                if (this.canEnter('inverted', false, false) && this.canHurtBoss()) {
                    if (trackerData.items.boots) {
                        availability.inverted = 'available';
                    }
                    else {
                        availability.inverted = 'possible';
                    }
                }
                else {
                    if (trackerData.items.boots) {
                        availability.inverted = 'glitchavailable';
                    }
                    else {
                        availability.inverted = 'glitchpossible';
                    }
                }
            }
            else if (this.canEnter('inverted', true, true) && trackerData.items.moonpearl) {
                if (this.canEnter('inverted', true, false) && this.canHurtBoss()) {
                    availability.inverted = 'agahnim';
                }
                else {
                    availability.inverted = 'glitchagahnim';
                }
            }
        }
        return availability;
    },
    canGetChest: function () {
        const availability = new Availability();
        if (this.canEnter('glitchless', false, false)) {
            if (trackerData.items.boots && (trackerData.dungeonchests[1] === 2 || (this.canHurtBoss() && canLightTorches() && canLiftRocks()))) {
                availability.glitchless = 'available';
            }
            else {
                availability.glitchless = 'possible';
            }
        }
        if (this.canEnter('owGlitches', false, false)) {
            if (trackerData.items.boots && (trackerData.dungeonchests[1] === 2 || (this.canHurtBoss() && canLightTorches() && canLiftRocks()))) {
                availability.owGlitches = 'available';
            }
            else {
                availability.owGlitches = 'possible';
            }
        }
        else if (this.canEnter('owGlitches', true, false)) {
            availability.owGlitches = 'agahnim';
        }
        else if (this.canEnter('owGlitches', true, true)) {
            availability.owGlitches = 'glitchedagahnim';
        }
        if (this.canEnter('majorGlitches', false, false)) {
            if (trackerData.items.boots && (trackerData.dungeonchests[1] === 2 || (this.canHurtBoss() && canLightTorches() && canLiftRocks()))) {
                availability.majorGlitches = 'available';
            }
            else {
                availability.majorGlitches = 'possible';
            }
        }
        else if (this.canEnter('majorGlitches', true, false)) {
            availability.majorGlitches = 'agahnim';
        }
        else if (this.canEnter('majorGlitches', true, true)) {
            availability.majorGlitches = 'glitchedagahnim';
        }
        if (this.canEnter('inverted', false, true)) {
            if (trackerData.items.moonpearl && this.canEnter('inverted', false, false)) {
                if (trackerData.items.boots && (trackerData.dungeonchests[1] === 2 || (this.canHurtBoss() && canLightTorches() && canLiftRocks()))) {
                    availability.inverted = 'available';
                }
                else {
                    availability.inverted = 'possible';
                }
            }
            else {
                if (trackerData.items.boots && (trackerData.dungeonchests[1] === 2 || (this.canHurtBoss() && canLightTorches() && canLiftRocks()))) {
                    availability.inverted = 'glitchavailable';
                }
                else {
                    availability.inverted = 'glitchpossible';
                }
            }
        }
        else if (this.canEnter('inverted', true, true)) {
            if (trackerData.items.moonpearl && this.canEnter('inverted', true, false)) {
                availability.inverted = 'agahnim';
            }
            else {
                availability.inverted = 'glitchagahnim';
            }
        }
        return availability;
    }
};

dungeons[2] = {
    name: "Tower of Hera",
    label: "ToH",
    x: "31.0%",
    y: "5.5%",
    image: "boss22.png",
    canEnter: function (logic, agahnimCheck, allowOutOfLogicGlitches) {
        if (logic === 'majorGlitches') {
            return trackerData.items.boots
                    || (canEnterWestDeathMountain('majorGlitches', allowOutOfLogicGlitches)
                            && (trackerData.items.mirror || (trackerData.items.hookshot && trackerData.items.hammer)))
                    // Enter from Misery Mire.
                    || (dungeons[8].canEnter('majorGlitches', agahnimCheck, allowOutOfLogicGlitches));
        }
        else if (logic === 'owGlitches') {
            return trackerData.items.boots
                    || (canEnterWestDeathMountain('owGlitches', allowOutOfLogicGlitches)
                            && (trackerData.items.mirror || (trackerData.items.hookshot && trackerData.items.hammer)));
        }
        else if (logic === 'glitchless') {
            return canEnterWestDeathMountain('glitchless', allowOutOfLogicGlitches)
                    && (trackerData.items.mirror || (trackerData.items.hookshot && trackerData.items.hammer));
        }
        else if (logic === 'inverted') {
            return canEnterEastDeathMountain('inverted', allowOutOfLogicGlitches) && trackerData.items.hammer && trackerData.items.moonpearl;
        }
    },
    mayEnter: function (logic, agahnimCheck, allowOutOfLogicGlitches) {
        return (logic === 'majorGlitches' && dungeons[8].mayEnter('majorGlitches', agahnimCheck, allowOutOfLogicGlitches)) || this.canEnter(logic);
    },
    isBeatable: function () {
        const availability = new Availability();
        if (trackerData.items.sword >= 1 || trackerData.items.hammer) {
            if (this.canEnter('glitchless', false, false)) {
                if (canLightTorches()) {
                    availability.glitchless = 'available';
                }
                else {
                    availability.glitchless = 'possible';
                }
            }
            else if (this.canEnter('glitchless', false, true)) {
                if (canLightTorches()) {
                    availability.glitchless = 'glitchavailable';
                }
                else {
                    availability.glitchless = 'glitchpossible';
                }
            }
            if (this.canEnter('owGlitches', false, false)) {
                if (canLightTorches()) {
                    availability.owGlitches = 'available';
                }
                else {
                    availability.owGlitches = 'possible';
                }
            }
            else if (this.canEnter('owGlitches', false, true)) {
                if (canLightTorches()) {
                    availability.owGlitches = 'glitchavailable';
                }
                else {
                    availability.owGlitches = 'glitchpossible';
                }
            }
            if (this.canEnter('majorGlitches', false, false)) {
                if (canLightTorches() || dungeons[8].canEnter('majorGlitches', false, false)) {
                    availability.majorGlitches = 'available';
                }
                else {
                    availability.majorGlitches = 'possible';
                }
            }
            else if (this.canEnter('majorGlitches', false, true)) {
                if (canLightTorches() || dungeons[8].canEnter('majorGlitches', false, true)) {
                    availability.majorGlitches = 'glitchavailable';
                }
                else {
                    availability.majorGlitches = 'glitchpossible';
                }
            }
            else if (this.mayEnter('majorGlitches', false, false)) {
                availability.majorGlitches = 'possible';
            }
            else if (this.mayEnter('majorGlitches', false, true)) {
                availability.majorGlitches = 'glitchpossible';
            }
            else if (this.mayEnter('majorGlitches', true, false)) {
                availability.majorGlitches = 'agahnim';
            }
            else if (this.mayEnter('majorGlitches', true, true)) {
                availability.majorGlitches = 'glitchagahnim';
            }
            if (this.canEnter('inverted', false, false)) {
                if (canLightTorches()) {
                    availability.inverted = 'available';
                }
                else {
                    availability.inverted = 'possible';
                }
            }
            else if (this.canEnter('inverted', false, true)) {
                if (canLightTorches()) {
                    availability.inverted = 'glitchavailable';
                }
                else {
                    availability.inverted = 'glitchpossible';
                }
            }
        }
        return availability;
    },
    canGetChest: function () {
        const availability = new Availability();
        if (this.canEnter('glitchless', false, false)) {
            if (canLightTorches() && (trackerData.dungeonchests[2] === 2 || trackerData.items.sword >= 1 || trackerData.items.hammer)) {
                availability.glitchless = 'available';
            }
            else {
                availability.glitchless = 'possible';
            }
        }
        else if (this.canEnter('glitchless', false, true)) {
            if (canLightTorches() && (trackerData.dungeonchests[2] === 2 || trackerData.items.sword >= 1 || trackerData.items.hammer)) {
                availability.glitchless = 'glitchavailable';
            }
            else {
                availability.glitchless = 'glitchpossible';
            }
        }
        if (this.canEnter('owGlitches', false, false)) {
            if (canLightTorches() && (trackerData.dungeonchests[2] === 2 || trackerData.items.sword >= 1 || trackerData.items.hammer)) {
                availability.owGlitches = 'available';
            }
            else {
                availability.owGlitches = 'possible';
            }
        }
        else if (this.canEnter('owGlitches', false, true)) {
            if (canLightTorches() && (trackerData.dungeonchests[2] === 2 || trackerData.items.sword >= 1 || trackerData.items.hammer)) {
                availability.owGlitches = 'glitchavailable';
            }
            else {
                availability.owGlitches = 'glitchpossible';
            }
        }
        if (this.canEnter('majorGlitches', false, false)) {
            if ((canLightTorches() || dungeons[8].canEnter('majorGlitches', false, false))
                    && (trackerData.dungeonchests[2] === 2 || trackerData.items.sword >= 1 || trackerData.items.hammer)) {
                availability.majorGlitches = 'available';
            }
            else {
                availability.majorGlitches = 'possible';
            }
        }
        else if (this.canEnter('majorGlitches', false, true)) {
            if ((canLightTorches() || dungeons[8].canEnter('majorGlitches', false, false))
                    && (trackerData.dungeonchests[2] === 2 || trackerData.items.sword >= 1 || trackerData.items.hammer)) {
                availability.majorGlitches = 'glitchavailable';
            }
            else {
                availability.majorGlitches = 'glitchpossible';
            }
        }
        else if (this.mayEnter('majorGlitches', false, false)) {
            availability.majorGlitches = 'possible';
        }
        else if (this.mayEnter('majorGlitches', false, true)) {
            availability.majorGlitches = 'glitchpossible';
        }
        else if (this.mayEnter('majorGlitches', true, false)) {
            availability.majorGlitches = 'agahnim';
        }
        else if (this.mayEnter('majorGlitches', true, true)) {
            availability.majorGlitches = 'glitchagahnim';
        }
        if (this.canEnter('inverted', false, false)) {
            if (canLightTorches() && (trackerData.dungeonchests[2] === 2 || trackerData.items.sword >= 1 || trackerData.items.hammer)) {
                availability.inverted = 'available';
            }
            else {
                availability.inverted = 'possible';
            }
        }
        else if (this.canEnter('inverted', false, true)) {
            if (canLightTorches() && (trackerData.dungeonchests[2] === 2 || trackerData.items.sword >= 1 || trackerData.items.hammer)) {
                availability.inverted = 'glitchavailable';
            }
            else {
                availability.inverted = 'glitchpossible';
            }
        }
        return availability;
    }
};

dungeons[3] = {
    name: "Palace of Darkness <img src='/images/lantern.png' class='mini'>",
    label: "PoD",
    x: "97.0%",
    y: "40.0%",
    image: "boss32.png",
    canEnter: function (logic, agahnimCheck, allowOutOfLogicGlitches) {
        if (logic === 'majorGlitches') {
            return (glitchedLinkInDarkWorld() && canEnterNorthEastDarkWorld('majorGlitches', agahnimCheck, allowOutOfLogicGlitches))
                    || (canEnterWestDeathMountain('majorGlitches', agahnimCheck, allowOutOfLogicGlitches));
        }
        else if (logic === 'glitchless' || logic === 'owGlitches') {
            return canEnterNorthEastDarkWorld(logic, agahnimCheck, allowOutOfLogicGlitches) && trackerData.items.moonpearl;
        }
        else if (logic === 'inverted') {
            return canEnterNorthEastDarkWorld('inverted', agahnimCheck, allowOutOfLogicGlitches);
        }
    },
    isBeatable: function () {
        const availability = new Availability();
        if (trackerData.items.hammer && trackerData.items.bow) {
            if (this.canEnter('glitchless', false, false) && trackerData.items.lantern) {
                availability.glitchless = 'available';
            }
            else if (this.canEnter('glitchless', false, true)) {
                availability.glitchless = 'glitchavailable';
            }
            else if (this.canEnter('glitchless', true, false) && trackerData.items.lantern) {
                availability.glitchless = 'agahnim';
            }
            else if (this.canEnter('glitchless', true, true)) {
                availability.glitchless = 'glitchedagahnim';
            }
            if (this.canEnter('owGlitches', false, false) && trackerData.items.lantern) {
                availability.owGlitches = 'available';
            }
            else if (this.canEnter('owGlitches', false, true)) {
                availability.owGlitches = 'glitchavailable';
            }
            else if (this.canEnter('owGlitches', true, false) && trackerData.items.lantern) {
                availability.owGlitches = 'agahnim';
            }
            else if (this.canEnter('owGlitches', true, true)) {
                availability.owGlitches = 'glitchedagahnim';
            }
            if (this.canEnter('majorGlitches', false, false) && trackerData.items.lantern) {
                availability.majorGlitches = 'available';
            }
            else if (this.canEnter('majorGlitches', false, true)) {
                availability.majorGlitches = 'glitchavailable';
            }
            else if (this.canEnter('majorGlitches', true, false) && trackerData.items.lantern) {
                availability.majorGlitches = 'agahnim';
            }
            else if (this.canEnter('majorGlitches', true, true)) {
                availability.majorGlitches = 'glitchedagahnim';
            }
            if (this.canEnter('inverted', false, false) && trackerData.items.lantern) {
                availability.inverted = 'available';
            }
            else if (this.canEnter('inverted', false, true)) {
                availability.inverted = 'glitchavailable';
            }
            else if (this.canEnter('inverted', true, false) && trackerData.items.lantern) {
                availability.inverted = 'agahnim';
            }
            else if (this.canEnter('inverted', true, true)) {
                availability.inverted = 'glitchedagahnim';
            }
        }
        return availability;
    },
    canGetChest: function () {
        const availability = new Availability();
        if (this.canEnter('glitchless', false, false)) {
            if (trackerData.items.bow && (trackerData.dungeonchests[3] >= 2 || trackerData.items.hammer)) {
                if (trackerData.items.lantern) {
                    availability.glitchless = 'available';
                }
                else {
                    availability.glitchless = 'possible';
                }
            }
            else if (trackerData.items.lantern) {
                availability.glitchless = 'possible';
            }
            else {
                availability.glitchless = 'glitchpossible';
            }
        }
        else if (this.canEnter('glitchless', true, false)) {
            availability.glitchless = 'agahnim';
        }
        else if (this.canEnter('glitchless', true, true)) {
            availability.glitchless = 'glitchagahnim';
        }
        if (this.canEnter('owGlitches', false, false)) {
            if (trackerData.items.bow && (trackerData.dungeonchests[3] >= 2 || trackerData.items.hammer)) {
                if (trackerData.items.lantern) {
                    availability.owGlitches = 'available';
                }
                else {
                    availability.owGlitches = 'possible';
                }
            }
            else if (trackerData.items.lantern) {
                availability.owGlitches = 'possible';
            }
            else {
                availability.owGlitches = 'glitchpossible';
            }
        }
        else if (this.canEnter('owGlitches', false, true)) {
            if (trackerData.items.bow && (trackerData.dungeonchests[3] >= 2 || trackerData.items.hammer)) {
                availability.owGlitches = 'glitchavailable';
            }
            else {
                availability.owGlitches = 'glitchpossible';
            }
        }
        else if (this.canEnter('owGlitches', true, false)) {
            availability.owGlitches = 'agahnim';
        }
        else if (this.canEnter('owGlitches', true, true)) {
            availability.owGlitches = 'glitchagahnim';
        }
        if (this.canEnter('majorGlitches', false, false)) {
            if (trackerData.items.bow && (trackerData.dungeonchests[3] >= 2 || trackerData.items.hammer)) {
                if (trackerData.items.lantern) {
                    availability.majorGlitches = 'available';
                }
                else {
                    availability.majorGlitches = 'possible';
                }
            }
            else if (trackerData.items.lantern) {
                availability.majorGlitches = 'possible';
            }
            else {
                availability.majorGlitches = 'glitchpossible';
            }
        }
        else if (this.canEnter('majorGlitches', false, true)) {
            if (trackerData.items.bow && (trackerData.dungeonchests[3] >= 2 || trackerData.items.hammer)) {
                availability.majorGlitches = 'glitchavailable';
            }
            else {
                availability.majorGlitches = 'glitchpossible';
            }
        }
        else if (this.canEnter('majorGlitches', true, false)) {
            availability.majorGlitches = 'agahnim';
        }
        else if (this.canEnter('majorGlitches', true, true)) {
            availability.majorGlitches = 'glitchagahnim';
        }
        if (this.canEnter('inverted', false, false)) {
            if (trackerData.items.bow && (trackerData.dungeonchests[3] >= 2 || trackerData.items.hammer)) {
                if (trackerData.items.lantern) {
                    availability.inverted = 'available';
                }
                else {
                    availability.inverted = 'possible';
                }
            }
            else if (trackerData.items.lantern) {
                availability.inverted = 'possible';
            }
            else {
                availability.inverted = 'glitchpossible';
            }
        }
        else if (this.canEnter('inverted', false, true)) {
            if (trackerData.items.bow && (trackerData.dungeonchests[3] >= 2 || trackerData.items.hammer)) {
                availability.inverted = 'glitchavailable';
            }
            else {
                availability.inverted = 'glitchpossible';
            }
        }
        else if (this.canEnter('inverted', true, false)) {
            availability.inverted = 'agahnim';
        }
        else if (this.canEnter('inverted', true, true)) {
            availability.inverted = 'glitchagahnim';
        }

        return availability;
    }
};

dungeons[4] = {
    name: "Swamp Palace <img src='/images/mirror.png' class='mini'>",
    label: "SP",
    x: "73.5%",
    y: "91.0%",
    image: "boss42.png",
    canEnter: function (logic, agahnimCheck, allowOutOfLogicGlitches) {
        if (logic === 'majorGlitches') {
            return dungeons[8].canEnter('majorGlitches', agahnimCheck, allowOutOfLogicGlitches)
                    || (trackerData.items.moonpearl
                            && trackerData.items.mirror
                            && trackerData.items.flippers
                            && canEnterSouthDarkWorld('majorGlitches', agahnimCheck, allowOutOfLogicGlitches));
        }
        else if (logic === 'glitchless' || logic === 'owGlitches') {
            return (trackerData.items.moonpearl
                    && trackerData.items.mirror
                    && trackerData.items.flippers
                    && canEnterSouthDarkWorld(logic, agahnimCheck, allowOutOfLogicGlitches));
        }
        else if (logic === 'inverted') {
            return trackerData.items.mirror && trackerData.items.flippers
                    && canEnterLightWorld('inverted', agahnimCheck, allowOutOfLogicGlitches)
                    && (trackerData.items.moonpearl || allowOutOfLogicGlitches);
        }
    },
    isBeatable: function () {
        const availability = new Availability();
        if (trackerData.items.hammer && trackerData.items.hookshot) {
            if (this.canEnter('glitchless', false, false)) {
                availability.glitchless = 'available';
            }
            else if (this.canEnter('glitchless', true, false)) {
                availability.glitchless = 'agahnim';
            }
            else if (this.canEnter('glitchless', true, true)) {
                availability.glitchless = 'glitchagahnim';
            }
            if (this.canEnter('owGlitches', false, false)) {
                availability.owGlitches = 'available';
            }
            else if (this.canEnter('owGlitches', false, true)) {
                availability.owGlitches = 'glitchavailable';
            }
            else if (this.canEnter('owGlitches', true, false)) {
                availability.owGlitches = 'agahnim';
            }
            else if (this.canEnter('owGlitches', true, true)) {
                availability.owGlitches = 'glitchagahnim';
            }
            if (this.canEnter('inverted', false, false)) {
                availability.inverted = 'available';
            }
            else if (this.canEnter('inverted', false, true)) {
                availability.inverted = 'glitchavailable';
            }
            else if (this.canEnter('inverted', true, false)) {
                availability.inverted = 'agahnim';
            }
            else if (this.canEnter('inverted', true, true)) {
                availability.inverted = 'glitchagahnim';
            }
        }
        if (trackerData.items.hookshot
                && trackerData.items.flippers
                && (trackerData.items.sword >= 1
                        || trackerData.items.hammer
                        || ((trackerData.items.bow || canExtendMagic())
                                && (trackerData.items.firerod || trackerData.items.icerod)))) {
            if ((this.canEnter('majorGlitches', false, false))
                    && (trackerData.items.hammer || dungeons[8].canEnter('majorGlitches', false, false))) {
                availability.majorGlitches = 'available';
            }
            else if ((this.canEnter('majorGlitches', false, true))
                    && (trackerData.items.hammer || dungeons[8].canEnter('majorGlitches', false, true))) {
                availability.majorGlitches = 'glitchavailable';
            }
            else if ((this.canEnter('majorGlitches', true, false))
                    && (trackerData.items.hammer || dungeons[8].canEnter('majorGlitches', true, false))) {
                availability.majorGlitches = 'agahnim';
            }
            else if ((this.canEnter('majorGlitches', true, true))
                    && (trackerData.items.hammer || dungeons[8].canEnter('majorGlitches', true, true))) {
                availability.majorGlitches = 'glitchedagahnim';
            }
        }
        return availability;
    },
    canGetChest: function () {
        const availability = new Availability();
        if (this.canEnter('glitchless', false, false)) {
            if (trackerData.items.hammer) {
                if (trackerData.items.hookshot || trackerData.dungeonchests[4] >= 5) {
                    availability.glitchless = 'available';
                }
                else if (trackerData.dungeonchests[4] >= 3) {
                    availability.glitchless = 'possible';
                }
            }
            else if (trackerData.dungeonchests[4] === 6) {
                availability.glitchless = 'possible';
            }
        }
        else if (this.canEnter('glitchless', true, false)) {
            availability.glitchless = 'agahnim';
        }
        else if (this.canEnter('glitchless', true, true)) {
            availability.glitchless = 'glitchagahnim';
        }
        if (this.canEnter('owGlitches', false, false)) {
            if (trackerData.items.hammer) {
                if (trackerData.items.hookshot || trackerData.dungeonchests[4] >= 5) {
                    availability.owGlitches = 'available';
                }
                else if (trackerData.dungeonchests[4] >= 3) {
                    availability.owGlitches = 'possible';
                }
            }
            else if (trackerData.dungeonchests[4] === 6) {
                availability.owGlitches = 'possible';
            }
        }
        else if (this.canEnter('owGlitches', false, true)) {
            if (trackerData.items.hammer) {
                if (trackerData.items.hookshot || trackerData.dungeonchests[4] >= 5) {
                    availability.owGlitches = 'glitchavailable';
                }
                else if (trackerData.dungeonchests[4] >= 3) {
                    availability.owGlitches = 'glitchpossible';
                }
            }
            else if (trackerData.dungeonchests[4] === 6) {
                availability.owGlitches = 'glitchpossible';
            }
        }
        else if (this.canEnter('owGlitches', true, false)) {
            availability.owGlitches = 'agahnim';
        }
        else if (this.canEnter('owGlitches', true, true)) {
            availability.owGlitches = 'glitchagahnim';
        }
        if (this.canEnter('majorGlitches', false, false)) {
            if (trackerData.items.flippers
                    && (trackerData.items.hammer || dungeons[8].canEnter('majorGlitches', false, false))) {
                if (trackerData.items.hookshot) {
                    availability.majorGlitches = 'available';
                }
                else if (trackerData.dungeonchests[4] >= 3) {
                    availability.majorGlitches = 'possible';
                }
            }
            else if (trackerData.dungeonchests[4] >= 5) {
                availability.majorGlitches = 'possible';
            }
        }
        else if (this.canEnter('majorGlitches', false, true)) {
            if (trackerData.items.flippers
                    && (trackerData.items.hammer || dungeons[8].canEnter('majorGlitches', false, false))) {
                if (trackerData.items.hookshot) {
                    availability.majorGlitches = 'glitchavailable';
                }
                else if (trackerData.dungeonchests[4] >= 3) {
                    availability.majorGlitches = 'glitchpossible';
                }
            }
            else if (trackerData.dungeonchests[4] >= 5) {
                availability.majorGlitches = 'glitchpossible';
            }
        }
        else if (this.canEnter('majorGlitches', true, false)) {
            availability.majorGlitches = 'agahnim';
        }
        else if (this.canEnter('majorGlitches', true, true)) {
            availability.majorGlitches = 'glitchagahnim';
        }
        if (this.canEnter('inverted', false, false)) {
            if (trackerData.items.hammer) {
                if (trackerData.items.hookshot || trackerData.dungeonchests[4] >= 5) {
                    availability.inverted = 'available';
                }
                else if (trackerData.dungeonchests[4] >= 3) {
                    availability.inverted = 'possible';
                }
            }
            else if (trackerData.dungeonchests[4] === 6) {
                availability.inverted = 'possible';
            }
        }
        else if (this.canEnter('inverted', false, true)) {
            if (trackerData.items.hammer) {
                if (trackerData.items.hookshot || trackerData.dungeonchests[4] >= 5) {
                    availability.inverted = 'glitchavailable';
                }
                else if (trackerData.dungeonchests[4] >= 3) {
                    availability.inverted = 'glitchpossible';
                }
            }
            else if (trackerData.dungeonchests[4] === 6) {
                availability.inverted = 'glitchpossible';
            }
        }
        else if (this.canEnter('inverted', true, false)) {
            availability.inverted = 'agahnim';
        }
        else if (this.canEnter('inverted', true, true)) {
            availability.inverted = 'glitchagahnim';
        }
        return availability;
    }
};

dungeons[5] = {
    name: "Skull Woods",
    label: "SW",
    x: "53.3%",
    y: "5.4%",
    image: "boss52.png",
    canEnter: function (logic, agahnimCheck, allowOutOfLogicGlitches) {
        if (logic === 'majorGlitches' || logic === 'owGlitches') {
            return canEnterNorthWestDarkWorld(logic, agahnimCheck, allowOutOfLogicGlitches);
        }
        else if (logic === 'glitchless') {
            return trackerData.items.moonpearl
                    && canEnterNorthWestDarkWorld('glitchless', agahnimCheck, allowOutOfLogicGlitches)
        }
        else if (logic === 'inverted') {
            return true;
        }
    },
    isBeatable: function () {
        const availability = new Availability();
        if (trackerData.items.moonpearl && trackerData.items.firerod && trackerData.items.sword >= 1) {
            if (this.canEnter('glitchless', false, false)) {
                availability.glitchless = 'available';
            }
            else if (this.canEnter('glitchless', true, false)) {
                availability.glitchless = 'agahnim';
            }
            else if (this.canEnter('glitchless', true, true)) {
                availability.glitchless = 'glitchagahnim';
            }
            if (this.canEnter('owGlitches', false, false)) {
                availability.owGlitches = 'available';
            }
            else if (this.canEnter('owGlitches', false, true)) {
                availability.owGlitches = 'glitchavailable';
            }
            else if (this.canEnter('owGlitches', true, false)) {
                availability.owGlitches = 'agahnim';
            }
            else if (this.canEnter('owGlitches', true, true)) {
                availability.owGlitches = 'glitchagahnim';
            }
            if (this.canEnter('majorGlitches', false, false)) {
                availability.majorGlitches = 'available';
            }
            else if (this.canEnter('majorGlitches', false, true)) {
                availability.majorGlitches = 'glitchavailable';
            }
            else if (this.canEnter('majorGlitches', true, false)) {
                availability.majorGlitches = 'agahnim';
            }
            else if (this.canEnter('majorGlitches', true, true)) {
                availability.majorGlitches = 'glitchagahnim';
            }
        }
        if (trackerData.items.firerod && trackerData.items.sword >= 1) {
            availability.inverted = 'available';
        }
        return availability;
    },
    canGetChest: function () {
        const availability = new Availability();
        if (this.canEnter('glitchless', false, false)) {
            if (trackerData.items.moonpearl
                    && trackerData.items.firerod
                    && (trackerData.items.sword >= 1 || trackerData.dungeonchests[5] === 2)) {
                availability.glitchless = 'available';
            }
            else {
                availability.glitchless = 'possible';
            }
        }
        else if (this.canEnter('glitchless', true, false)) {
            availability.glitchless = 'agahnim';
        }
        else if (this.canEnter('glitchless', true, true)) {
            availability.glitchless = 'glitchagahnim';
        }
        if (this.canEnter('owGlitches', false, false)) {
            if (trackerData.items.moonpearl
                    && trackerData.items.firerod
                    && (trackerData.items.sword >= 1 || trackerData.dungeonchests[5] === 2)) {
                availability.owGlitches = 'available';
            }
            else {
                availability.owGlitches = 'possible';
            }
        }
        else if (this.canEnter('owGlitches', false, true)) {
            if (trackerData.items.moonpearl
                    && trackerData.items.firerod
                    && (trackerData.items.sword >= 1 || trackerData.dungeonchests[5] === 2)) {
                availability.owGlitches = 'glitchavailable';
            }
            else {
                availability.owGlitches = 'glitchpossible';
            }
        }
        else if (this.canEnter('owGlitches', true, false)) {
            availability.owGlitches = 'agahnim';
        }
        else if (this.canEnter('owGlitches', true, true)) {
            availability.owGlitches = 'glitchagahnim';
        }
        if (this.canEnter('majorGlitches', false, false)) {
            if (trackerData.items.moonpearl
                    && trackerData.items.firerod
                    && (trackerData.items.sword >= 1 || trackerData.dungeonchests[5] === 2)) {
                availability.majorGlitches = 'available';
            }
            else {
                availability.majorGlitches = 'possible';
            }
        }
        else if (this.canEnter('majorGlitches', false, true)) {
            if (trackerData.items.moonpearl
                    && trackerData.items.firerod
                    && (trackerData.items.sword >= 1 || trackerData.dungeonchests[5] === 2)) {
                availability.majorGlitches = 'glitchavailable';
            }
            else {
                availability.majorGlitches = 'glitchpossible';
            }
        }
        else if (this.canEnter('majorGlitches', true, false)) {
            availability.majorGlitches = 'agahnim';
        }
        else if (this.canEnter('majorGlitches', true, true)) {
            availability.majorGlitches = 'glitchagahnim';
        }
        if (trackerData.items.firerod && (trackerData.items.sword >= 1 || trackerData.dungeonchests[5] === 2)) {
            availability.inverted = 'available';
        }
        else {
            availability.inverted = 'possible';
        }

        return availability;
    }
};

dungeons[6] = {
    name: "Thieves' Town",
    label: "TT",
    x: "56.4%",
    y: "47.9%",
    image: "boss62.png",
    canEnter: function (logic, agahnimCheck, allowOutOfLogicGlitches) {
        if (logic === 'majorGlitches') {
            return glitchedLinkInDarkWorld() && canEnterNorthWestDarkWorld(logic, agahnimCheck, allowOutOfLogicGlitches);
        }
        else if (logic === 'owGlitches' || logic === 'glitchless') {
            return trackerData.items.moonpearl && canEnterNorthWestDarkWorld(logic, agahnimCheck, allowOutOfLogicGlitches);
        }
        else if (logic === 'inverted') {
            return true;
        }
    },
    canHurtBoss: function () {
        return trackerData.items.sword >= 1
                || trackerData.items.hammer
                || trackerData.items.somaria
                || trackerData.items.byrna;
    },
    isBeatable: function () {
        const availability = new Availability();
        if (this.canHurtBoss()) {
            if (this.canEnter('glitchless', false, false)) {
                availability.glitchless = 'available';
            }
            else if (this.canEnter('glitchless', true, false)) {
                availability.glitchless = 'agahnim';
            }
            else if (this.canEnter('glitchless', true, true)) {
                availability.glitchless = 'glitchagahnim';
            }
            if (this.canEnter('owGlitches', false, false)) {
                availability.owGlitches = 'available';
            }
            else if (this.canEnter('owGlitches', false, true)) {
                availability.owGlitches = 'glitchavailable';
            }
            else if (this.canEnter('owGlitches', true, false)) {
                availability.owGlitches = 'agahnim';
            }
            else if (this.canEnter('owGlitches', true, true)) {
                availability.owGlitches = 'glitchagahnim';
            }
            if (this.canEnter('majorGlitches', false, false)) {
                availability.majorGlitches = 'available';
            }
            else if (this.canEnter('majorGlitches', false, true)) {
                availability.majorGlitches = 'glitchavailable';
            }
            else if (this.canEnter('majorGlitches', true, false)) {
                availability.majorGlitches = 'agahnim';
            }
            else if (this.canEnter('majorGlitches', true, true)) {
                availability.majorGlitches = 'glitchagahnim';
            }
            availability.inverted = 'available';
        }
        return availability;
    },
    canGetChest: function () {
        const availability = new Availability();
        if (this.canEnter('glitchless', false, false)) {
            if (trackerData.items.hammer
                    || trackerData.dungeonchests[6] >= 3
                    || (this.canHurtBoss() && trackerData.dungeonchests[6] >= 2)) {
                availability.glitchless = 'available';
            }
            else {
                availability.glitchless = 'possible';
            }
        }
        else if (this.canEnter('glitchless', true, false)) {
            availability.glitchless = 'agahnim';
        }
        else if (this.canEnter('glitchless', true, true)) {
            availability.glitchless = 'glitchagahnim';
        }
        if (this.canEnter('owGlitches', false, false)) {
            if (trackerData.items.hammer
                    || trackerData.dungeonchests[6] >= 3
                    || (this.canHurtBoss() && trackerData.dungeonchests[6] >= 2)) {
                availability.owGlitches = 'available';
            }
            else {
                availability.owGlitches = 'possible';
            }
        }
        else if (this.canEnter('owGlitches', false, true)) {
            if (trackerData.items.hammer
                    || trackerData.dungeonchests[6] >= 3
                    || (this.canHurtBoss() && trackerData.dungeonchests[6] >= 2)) {
                availability.owGlitches = 'glitchavailable';
            }
            else {
                availability.owGlitches = 'glitchpossible';
            }
        }
        else if (this.canEnter('owGlitches', true, false)) {
            availability.owGlitches = 'agahnim';
        }
        else if (this.canEnter('owGlitches', true, true)) {
            availability.owGlitches = 'glitchagahnim';
        }
        if (this.canEnter('majorGlitches', false, false)) {
            if (trackerData.items.hammer
                    || trackerData.dungeonchests[6] >= 3
                    || (this.canHurtBoss() && trackerData.dungeonchests[6] >= 2)) {
                availability.majorGlitches = 'available';
            }
            else {
                availability.majorGlitches = 'possible';
            }
        }
        else if (this.canEnter('majorGlitches', false, true)) {
            if (trackerData.items.hammer
                    || trackerData.dungeonchests[6] >= 3
                    || (this.canHurtBoss() && trackerData.dungeonchests[6] >= 2)) {
                availability.majorGlitches = 'glitchavailable';
            }
            else {
                availability.majorGlitches = 'glitchpossible';
            }
        }
        else if (this.canEnter('majorGlitches', true, false)) {
            availability.majorGlitches = 'agahnim';
        }
        else if (this.canEnter('majorGlitches', true, true)) {
            availability.majorGlitches = 'glitchagahnim';
        }
        if (trackerData.items.hammer
                || trackerData.dungeonchests[6] >= 3
                || (this.canHurtBoss() && trackerData.dungeonchests[6] >= 2)) {
            availability.inverted = 'available';
        }
        else {
            availability.inverted = 'possible';
        }
        return availability;
    }
};

dungeons[7] = {
    name: "Ice Palace",
    label: "IP",
    x: "89.8%",
    y: "85.8%",
    image: "boss72.png",
    canEnter: function (logic, agahnimCheck, allowOutOfLogicGlitches) {
        if (logic === 'majorGlitches') {
            return canLiftDarkRocks()
                    || (trackerData.items.mirror && glitchedLinkInDarkWorld() && canEnterSouthDarkWorld(logic, agahnimCheck, allowOutOfLogicGlitches));
        }
        else if (logic === 'owGlitches') {
            return canLiftDarkRocks() && canMeltThings();
        }
        else if (logic === 'glitchless') {
            return canLiftDarkRocks()
                    && canMeltThings()
                    && (allowOutOfLogicGlitches || (trackerData.items.moonpearl && trackerData.items.flippers));
        }
        else if (logic === 'inverted') {
            return canMeltThings()
                    && (trackerData.items.flippers
                        || (allowOutOfLogicGlitches && canEnterNorthEastDarkWorld('inverted', agahnimCheck, allowOutOfLogicGlitches))
                        || (allowOutOfLogicGlitches && trackerData.items.boots)
                        || (allowOutOfLogicGlitches && canFly('inverted')));
        }
    },
    isBeatable: function () {
        const availability = new Availability();
        if (canMeltThings() && canLiftRocks()) {
            if (this.canEnter('glitchless', false, false) && trackerData.items.hammer) {
                if (trackerData.items.hookshot && trackerData.items.somaria) {
                    availability.glitchless = 'available';
                }
                else {
                    availability.glitchless = 'possible';
                }
            }
            else if (this.canEnter('glitchless', false, true) && trackerData.items.hammer) {
                availability.glitchless = 'glitchavailable';
            }
            if (this.canEnter('owGlitches', false, false) && trackerData.items.hammer) {
                if (trackerData.items.hookshot && trackerData.items.somaria) {
                    availability.owGlitches = 'available';
                }
                else {
                    availability.owGlitches = 'possible';
                }
            }
            else if (this.canEnter('owGlitches', false, true)) {
                availability.owGlitches = 'glitchavailable';
            }
            if (this.canEnter('majorGlitches', false, false) && trackerData.items.hammer) {
                if (trackerData.items.hookshot && trackerData.items.somaria) {
                    availability.majorGlitches = 'available';
                }
                else {
                    availability.majorGlitches = 'possible';
                }
            }
            else if (this.canEnter('majorGlitches', false, true)) {
                availability.majorGlitches = 'glitchavailable';
            }
            else if (this.canEnter('majorGlitches', true, false) && trackerData.items.hammer) {
                availability.majorGlitches = 'agahnim';
            }
            else if (this.canEnter('majorGlitches', true, true)) {
                availability.majorGlitches = 'glitchagahnim';
            }
            if (this.canEnter('inverted', false, false) && trackerData.items.hammer) {
                if (trackerData.items.hookshot && trackerData.items.somaria) {
                    availability.inverted = 'available';
                }
                else {
                    availability.inverted = 'possible';
                }
            }
            else if (this.canEnter('inverted', false, true) && trackerData.items.hammer) {
                availability.inverted = 'glitchavailable';
            }
        }

        return availability;
    },
    canGetChest: function () {
        const availability = new Availability();
        if (this.canEnter('glitchless', false, false)) {
            if (trackerData.items.hammer && canLiftRocks()) {
                if (trackerData.items.hookshot) {
                    availability.glitchless = 'available';
                }
                else if (trackerData.items.byrna || trackerData.items.cape) {
                    if (trackerData.dungeonchests[7] >= 2) {
                        availability.glitchless = 'available';
                    }
                    else {
                        availability.glitchless = 'possible';
                    }
                }
                else {
                    availability.glitchless = 'possible';
                }
            }
            else {
                availability.glitchless = 'possible';
            }
        }
        else if (this.canEnter('glitchless', false, true)) {
            if (trackerData.items.hammer && canLiftRocks()) {
                if (trackerData.items.hookshot) {
                    availability.glitchless = 'glitchavailable';
                }
                else {
                    if (trackerData.dungeonchests[7] >= 2) {
                        availability.glitchless = 'glitchavailable';
                    }
                    else {
                        availability.glitchless = 'glitchpossible';
                    }
                }
            }
            else {
                availability.glitchless = 'glitchpossible';
            }
        }
        if (this.canEnter('owGlitches', false, false)) {
            if (trackerData.items.hammer && canLiftRocks()) {
                if (trackerData.items.hookshot) {
                    availability.owGlitches = 'available';
                }
                else if (trackerData.items.byrna || trackerData.items.cape) {
                    if (trackerData.dungeonchests[7] >= 2) {
                        availability.owGlitches = 'available';
                    }
                    else {
                        availability.owGlitches = 'possible';
                    }
                }
                else {
                    availability.owGlitches = 'possible';
                }
            }
            else {
                availability.owGlitches = 'possible';
            }
        }
        else if (this.canEnter('owGlitches', false, true)) {
            if (trackerData.items.hammer && canLiftRocks()) {
                if (trackerData.items.hookshot) {
                    availability.owGlitches = 'glitchavailable';
                }
                else {
                    if (trackerData.dungeonchests[7] >= 2) {
                        availability.owGlitches = 'glitchavailable';
                    }
                    else {
                        availability.owGlitches = 'glitchpossible';
                    }
                }
            }
            else {
                availability.owGlitches = 'glitchpossible';
            }
        }
        if (this.canEnter('majorGlitches', false, false)) {
            if (trackerData.items.hammer && canLiftRocks()) {
                if (trackerData.items.hookshot) {
                    availability.majorGlitches = 'available';
                }
                else if (trackerData.items.byrna || trackerData.items.cape) {
                    if (trackerData.dungeonchests[7] >= 2) {
                        availability.majorGlitches = 'available';
                    }
                    else {
                        availability.majorGlitches = 'possible';
                    }
                }
                else {
                    availability.majorGlitches = 'possible';
                }
            }
            else {
                availability.majorGlitches = 'possible';
            }
        }
        else if (this.canEnter('majorGlitches', false, true)) {
            if (trackerData.items.hammer && canLiftRocks()) {
                if (trackerData.items.hookshot) {
                    availability.majorGlitches = 'glitchavailable';
                }
                else {
                    if (trackerData.dungeonchests[7] >= 2) {
                        availability.majorGlitches = 'glitchavailable';
                    }
                    else {
                        availability.majorGlitches = 'glitchpossible';
                    }
                }
            }
            else {
                availability.majorGlitches = 'glitchpossible';
            }
        }
        if (this.canEnter('inverted', false, false)) {
            if (trackerData.items.hammer && canLiftRocks()
                    && (trackerData.items.hookshot
                        || ((trackerData.items.byrna || trackerData.items.cape) && trackerData.dungeonchests[7] >= 2))) {
                availability.inverted = 'available';
            }
            else {
                availability.inverted = 'possible';
            }
        }
        else if (this.canEnter('inverted', false, true)) {
            if (trackerData.items.hammer && canLiftRocks() && trackerData.items.hookshot) {
                availability.inverted = 'glitchavailable';
            }
            else {
                availability.inverted = 'glitchpossible';
            }
        }
        return availability;
    }
};

dungeons[8] = {
    name: "Misery Mire <img src='/images/medallion0.png' class='mini'><img src='/images/lantern.png' class='mini'>",
    label: "MM",
    x: "55.8%",
    y: "82.9%",
    image: "boss82.png",
    hasMedallion: function () {
        return (trackerData.medallions[8] === 1 && trackerData.items.bombos)
                || (trackerData.medallions[8] === 2 && trackerData.items.ether)
                || (trackerData.medallions[8] === 3 && trackerData.items.quake)
                || (trackerData.items.bombos && trackerData.items.ether && trackerData.items.quake);
    },
    mayHaveMedallion: function () {
        return !((trackerData.medallions[8] === 1 && !trackerData.items.bombos)
                || (trackerData.medallions[8] === 2 && !trackerData.items.ether)
                || (trackerData.medallions[8] === 3 && !trackerData.items.quake)
                || (!trackerData.items.bombos && !trackerData.items.ether && !trackerData.items.quake));
    },
    canEnter: function (logic, agahnimCheck, allowOutOfLogicGlitches) {
        if (logic === 'glitchless') {
            return this.hasMedallion()
                    && trackerData.items.sword >= 1
                    && trackerData.items.moonpearl
                    && (trackerData.items.boots || trackerData.items.hookshot)
                    && canEnterMireArea('glitchless', agahnimCheck, allowOutOfLogicGlitches);
        }
        else if (logic === 'owGlitches') {
            return this.hasMedallion()
                    && trackerData.items.sword >= 1
                    && trackerData.items.moonpearl
                    && (trackerData.items.boots || trackerData.items.hookshot)
                    && canEnterMireArea('owGlitches', agahnimCheck, allowOutOfLogicGlitches);
        }
        else if (logic === 'majorGlitches') {
            return this.hasMedallion()
                    && trackerData.items.sword >= 1
                    && (trackerData.items.moonpearl || (trackerData.items.bottle >= 1 && trackerData.items.boots))
                    && (trackerData.items.boots || trackerData.items.hookshot)
                    && canEnterMireArea('majorGlitches', agahnimCheck, allowOutOfLogicGlitches);
        }
        else if (logic === 'inverted') {
            return this.hasMedallion()
                    && trackerData.items.sword >= 1
                    && (trackerData.items.boots || trackerData.items.hookshot)
                    && canEnterMireArea('inverted', agahnimCheck, allowOutOfLogicGlitches);
        }
    },
    mayEnter: function (logic, agahnimCheck, allowOutOfLogicGlitches) {
        if (logic === 'glitchless') {
            return this.mayHaveMedallion()
                    && trackerData.items.sword >= 1
                    && trackerData.items.moonpearl
                    && (trackerData.items.boots || trackerData.items.hookshot)
                    && canEnterMireArea('glitchless', agahnimCheck, allowOutOfLogicGlitches);
        }
        else if (logic === 'owGlitches') {
            return this.mayHaveMedallion()
                    && trackerData.items.sword >= 1
                    && trackerData.items.moonpearl
                    && (trackerData.items.boots || trackerData.items.hookshot)
                    && canEnterMireArea('owGlitches', agahnimCheck, allowOutOfLogicGlitches);
        }
        else if (logic === 'majorGlitches') {
            return this.mayHaveMedallion()
                    && trackerData.items.sword >= 1
                    && (trackerData.items.moonpearl || (trackerData.items.bottle >= 1 && trackerData.items.boots))
                    && (trackerData.items.boots || trackerData.items.hookshot)
                    && canEnterMireArea('majorGlitches', agahnimCheck, allowOutOfLogicGlitches);
        }
        else if (logic === 'inverted') {
            return this.mayHaveMedallion()
                    && trackerData.items.sword >= 1
                    && (trackerData.items.boots || trackerData.items.hookshot)
                    && canEnterMireArea('inverted', agahnimCheck, allowOutOfLogicGlitches);
        }
    },
    canHurtBoss: function () {
        return trackerData.items.sword >= 1 || trackerData.items.hammer || trackerData.items.bow;
    },
    isBeatable: function () {
        const availability = new Availability();
        if (trackerData.items.somaria && this.canHurtBoss()) {
            if (this.canEnter('glitchless', false, false) && trackerData.items.lantern) {
                if (trackerData.items.byrna || trackerData.items.cape) {
                    availability.glitchless = 'available';
                }
                else {
                    availability.glitchless = 'possible';
                }
            }
            else if (this.mayEnter('glitchless', false, false) && trackerData.items.lantern) {
                availability.glitchless = 'possible';
            }
            else if (this.canEnter('glitchless', false, true)) {
                if (canLightTorches() && (trackerData.items.byrna || trackerData.items.cape)) {
                    availability.glitchless = 'glitchavailable';
                }
                else {
                    availability.glitchless = 'glitchpossible';
                }
            }
            else if (this.mayEnter('glitchless', false, true)) {
                availability.glitchless = 'glitchpossible';
            }
            if (this.canEnter('owGlitches', false, false) && trackerData.items.lantern) {
                if (trackerData.items.byrna || trackerData.items.cape) {
                    availability.owGlitches = 'available';
                }
                else {
                    availability.owGlitches = 'possible';
                }
            }
            else if (this.mayEnter('owGlitches', false, false) && trackerData.items.lantern) {
                availability.owGlitches = 'possible';
            }
            else if (this.canEnter('owGlitches', false, true)) {
                if (canLightTorches() && (trackerData.items.byrna || trackerData.items.cape)) {
                    availability.owGlitches = 'glitchavailable';
                }
                else {
                    availability.owGlitches = 'glitchpossible';
                }
            }
            else if (this.mayEnter('owGlitches', false, true)) {
                availability.owGlitches = 'glitchpossible';
            }
            else if (this.mayEnter('owGlitches', true, false)) {
                availability.owGlitches = 'agahnim';
            }
            else if (this.mayEnter('owGlitches', true, true)) {
                availability.owGlitches = 'glitchagahnim';
            }
            if (this.canEnter('majorGlitches', false, false) && trackerData.items.lantern) {
                if (trackerData.items.byrna || trackerData.items.cape) {
                    availability.majorGlitches = 'available';
                }
                else {
                    availability.majorGlitches = 'possible';
                }
            }
            else if (this.mayEnter('majorGlitches', false, false) && trackerData.items.lantern) {
                availability.majorGlitches = 'possible';
            }
            else if (this.canEnter('majorGlitches', false, true)) {
                if (canLightTorches() && (trackerData.items.byrna || trackerData.items.cape)) {
                    availability.majorGlitches = 'glitchavailable';
                }
                else {
                    availability.majorGlitches = 'glitchpossible';
                }
            }
            else if (this.mayEnter('majorGlitches', false, true)) {
                availability.majorGlitches = 'glitchpossible';
            }
            else if (this.mayEnter('majorGlitches', true, false)) {
                availability.majorGlitches = 'agahnim';
            }
            else if (this.mayEnter('majorGlitches', true, true)) {
                availability.majorGlitches = 'glitchagahnim';
            }
            if (this.canEnter('inverted', false, true)) {
                if (this.canEnter('inverted', false, false) && trackerData.items.lantern) {
                    if (trackerData.items.byrna || trackerData.items.cape) {
                        availability.inverted = 'available';
                    }
                    else {
                        availability.inverted = 'possible';
                    }
                }
                else {
                    if (canLightTorches() && (trackerData.items.byrna || trackerData.items.cape)) {
                        availability.inverted = 'glitchavailable';
                    }
                    else {
                        availability.inverted = 'glitchpossible';
                    }
                }
            }
            else if (this.mayEnter('inverted', false, true)) {
                if (this.mayEnter('inverted', false, false) && trackerData.items.lantern) {
                    availability.inverted = 'possible';
                }
                else {
                    availability.inverted = 'glitchpossible';
                }
            }
            else if (this.mayEnter('inverted', true, false)) {
                availability.inverted = 'agahnim';
            }
            else if (this.mayEnter('inverted', true, true)) {
                availability.inverted = 'glitchagahnim';
            }
        }
        return availability;
    },
    canGetChest: function () {
        const availability = new Availability();
        if (this.canEnter('glitchless', false, false)) {
            if (canLightTorches()) {
                if (trackerData.dungeonchests[8] === 2
                        && (trackerData.items.cape
                                || trackerData.items.byrna
                                || (trackerData.items.somaria && this.canHurtBoss()))) {
                    availability.glitchless = 'available';
                }
                else if (trackerData.dungeonchests[8] === 1
                        && (trackerData.items.cape || trackerData.items.byrna)
                        && trackerData.items.somaria
                        && this.canHurtBoss()) {
                    availability.glitchless = 'available';
                }
                else {
                    availability.glitchless = 'possible';
                }
            }
            else {
                availability.glitchless = 'possible';
            }
        }
        else if (this.mayEnter('glitchless', false, false)) {
            availability.glitchless = 'possible';
        }
        if (this.canEnter('owGlitches', false, false)) {
            if (canLightTorches()) {
                if (trackerData.dungeonchests[8] === 2
                        && (trackerData.items.cape
                                || trackerData.items.byrna
                                || (trackerData.items.somaria && this.canHurtBoss() && trackerData.items.lantern))) {
                    availability.owGlitches = 'available';
                }
                else if (trackerData.dungeonchests[8] === 1
                        && (trackerData.items.cape || trackerData.items.byrna)
                        && trackerData.items.somaria
                        && this.canHurtBoss()
                        && trackerData.items.lantern) {
                    availability.owGlitches = 'available';
                }
                else {
                    availability.owGlitches = 'possible';
                }
            }
            else {
                availability.owGlitches = 'possible';
            }
        }
        else if (this.mayEnter('owGlitches', false, false)) {
            availability.owGlitches = 'possible';
        }
        else if (this.canEnter('owGlitches', false, true)) {
            if (canLightTorches()) {
                if (trackerData.dungeonchests[8] === 2
                        && (trackerData.items.cape
                                || trackerData.items.byrna
                                || (trackerData.items.somaria && this.canHurtBoss() && trackerData.items.lantern))) {
                    availability.owGlitches = 'glitchavailable';
                }
                else if (trackerData.dungeonchests[8] === 1
                        && (trackerData.items.cape || trackerData.items.byrna)
                        && trackerData.items.somaria
                        && this.canHurtBoss()
                        && trackerData.items.lantern) {
                    availability.owGlitches = 'glitchavailable';
                }
                else {
                    availability.owGlitches = 'glitchpossible';
                }
            }
            else {
                availability.owGlitches = 'glitchpossible';
            }
        }
        else if (this.mayEnter('owGlitches', false, true)) {
            availability.owGlitches = 'glitchpossible';
        }
        else if (this.mayEnter('owGlitches', true, false)) {
            availability.owGlitches = 'agahnim';
        }
        else if (this.mayEnter('owGlitches', true, true)) {
            availability.owGlitches = 'glitchagahnim';
        }
        if (this.canEnter('majorGlitches', false, false)) {
            if (canLightTorches()) {
                if (trackerData.dungeonchests[8] === 2
                        && (trackerData.items.cape
                                || trackerData.items.byrna
                                || (trackerData.items.somaria && this.canHurtBoss() && trackerData.items.lantern))) {
                    availability.majorGlitches = 'available';
                }
                else if (trackerData.dungeonchests[8] === 1
                        && (trackerData.items.cape || trackerData.items.byrna)
                        && trackerData.items.somaria
                        && this.canHurtBoss()
                        && trackerData.items.lantern) {
                    availability.majorGlitches = 'available';
                }
                else {
                    availability.majorGlitches = 'possible';
                }
            }
            else {
                availability.majorGlitches = 'possible';
            }
        }
        else if (this.mayEnter('majorGlitches', false, false)) {
            availability.majorGlitches = 'possible';
        }
        else if (this.canEnter('majorGlitches', false, true)) {
            if (canLightTorches()) {
                if (trackerData.dungeonchests[8] === 2
                        && (trackerData.items.cape
                                || trackerData.items.byrna
                                || (trackerData.items.somaria && this.canHurtBoss() && trackerData.items.lantern))) {
                    availability.majorGlitches = 'glitchavailable';
                }
                else if (trackerData.dungeonchests[8] === 1
                        && (trackerData.items.cape || trackerData.items.byrna)
                        && trackerData.items.somaria
                        && this.canHurtBoss()
                        && trackerData.items.lantern) {
                    availability.majorGlitches = 'glitchavailable';
                }
                else {
                    availability.majorGlitches = 'glitchpossible';
                }
            }
            else {
                availability.majorGlitches = 'glitchpossible';
            }
        }
        else if (this.mayEnter('majorGlitches', false, true)) {
            availability.majorGlitches = 'glitchpossible';
        }
        else if (this.mayEnter('majorGlitches', true, false)) {
            availability.majorGlitches = 'agahnim';
        }
        else if (this.mayEnter('majorGlitches', true, true)) {
            availability.majorGlitches = 'glitchagahnim';
        }
        if (this.canEnter('inverted', false, false)) {
            if (canLightTorches()) {
                if (trackerData.dungeonchests[8] === 2
                        && (trackerData.items.cape
                                || trackerData.items.byrna
                                || (trackerData.items.somaria && this.canHurtBoss()))) {
                    availability.inverted = 'available';
                }
                else if (trackerData.dungeonchests[8] === 1
                        && (trackerData.items.cape || trackerData.items.byrna)
                        && trackerData.items.somaria
                        && this.canHurtBoss()) {
                    availability.inverted = 'available';
                }
                else {
                    availability.inverted = 'possible';
                }
            }
            else {
                availability.inverted = 'possible';
            }
        }
        else if (this.mayEnter('inverted', false, false)) {
            availability.inverted = 'possible';
        }
        else if (this.mayEnter('inverted', true, false)) {
            availability.inverted = 'agahnim';
        }
        else if (this.mayEnter('inverted', true, true)) {
            availability.inverted = 'glitchagahnim';
        }
        return availability;
    }
};

dungeons[9] = {
    name: "Turtle Rock <img src='/images/medallion0.png' class='mini'><img src='/images/lantern.png' class='mini'>",
    label: "TR",
    x: "96.9%",
    y: "7.0%",
    image: "boss92.png",
    hasMedallion: function () {
        return (trackerData.medallions[9] === 1 && trackerData.items.bombos)
                || (trackerData.medallions[9] === 2 && trackerData.items.ether)
                || (trackerData.medallions[9] === 3 && trackerData.items.quake)
                || (trackerData.items.bombos && trackerData.items.ether && trackerData.items.quake);
    },
    mayHaveMedallion: function () {
        return !((trackerData.medallions[9] === 1 && !trackerData.items.bombos)
                || (trackerData.medallions[9] === 2 && !trackerData.items.ether)
                || (trackerData.medallions[9] === 3 && !trackerData.items.quake)
                || (!trackerData.items.bombos && !trackerData.items.ether && !trackerData.items.quake));
    },
    lower: function (logic, allowOutOfLogicGlitches) {
        if (logic === 'majorGlitches') {
            return logic === canEnterWestDeathMountain('majorGlitches', allowOutOfLogicGlitches)
                    && (trackerData.items.moonpearl || (trackerData.items.bottle >= 1 && trackerData.items.boots))
                    && trackerData.items.mirror;
        }
        else if (logic === 'inverted') {
            return canEnterEastDeathMountain(logic, allowOutOfLogicGlitches) && trackerData.items.mirror;
        }
    },
    middle: function (logic, allowOutOfLogicGlitches) {
        if (logic === 'majorGlitches') {
            return (trackerData.items.mirror || (glitchedLinkInDarkWorld() && canSpinSpeed()))
                    && (trackerData.items.boots || trackerData.items.somaria || trackerData.items.hookshot)
                    && canEnterDarkWorldDeathMountain('majorGlitches', allowOutOfLogicGlitches);
        }
        else if (logic === 'owGlitches') {
            return (trackerData.items.mirror || (trackerData.items.moonpearl && canSpinSpeed()))
                    && (trackerData.items.boots || trackerData.items.somaria || trackerData.items.hookshot)
                    && canEnterDarkWorldDeathMountain('owGlitches', allowOutOfLogicGlitches);
        }
        else if (logic === 'glitchless') {
            return false;
        }
        else if (logic === 'inverted') {
            return canEnterEastDeathMountain(logic, allowOutOfLogicGlitches) && trackerData.items.mirror;
        }
    },
    upperCan: function (logic, allowOutOfLogicGlitches) {
        if (logic === 'majorGlitches') {
            return this.hasMedallion()
                    && trackerData.items.sword >= 1
                    && (trackerData.items.moonpearl || (trackerData.items.bottle >= 1 && trackerData.items.boots))
                    && trackerData.items.somaria
                    && trackerData.items.hammer
                    && (canLiftDarkRocks() || trackerData.items.boots)
                    && canEnterEastDeathMountain(logic, allowOutOfLogicGlitches);
        }
        else if (logic === 'owGlitches') {
            return this.hasMedallion()
                    && trackerData.items.sword >= 1
                    && trackerData.items.moonpearl
                    && trackerData.items.somaria
                    && trackerData.items.hammer
                    && (canLiftDarkRocks() || trackerData.items.boots)
                    && canEnterEastDeathMountain(logic, allowOutOfLogicGlitches);
        }
        else if (logic === 'glitchless') {
            return this.hasMedallion()
                    && trackerData.items.sword >= 1
                    && trackerData.items.moonpearl
                    && trackerData.items.somaria
                    && trackerData.items.hammer
                    && canLiftDarkRocks()
                    && canEnterEastDeathMountain(logic, allowOutOfLogicGlitches);
        }
        else if (logic === 'inverted') {
            return this.hasMedallion()
                    && trackerData.items.sword >= 1
                    && trackerData.items.somaria
                    && canEnterDarkWorldDeathMountain(logic, allowOutOfLogicGlitches);
        }
    },
    upperMay: function (logic, allowOutOfLogicGlitches) {
        if (logic === 'majorGlitches') {
            return this.mayHaveMedallion()
                    && trackerData.items.sword >= 1
                    && (trackerData.items.moonpearl || (trackerData.items.bottle >= 1 && trackerData.items.boots))
                    && trackerData.items.somaria
                    && trackerData.items.hammer
                    && (canLiftDarkRocks() || trackerData.items.boots)
                    && canEnterEastDeathMountain(logic, allowOutOfLogicGlitches);
        }
        else if (logic === 'owGlitches') {
            return this.mayHaveMedallion()
                    && trackerData.items.sword >= 1
                    && trackerData.items.moonpearl
                    && trackerData.items.somaria
                    && trackerData.items.hammer
                    && (canLiftDarkRocks() || trackerData.items.boots)
                    && canEnterEastDeathMountain(logic, allowOutOfLogicGlitches);
        }
        else if (logic === 'glitchless') {
            return this.mayHaveMedallion()
                    && trackerData.items.sword >= 1
                    && trackerData.items.moonpearl
                    && trackerData.items.somaria
                    && trackerData.items.hammer
                    && canLiftDarkRocks()
                    && canEnterEastDeathMountain(logic, allowOutOfLogicGlitches);
        }
        else if (logic === 'inverted') {
            return this.mayHaveMedallion()
                    && trackerData.items.sword >= 1
                    && trackerData.items.somaria
                    && canEnterDarkWorldDeathMountain(logic, allowOutOfLogicGlitches);
        }
    },
    canEnter: function (logic, allowOutOfLogicGlitches) {
        if (logic === 'majorGlitches') {
            return this.lower('majorGlitches', allowOutOfLogicGlitches) || this.middle('majorGlitches', allowOutOfLogicGlitches) || this.upperCan('majorGlitches', allowOutOfLogicGlitches);
        }
        else if (logic === 'owGlitches') {
            return this.middle('owGlitches', allowOutOfLogicGlitches) || this.upperCan('owGlitches', allowOutOfLogicGlitches);
        }
        else if (logic === 'glitchless') {
            return this.upperCan('glitchless', allowOutOfLogicGlitches);
        }
        else if (logic === 'inverted') {
            return this.lower('inverted', allowOutOfLogicGlitches) || this.middle('inverted', allowOutOfLogicGlitches) || this.upperCan('inverted', allowOutOfLogicGlitches);
        }
    },
    mayEnter: function (logic, allowOutOfLogicGlitches) {
        if (logic === 'majorGlitches') {
            return this.lower('majorGlitches', allowOutOfLogicGlitches) || this.middle('majorGlitches', allowOutOfLogicGlitches) || this.upperMay('majorGlitches', allowOutOfLogicGlitches);
        }
        else if (logic === 'owGlitches') {
            return this.middle('owGlitches', allowOutOfLogicGlitches) || this.upperMay('owGlitches', allowOutOfLogicGlitches);
        }
        else if (logic === 'glitchless') {
            return this.upperMay('glitchless', allowOutOfLogicGlitches);
        }
        else if (logic === 'inverted') {
            return this.lower('inverted', allowOutOfLogicGlitches) || this.middle('inverted', allowOutOfLogicGlitches) || this.upperMay('inverted', allowOutOfLogicGlitches);
        }
    },
    isBeatable: function () {
        const availability = new Availability();
        if (trackerData.items.firerod && trackerData.items.icerod && trackerData.items.somaria) {
            if (this.canEnter('glitchless', false)
                    && trackerData.items.lantern
                    && (trackerData.items.hammer || trackerData.items.sword >= 2)) {
                if (trackerData.items.cape || trackerData.items.byrna || canBlockLasers()) {
                    availability.glitchless = 'available';
                }
                else {
                    availability.glitchless = 'possible';
                }
            }
            else if (this.mayEnter('glitchless', false)) {
                availability.glitchless = 'possible';
            }
            else if (this.canEnter('glitchless', true)) {
                if (trackerData.items.cape || trackerData.items.byrna || canBlockLasers()) {
                    availability.glitchless = 'glitchavailable';
                }
                else {
                    availability.glitchless = 'glitchpossible';
                }
            }
            else if (this.mayEnter('glitchless', true)) {
                availability.glitchless = 'glitchpossible';
            }
            if (this.canEnter('owGlitches', false)
                    && trackerData.items.lantern
                    && (trackerData.items.hammer || trackerData.items.sword >= 2)) {
                if (trackerData.items.cape || trackerData.items.byrna || canBlockLasers()) {
                    availability.owGlitches = 'available';
                }
                else {
                    availability.owGlitches = 'possible';
                }
            }
            else if (this.mayEnter('owGlitches', false)) {
                availability.owGlitches = 'possible';
            }
            else if (this.canEnter('owGlitches', true)) {
                if (trackerData.items.cape || trackerData.items.byrna || canBlockLasers()) {
                    availability.owGlitches = 'glitchavailable';
                }
                else {
                    availability.owGlitches = 'glitchpossible';
                }
            }
            else if (this.mayEnter('owGlitches', true)) {
                availability.owGlitches = 'glitchpossible';
            }
            if (this.canEnter('majorGlitches', false)
                    && trackerData.items.lantern
                    && (trackerData.items.hammer || trackerData.items.sword >= 2)) {
                if (trackerData.items.cape || trackerData.items.byrna || canBlockLasers()) {
                    availability.majorGlitches = 'available';
                }
                else {
                    availability.majorGlitches = 'possible';
                }
            }
            else if (this.mayEnter('majorGlitches', false)) {
                availability.majorGlitches = 'possible';
            }
            else if (this.canEnter('majorGlitches', true)) {
                if (trackerData.items.cape || trackerData.items.byrna || canBlockLasers()) {
                    availability.majorGlitches = 'glitchavailable';
                }
                else {
                    availability.majorGlitches = 'glitchpossible';
                }
            }
            else if (this.mayEnter('majorGlitches', true)) {
                availability.majorGlitches = 'glitchpossible';
            }
            if (this.canEnter('inverted', false)
                    && trackerData.items.lantern
                    && (trackerData.items.hammer || trackerData.items.sword >= 2)) {
                if (trackerData.items.cape || trackerData.items.byrna || canBlockLasers()) {
                    availability.inverted = 'available';
                }
                else {
                    availability.inverted = 'possible';
                }
            }
            else if (this.mayEnter('inverted', false)) {
                availability.inverted = 'possible';
            }
            else if (this.canEnter('inverted', true)) {
                if (trackerData.items.cape || trackerData.items.byrna || canBlockLasers()) {
                    availability.inverted = 'glitchavailable';
                }
                else {
                    availability.inverted = 'glitchpossible';
                }
            }
            else if (this.mayEnter('inverted', true)) {
                availability.inverted = 'glitchpossible';
            }
        }
        return availability;
    },
    canGetChest: function () {
        const availability = new Availability();
        if (this.canEnter('glitchless', false)) {
            if (trackerData.items.firerod) {
                if (trackerData.items.lantern && (trackerData.items.cape || trackerData.items.byrna || canBlockLasers())) {
                    if (trackerData.dungeonchests[9] >= 2 || this.isBeatable().glitchless === 'available') {
                        availability.glitchless = 'available';
                    }
                    else {
                        availability.glitchless = 'possible';
                    }
                }
                else if (trackerData.dungeonchests[9] >= 2) {
                    availability.glitchless = 'possible';
                }
                else {
                    availability.glitchless = 'glitchpossible'
                }
            }
            else {
                if (trackerData.items.lantern && (trackerData.items.cape || trackerData.items.byrna || canBlockLasers())) {
                    availability.glitchless = 'possible';
                }
                else if (trackerData.dungeonchests[9] >= 4) {
                    availability.glitchless = 'possible';
                }
                else {
                    availability.glitchless = 'glitchpossible';
                }
            }
        }
        else if (this.mayEnter('glitchless', false)) {
            if (trackerData.items.firerod) {
                if (trackerData.items.lantern && (trackerData.items.cape || trackerData.items.byrna || canBlockLasers())) {
                    availability.glitchless = 'possible';
                }
                else if (trackerData.dungeonchests[9] >= 2) {
                    availability.glitchless = 'possible';
                }
                else {
                    availability.glitchless = 'glitchpossible'
                }
            }
            else {
                if (trackerData.items.lantern && (trackerData.items.cape || trackerData.items.byrna || canBlockLasers())) {
                    availability.glitchless = 'possible';
                }
                else if (trackerData.dungeonchests[9] >= 4) {
                    availability.glitchless = 'possible';
                }
                else {
                    availability.glitchless = 'glitchpossible';
                }
            }
        }
        else if (this.canEnter('glitchless', true)) {
            if (trackerData.items.firerod) {
                if (trackerData.dungeonchests[9] >= 2
                        || this.isBeatable().glitchless === 'available'
                        || this.isBeatable().glitchless === 'glitchavailable') {
                    availability.glitchless = 'glitchavailable';
                }
                else {
                    availability.glitchless = 'glitchpossible';
                }
            }
            else {
                availability.glitchless = 'glitchpossible';
            }
        }
        else if (this.mayEnter('glitchless', true)) {
            availability.glitchless = 'glitchpossible';
        }
        // TODO: Account for lower/middle entrances for owGlitches and majorGlitches chest counts.
        if (this.canEnter('owGlitches', false)) {
            if (trackerData.items.firerod) {
                if (trackerData.items.lantern && (trackerData.items.cape || trackerData.items.byrna || canBlockLasers())) {
                    if (trackerData.dungeonchests[9] >= 2 || this.isBeatable().glitchless === 'available') {
                        availability.owGlitches = 'available';
                    }
                    else {
                        availability.owGlitches = 'possible';
                    }
                }
                else if (trackerData.dungeonchests[9] >= 2) {
                    availability.owGlitches = 'possible';
                }
                else {
                    availability.owGlitches = 'glitchpossible'
                }
            }
            else {
                if (trackerData.items.lantern && (trackerData.items.cape || trackerData.items.byrna || canBlockLasers())) {
                    availability.owGlitches = 'possible';
                }
                else if (trackerData.dungeonchests[9] >= 4) {
                    availability.owGlitches = 'possible';
                }
                else {
                    availability.owGlitches = 'glitchpossible';
                }
            }
        }
        else if (this.mayEnter('owGlitches', false)) {
            if (trackerData.items.firerod) {
                if (trackerData.items.lantern && (trackerData.items.cape || trackerData.items.byrna || canBlockLasers())) {
                    availability.owGlitches = 'possible';
                }
                else if (trackerData.dungeonchests[9] >= 2) {
                    availability.owGlitches = 'possible';
                }
                else {
                    availability.owGlitches = 'glitchpossible'
                }
            }
            else {
                if (trackerData.items.lantern && (trackerData.items.cape || trackerData.items.byrna || canBlockLasers())) {
                    availability.owGlitches = 'possible';
                }
                else if (trackerData.dungeonchests[9] >= 4) {
                    availability.owGlitches = 'possible';
                }
                else {
                    availability.owGlitches = 'glitchpossible';
                }
            }
        }
        else if (this.canEnter('owGlitches', true)) {
            if (trackerData.items.firerod) {
                if (trackerData.dungeonchests[9] >= 2
                        || this.isBeatable().glitchless === 'available'
                        || this.isBeatable().glitchless === 'glitchavailable') {
                    availability.owGlitches = 'glitchavailable';
                }
                else {
                    availability.owGlitches = 'glitchpossible';
                }
            }
            else {
                availability.owGlitches = 'glitchpossible';
            }
        }
        else if (this.mayEnter('owGlitches', true)) {
            availability.owGlitches = 'glitchpossible';
        }
        if (this.canEnter('majorGlitches', false)) {
            if (trackerData.items.firerod) {
                if (trackerData.items.lantern && (trackerData.items.cape || trackerData.items.byrna || canBlockLasers())) {
                    if (trackerData.dungeonchests[9] >= 2 || this.isBeatable().glitchless === 'available') {
                        availability.majorGlitches = 'available';
                    }
                    else {
                        availability.majorGlitches = 'possible';
                    }
                }
                else if (trackerData.dungeonchests[9] >= 2) {
                    availability.majorGlitches = 'possible';
                }
                else {
                    availability.majorGlitches = 'glitchpossible'
                }
            }
            else {
                if (trackerData.items.lantern && (trackerData.items.cape || trackerData.items.byrna || canBlockLasers())) {
                    availability.majorGlitches = 'possible';
                }
                else if (trackerData.dungeonchests[9] >= 4) {
                    availability.majorGlitches = 'possible';
                }
                else {
                    availability.majorGlitches = 'glitchpossible';
                }
            }
        }
        else if (this.mayEnter('majorGlitches', false)) {
            if (trackerData.items.firerod) {
                if (trackerData.items.lantern && (trackerData.items.cape || trackerData.items.byrna || canBlockLasers())) {
                    availability.majorGlitches = 'possible';
                }
                else if (trackerData.dungeonchests[9] >= 2) {
                    availability.majorGlitches = 'possible';
                }
                else {
                    availability.majorGlitches = 'glitchpossible'
                }
            }
            else {
                if (trackerData.items.lantern && (trackerData.items.cape || trackerData.items.byrna || canBlockLasers())) {
                    availability.majorGlitches = 'possible';
                }
                else if (trackerData.dungeonchests[9] >= 4) {
                    availability.majorGlitches = 'possible';
                }
                else {
                    availability.majorGlitches = 'glitchpossible';
                }
            }
        }
        else if (this.canEnter('majorGlitches', true)) {
            if (trackerData.items.firerod) {
                if (trackerData.dungeonchests[9] >= 2
                        || this.isBeatable().glitchless === 'available'
                        || this.isBeatable().glitchless === 'glitchavailable') {
                    availability.majorGlitches = 'glitchavailable';
                }
                else {
                    availability.majorGlitches = 'glitchpossible';
                }
            }
            else {
                availability.majorGlitches = 'glitchpossible';
            }
        }
        else if (this.mayEnter('majorGlitches', true)) {
            availability.majorGlitches = 'glitchpossible';
        }
        if (this.canEnter('inverted', false)) {
            if (trackerData.items.firerod) {
                if (trackerData.items.lantern && (trackerData.items.cape || trackerData.items.byrna || canBlockLasers())) {
                    if (trackerData.dungeonchests[9] >= 2 || this.isBeatable().glitchless === 'available') {
                        availability.inverted = 'available';
                    }
                    else {
                        availability.inverted = 'possible';
                    }
                }
                else if (trackerData.dungeonchests[9] >= 2) {
                    availability.inverted = 'possible';
                }
                else {
                    availability.inverted = 'glitchpossible'
                }
            }
            else {
                if (trackerData.items.lantern && (trackerData.items.cape || trackerData.items.byrna || canBlockLasers())) {
                    availability.inverted = 'possible';
                }
                else if (trackerData.dungeonchests[9] >= 4) {
                    availability.inverted = 'possible';
                }
                else {
                    availability.inverted = 'glitchpossible';
                }
            }
        }
        else if (this.mayEnter('inverted', false)) {
            if (trackerData.items.firerod) {
                if (trackerData.items.lantern && (trackerData.items.cape || trackerData.items.byrna || canBlockLasers())) {
                    availability.inverted = 'possible';
                }
                else if (trackerData.dungeonchests[9] >= 2) {
                    availability.inverted = 'possible';
                }
                else {
                    availability.inverted = 'glitchpossible'
                }
            }
            else {
                if (trackerData.items.lantern && (trackerData.items.cape || trackerData.items.byrna || canBlockLasers())) {
                    availability.inverted = 'possible';
                }
                else if (trackerData.dungeonchests[9] >= 4) {
                    availability.inverted = 'possible';
                }
                else {
                    availability.inverted = 'glitchpossible';
                }
            }
        }
        else if (this.canEnter('inverted', true)) {
            if (trackerData.items.firerod) {
                if (trackerData.dungeonchests[9] >= 2
                        || this.isBeatable().glitchless === 'available'
                        || this.isBeatable().glitchless === 'glitchavailable') {
                    availability.inverted = 'glitchavailable';
                }
                else {
                    availability.inverted = 'glitchpossible';
                }
            }
            else {
                availability.inverted = 'glitchpossible';
            }
        }
        else if (this.mayEnter('inverted', true)) {
            availability.inverted = 'glitchpossible';
        }
        return availability;
    }
};

dungeons[10] = {
    name: "Ganon's Tower",
    label: "GT",
    x: "77.0%",
    y: "5.5%",
    image: "boss102.png",
    canEnter: function (logic, allowOutOfLogicGlitches) {
        if (logic === 'majorGlitches') {
            return canEnterWestDeathMountain(logic, allowOutOfLogicGlitches);
        }
        else if (logic === 'owGlitches') {
            return trackerData.items.boots && trackerData.items.moonpearl;
        }
        else if (logic === 'glitchless') {
            let crystalCount = 0;
            for (let k = 0; k < 10; k++) {
                if ((trackerData.prizes[k] === 4 || trackerData.prizes[k] === 3) && trackerData.items["boss" + k] === 2) {
                    crystalCount++;
                    if (crystalCount === 7) {
                        break;
                    }
                }
            }
            return crystalCount === 7 && trackerData.items.moonpearl && canEnterDarkWorldDeathMountain(logic, allowOutOfLogicGlitches);
        }
        else if (logic === 'inverted') {
            let crystalCount = 0;
            for (let k = 0; k < 10; k++) {
                if ((trackerData.prizes[k] === 4 || trackerData.prizes[k] === 3) && trackerData.items["boss" + k] === 2) {
                    crystalCount++;
                    if (crystalCount === 7) {
                        break;
                    }
                }
            }
            return crystalCount === 7 && canEnterLightWorld('inverted', false, allowOutOfLogicGlitches);
        }
    },
    isBeatable: function () {
        const availability = new Availability();
        if (trackerData.items.hookshot
                && trackerData.items.bow
                && canLightTorches()
                && (trackerData.items.hammer || trackerData.items.sword >= 1)) {
            if (this.canEnter('glitchless', false)) {
                if (trackerData.items.boots && trackerData.items.hammer && trackerData.items.firerod && trackerData.items.somaria) {
                    availability.glitchless = 'available';
                }
                else {
                    availability.glitchless = 'possible';
                }
            }
            else if (this.canEnter('glitchless', true)) {
                if (trackerData.items.boots && trackerData.items.hammer && trackerData.items.firerod && trackerData.items.somaria) {
                    availability.glitchless = 'glitchavailable';
                }
                else {
                    availability.glitchless = 'glitchpossible';
                }
            }
            if (this.canEnter('owGlitches', false)) {
                if (trackerData.items.boots && trackerData.items.hammer && trackerData.items.firerod && trackerData.items.somaria) {
                    availability.owGlitches = 'available';
                }
                else {
                    availability.owGlitches = 'possible';
                }
            }
            if (this.canEnter('majorGlitches', false)) {
                if (trackerData.items.boots && trackerData.items.hammer && trackerData.items.firerod && trackerData.items.somaria) {
                    availability.majorGlitches = 'available';
                }
                else {
                    availability.majorGlitches = 'possible';
                }
            }
            else if (this.canEnter('majorGlitches', true)) {
                if (trackerData.items.boots && trackerData.items.hammer && trackerData.items.firerod && trackerData.items.somaria) {
                    availability.majorGlitches = 'glitchavailable';
                }
                else {
                    availability.majorGlitches = 'glitchpossible';
                }
            }
            if (this.canEnter('inverted', false)) {
                if (trackerData.items.boots && trackerData.items.hammer && trackerData.items.firerod && trackerData.items.somaria) {
                    availability.inverted = 'available';
                }
                else {
                    availability.inverted = 'possible';
                }
            }
            else if (this.canEnter('inverted', true)) {
                if (trackerData.items.boots && trackerData.items.hammer && trackerData.items.firerod && trackerData.items.somaria) {
                    availability.inverted = 'glitchavailable';
                }
                else {
                    availability.inverted = 'glitchpossible';
                }
            }
        }
        return availability;
    },
    canGetChest: function () {
        const availability = new Availability();
        let smallKeysNeeded = 0;
        let bigKeyNeeded = 0;
        let bigKeyGuaranteed = false;
        // Hope Room x2
        let minAvailableChests = 2;
        let maxAvailableChests = 2;
        // Bob's Torch
        if (trackerData.items.boots) {
            minAvailableChests++;
            maxAvailableChests++;
        }
        // DMs Room x4 + Randomizer Room x4 + Firesnake Room
        if (trackerData.items.hammer && trackerData.items.hookshot) {
            minAvailableChests += 9;
            maxAvailableChests += 9;
            smallKeysNeeded = 4;
        }
        // Map Chest
        if (trackerData.items.hammer
                && (trackerData.items.boots || trackerData.items.hookshot)) {
            minAvailableChests++;
            maxAvailableChests++;
        }
        // Bob's Chest + Big Key Room x3
        if ((trackerData.items.hammer && trackerData.items.hookshot)
                || (trackerData.items.firerod && trackerData.items.somaria)) {
            minAvailableChests += 4;
            maxAvailableChests += 4;
            smallKeysNeeded = Math.max(3, smallKeysNeeded);
        }
        // Tile Room
        if (trackerData.items.somaria) {
            minAvailableChests++;
            maxAvailableChests++;
        }
        // Compass Room x4
        if (trackerData.items.firerod && trackerData.items.somaria) {
            minAvailableChests += 4;
            maxAvailableChests += 4;
            smallKeysNeeded = Math.max(4, smallKeysNeeded);
        }
        // Big Chest
        if (trackerData.items.hammer
                && trackerData.items.boots
                && trackerData.items.hookshot
                && trackerData.items.somaria
                && trackerData.items.firerod) {
            minAvailableChests++;
            maxAvailableChests++;
            bigKeyNeeded = 1;
            bigKeyGuaranteed = true;
        }
        // Mini Helmasaur Room x2 + Pre-Moldorm Chest
        if (trackerData.items.bow && canLightTorches()) {
            if (bigKeyGuaranteed) {
                minAvailableChests += 3;
            }
            maxAvailableChests += 3;
            smallKeysNeeded = Math.max(3, smallKeysNeeded);
            bigKeyNeeded = 1;
        }
        // Moldorm Chest
        if (trackerData.items.hookshot
                && trackerData.items.bow
                && canLightTorches()
                && (trackerData.items.hammer || trackerData.items.sword >= 1)) {
            if (bigKeyGuaranteed) {
                minAvailableChests++;
            }
            maxAvailableChests++;
            smallKeysNeeded = Math.max(4, smallKeysNeeded);
            bigKeyNeeded = 1;
        }
        let maxItemsAvailable = Math.min(20, maxAvailableChests - smallKeysNeeded - bigKeyNeeded);
        // 4 keys + big key + map + compass
        let minItemsAvailable = Math.max(0, minAvailableChests - 7);
        if (this.canEnter('glitchless', false)) {
            if (trackerData.dungeonchests[10] > (20 - minItemsAvailable)) {
                availability.glitchless = 'available';
            }
            else if (trackerData.dungeonchests[10] > (20 - maxItemsAvailable)) {
                availability.glitchless = 'possible';
            }
        }
        else if (this.canEnter('glitchless', true)) {
            if (trackerData.dungeonchests[10] > (20 - minItemsAvailable)) {
                availability.glitchless = 'glitchavailable';
            }
            else if (trackerData.dungeonchests[10] > (20 - maxItemsAvailable)) {
                availability.glitchless = 'glitchpossible';
            }
        }
        if (this.canEnter('owGlitches', false)) {
            if (trackerData.dungeonchests[10] > (20 - minItemsAvailable)) {
                availability.owGlitches = 'available';
            }
            else if (trackerData.dungeonchests[10] > (20 - maxItemsAvailable)) {
                availability.owGlitches = 'possible';
            }
        }
        if (this.canEnter('majorGlitches', false)) {
            if (trackerData.dungeonchests[10] > (20 - minItemsAvailable)) {
                availability.majorGlitches = 'available';
            }
            else if (trackerData.dungeonchests[10] > (20 - maxItemsAvailable)) {
                availability.majorGlitches = 'possible';
            }
        }
        else if (this.canEnter('majorGlitches', true)) {
            if (trackerData.dungeonchests[10] > (20 - minItemsAvailable)) {
                availability.majorGlitches = 'glitchavailable';
            }
            else if (trackerData.dungeonchests[10] > (20 - maxItemsAvailable)) {
                availability.majorGlitches = 'glitchpossible';
            }
        }
        if (this.canEnter('inverted', false)) {
            if (trackerData.dungeonchests[10] > (20 - minItemsAvailable)) {
                availability.inverted = 'available';
            }
            else if (trackerData.dungeonchests[10] > (20 - maxItemsAvailable)) {
                availability.inverted = 'possible';
            }
        }
        else if (this.canEnter('inverted', true)) {
            if (trackerData.dungeonchests[10] > (20 - minItemsAvailable)) {
                availability.inverted = 'glitchavailable';
            }
            else if (trackerData.dungeonchests[10] > (20 - maxItemsAvailable)) {
                availability.inverted = 'glitchpossible';
            }
        }
        return availability;
    }
};

//define overworld chests
const chests = [];

chests[0] = {
    name: "King's Tomb <img src='/images/boots.png' class='mini'> + <img src='/images/glove2.png' class='mini'>/<img src='/images/mirror.png' class='mini'>",
    x: "30.8%",
    y: "29.6%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (trackerData.items.boots && canLiftDarkRocks()) {
            availability.glitchless = "available";
            if (trackerData.items.moonpearl) {
                availability.inverted = "available";
            }
        }
        else if (trackerData.items.boots && trackerData.items.mirror) {
            if (trackerData.items.moonpearl) {
                if (canEnterNorthWestDarkWorld('glitchless', false, false)) {
                    availability.glitchless = "available";
                }
                else if (canEnterNorthWestDarkWorld('glitchless', true, false)) {
                    availability.glitchless = "agahnim";
                }
                else if (canEnterNorthWestDarkWorld('glitchless', true, true)) {
                    availability.glitchless = "glitchagahnim";
                }
                else {
                    availability.owGlitches = "available";
                }
            }
            else if (glitchedLinkInDarkWorld()) {
                availability.majorGlitches = "available";
            }
        }
        return availability;
    }
};

chests[1] = {
    name: "Light World Swamp (2)",
    x: "23.4%",
    y: "93.4%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = "available";
        if (canEnterLightWorld('inverted', false, false)) {
            if (trackerData.items.moonpearl) {
                availability.inverted = "available";
            }
            else if (trackerData.items.mirror) {
                availability.inverted = "glitchavailable";
            }
        }
        else if (canEnterLightWorld('inverted', true, false) && trackerData.items.moonpearl) {
            availability.inverted = "agahnim";
        }
        else if (canEnterLightWorld('inverted', true, true) && (trackerData.items.moonpearl || trackerData.items.mirror)) {
            availability.inverted = "glitchagahnim";
        }
        return availability;
    }
};

chests[2] = {
    name: "Link's House",
    x: "27.4%",
    y: "67.9%",
    isOpened: true,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = "available";
        availability.inverted = "available";
        return availability;
    }
};

chests[3] = {
    name: "Spiral Cave",
    x: "39.9%",
    y: "9.3%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canEnterEastDeathMountain("glitchless", false)) {
            availability.glitchless = "available";
        }
        else if (canEnterEastDeathMountain("glitchless", true)) {
            availability.glitchless = "glitchavailable";
        }
        if (canEnterEastDeathMountain("owGlitches", false)) {
            availability.owGlitches = "available";
        }
        else if (canEnterEastDeathMountain("owGlitches", true)) {
            availability.owGlitches = "glitchavailable";
        }
        if (canEnterEastDeathMountain("majorGlitches", false)) {
            availability.majorGlitches = "available";
        }
        else if (canEnterEastDeathMountain("majorGlitches", true)) {
            availability.majorGlitches = "glitchavailable";
        }
        if (canEnterEastDeathMountain("inverted", false) && trackerData.items.moonpearl) {
            availability.inverted = "available";
        }
        else if (canEnterEastDeathMountain("inverted", true)) {
            availability.inverted = "glitchavailable";
        }
        return availability;
    }
};

chests[4] = {
    name: "Mimic Cave (<img src='/images/mirror.png' class='mini'> outside of Turtle Rock)(Yellow = <img src='/images/medallion0.png' class='mini'> unknown OR possible w/out <img src='/images/firerod.png' class='mini'>)",
    x: "42.6%",
    y: "9.3%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canEnterEastDeathMountain("glitchless", false) && trackerData.items.mirror && dungeons[9].mayEnter("glitchless", false)) {
            if (trackerData.items.firerod && dungeons[9].canEnter("glitchless", false)) {
                availability.glitchless = "available";
            }
            else {
                availability.glitchless = "possible";
            }
        }
        else if (canEnterEastDeathMountain("glitchless", true) && trackerData.items.mirror && dungeons[9].mayEnter("glitchless", true)) {
            if (trackerData.items.firerod && dungeons[9].canEnter("glitchless", true)) {
                availability.glitchless = "glitchavailable";
            }
            else {
                availability.glitchless = "glitchpossible";
            }
        }
        if (trackerData.items.hammer && trackerData.items.mirror) {
            if (canEnterEastDeathMountain("owGlitches", false) && canEnterDarkWorldDeathMountain("owGlitches", false)) {
                availability.owGlitches = "available";
            }
            else if (canEnterEastDeathMountain("owGlitches", true) && canEnterDarkWorldDeathMountain("owGlitches", true)) {
                availability.owGlitches = "glitchavailable";
            }
            if (canEnterEastDeathMountain("majorGlitches", false) && canEnterDarkWorldDeathMountain("majorGlitches", false)) {
                availability.majorGlitches = "available";
            }
            else if (canEnterEastDeathMountain("majorGlitches", true) && canEnterDarkWorldDeathMountain("majorGlitches", true)) {
                availability.majorGlitches = "glitchavailable";
            }
        }
        if (trackerData.items.moonpearl && trackerData.items.hammer) {
            if (canEnterEastDeathMountain('inverted', false)) {
                availability.inverted = "available";
            }
            else if (canEnterEastDeathMountain('inverted', true)) {
                availability.inverted = "glitchavailable";
            }
        }
        return availability;
    }
};

chests[5] = {
    name: "Tavern",
    x: "8.1%",
    y: "57.8%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = "available";
        if (canEnterLightWorld('inverted', false, false)) {
            if (trackerData.items.moonpearl) {
                availability.inverted = "available";
            }
            else {
                availability.inverted = "glitchavailable";
            }
        }
        else if (canEnterLightWorld('inverted', true, false) && trackerData.items.moonpearl) {
            availability.inverted = "agahnim";
        }
        else if (canEnterLightWorld('inverted', true, true)) {
            availability.inverted = "glitchagahnim";
        }
        return availability;
    }
};

chests[6] = {
    name: "Chicken House <img src='/images/bomb.png' class='mini'>",
    x: "4.4%",
    y: "54.2%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = "available";
        if (trackerData.items.moonpearl) {
            if (canEnterLightWorld('inverted', false, false)) {
                availability.inverted = "available";
            }
            else if (canEnterLightWorld('inverted', true, false)) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
        return availability;
    }
};

chests[7] = {
    name: "Bombable Hut <img src='/images/bomb.png' class='mini'>",
    x: "55.4%",
    y: "57.8%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canEnterNorthWestDarkWorld("glitchless", false, false)) {
            availability.glitchless = "available";
        }
        else if (canEnterNorthWestDarkWorld("glitchless", true, false)) {
            availability.glitchless = "agahnim";
        }
        else if (canEnterNorthWestDarkWorld("glitchless", true, true)) {
            availability.glitchless = "glitchagahnim";
        }
        if (trackerData.items.moonpearl) {
            if (canEnterNorthWestDarkWorld("owGlitches", false, false)) {
                availability.owGlitches = "available";
            }
            else if (canEnterNorthWestDarkWorld("owGlitches", true, false)) {
                availability.owGlitches = "agahnim";
            }
            else if (canEnterNorthWestDarkWorld("owGlitches", true, true)) {
                availability.owGlitches = "glitchagahnim";
            }
        }
        if (glitchedLinkInDarkWorld()) {
            if (canEnterNorthWestDarkWorld("majorGlitches", false, false)) {
                availability.majorGlitches = "available";
            }
            else if (canEnterNorthWestDarkWorld("majorGlitches", true, false)) {
                availability.majorGlitches = "agahnim";
            }
            else if (canEnterNorthWestDarkWorld("majorGlitches", true, true)) {
                availability.majorGlitches = "glitchagahnim";
            }
        }
        availability.inverted = "available";
        return availability;
    }
};

chests[8] = {
    name: "C House",
    x: "60.8%",
    y: "47.9%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canEnterNorthWestDarkWorld("glitchless", false, false)) {
            availability.glitchless = "available";
        }
        else if (canEnterNorthWestDarkWorld("glitchless", true, false)) {
            availability.glitchless = "agahnim";
        }
        else if (canEnterNorthWestDarkWorld("glitchless", true, true)) {
            availability.glitchless = "glitchagahnim";
        }
        if (canEnterNorthWestDarkWorld("owGlitches", false, false)) {
            availability.owGlitches = "available";
        }
        else if (canEnterNorthWestDarkWorld("owGlitches", true, false)) {
            availability.owGlitches = "agahnim";
        }
        else if (canEnterNorthWestDarkWorld("owGlitches", true, true)) {
            availability.owGlitches = "glitchagahnim";
        }
        if (canEnterNorthWestDarkWorld("majorGlitches", false, false)) {
            availability.majorGlitches = "available";
        }
        else if (canEnterNorthWestDarkWorld("majorGlitches", true, false)) {
            availability.majorGlitches = "agahnim";
        }
        else if (canEnterNorthWestDarkWorld("majorGlitches", true, true)) {
            availability.majorGlitches = "glitchagahnim";
        }
        availability.inverted = "available";
        return availability;
    }
};

chests[9] = {
    name: "Aginah's Cave <img src='/images/bomb.png' class='mini'>",
    x: "10.0%",
    y: "82.6%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = "available";
        if (trackerData.items.moonpearl) {
            if (canEnterLightWorld('inverted', false, false)) {
                availability.inverted = "available";
            }
            else if (canEnterLightWorld('inverted', true, false)) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
        return availability;
    }
};

chests[10] = {
    name: "West of Mire (2)",
    x: "51.7%",
    y: "79.5%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canEnterMireArea('glitchless', false, false)) {
            if (trackerData.items.moonpearl) {
                availability.glitchless = 'available';
            }
            else if (trackerData.items.mirror) {
                availability.glitchless = 'glitchavailable';
            }
        }
        if (trackerData.items.moonpearl || trackerData.items.mirror) {
            if (canEnterMireArea('owGlitches', false, false)) {
                availability.owGlitches = 'available';
            }
            else if (canEnterMireArea('owGlitches', true, false)) {
                availability.owGlitches = 'agahnim';
            }
            else if (canEnterMireArea('owGlitches', true, true)) {
                availability.owGlitches = 'glitchagahnim';
            }
        }
        if (canEnterMireArea('majorGlitches', false, false)) {
            availability.majorGlitches = 'available';
        }
        else if (canEnterMireArea('majorGlitches', true, false)) {
            availability.majorGlitches = 'agahnim';
        }
        else if (canEnterMireArea('majorGlitches', true, true)) {
            availability.majorGlitches = 'glitchagahnim';
        }
        if (canEnterMireArea('inverted', false, false)) {
            availability.inverted = 'available';
        }
        else if (canEnterMireArea('inverted', false, true)) {
            availability.inverted = 'glitchavailable';
        }
        else if (canEnterMireArea('inverted', true, false)) {
            availability.inverted = 'agahnim';
        }
        else if (canEnterMireArea('inverted', true, true)) {
            availability.inverted = 'glitchagahnim';
        }
        return availability;
    }
};

chests[11] = {
    name: "DW Death Mountain (2) : Don't need <img src='/images/moonpearl.png' class='mini'>",
    x: "92.8%",
    y: "14.7%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canEnterDarkWorldDeathMountain('glitchless', true)) {
            if (canEnterDarkWorldDeathMountain('glitchless', false) && trackerData.items.moonpearl) {
                availability.glitchless = 'available';
            }
            else {
                availability.glitchless = 'glitchavailable'
            }
        }
        if (canEnterDarkWorldDeathMountain('owGlitches', false)) {
            availability.owGlitches = 'available';
        }
        else if (canEnterDarkWorldDeathMountain('owGlitches', true)) {
            availability.owGlitches = 'glitchavailable';
        }
        if (canEnterDarkWorldDeathMountain('majorGlitches', false)) {
            availability.majorGlitches = 'available';
        }
        else if (canEnterDarkWorldDeathMountain('majorGlitches', true)) {
            availability.majorGlitches = 'glitchavailable';
        }
        if (canEnterDarkWorldDeathMountain('inverted', false)) {
            availability.inverted = 'available';
        }
        else if (canEnterDarkWorldDeathMountain('inverted', true)) {
            availability.inverted = 'glitchavailable';
        }
        return availability;
    }
};

chests[12] = {
    name: "Sahasrahla's Hut (3) <img src='/images/bomb.png' class='mini'>/<img src='/images/boots.png' class='mini'>",
    x: "40.7%",
    y: "41.4%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = "available";
        if (canEnterLightWorld('inverted', false, false)) {
            if (trackerData.items.moonpearl) {
                availability.inverted = "available";
            }
            else if (trackerData.items.boots) {
                availability.inverted = "glitchavailable";
            }
        }
        else if (canEnterLightWorld('inverted', true, false) && trackerData.items.moonpearl) {
            availability.inverted = "agahnim";
        }
        else if (canEnterLightWorld('inverted', true, true) && (trackerData.items.moonpearl || trackerData.items.boots)) {
            availability.inverted = "glitchagahnim";
        }
        return availability;
    }
};

chests[13] = {
    name: "Byrna Spike Cave",
    x: "78.6%",
    y: "14.9%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canLiftRocks() && trackerData.items.hammer) {
            if (canEnterWestDeathMountain('glitchless', true) && trackerData.items.moonpearl) {
                if (canExtendMagic() && (trackerData.items.cape || trackerData.items.byrna)) {
                    if (canEnterWestDeathMountain('glitchless', false)) {
                        availability.glitchless = 'available';
                    }
                    else {
                        availability.glitchless = 'glitchavailable';
                    }
                }
                else {
                    availability.glitchless = 'glitchpossible';
                }
            }
            if (canEnterWestDeathMountain('owGlitches', true) && trackerData.items.moonpearl) {
                if (canExtendMagic() && (trackerData.items.cape || trackerData.items.byrna)) {
                    if (canEnterWestDeathMountain('owGlitches', false)) {
                        availability.owGlitches = 'available';
                    }
                    else {
                        availability.owGlitches = 'glitchavailable';
                    }
                }
                else {
                    availability.owGlitches = 'glitchpossible';
                }
            }
            else if (canEnterWestDeathMountain('majorGlitches', true) && (trackerData.items.moonpearl || (trackerData.items.bottle >= 1 && trackerData.items.boots))) {
                if (canExtendMagic() && (trackerData.items.cape || trackerData.items.byrna)) {
                    if (canEnterWestDeathMountain('majorGlitches', false)) {
                        availability.majorGlitches = 'available';
                    }
                    else {
                        availability.majorGlitches = 'glitchavailable';
                    }
                }
                else {
                    availability.majorGlitches = 'glitchpossible';
                }
            }
            if (canEnterWestDeathMountain('inverted', true)) {
                if (canExtendMagic() && (trackerData.items.cape || trackerData.items.byrna)) {
                    if (canEnterWestDeathMountain('inverted', false)) {
                        availability.inverted = 'available';
                    }
                    else {
                        availability.inverted = 'glitchavailable';
                    }
                }
                else {
                    availability.inverted = 'glitchpossible';
                }
            }
        }
        return availability;
    }
};

chests[14] = {
    name: "Kakariko Well (4 + <img src='/images/bomb.png' class='mini'>)",
    x: "1.7%",
    y: "41.0%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = "available";
        if (canEnterLightWorld('inverted', false, false)) {
            if (trackerData.items.moonpearl) {
                availability.inverted = "available";
            }
            else {
                availability.inverted = "glitchpossible";
            }
        }
        else if (canEnterLightWorld('inverted', true, false) && trackerData.items.moonpearl) {
            availability.inverted = "agahnim";
        }
        else if (canEnterLightWorld('inverted', true, true)) {
            availability.inverted = "glitchagahnim";
        }
        return availability;
    }
};

chests[15] = {
    name: "Thieves' Hut (4 + <img src='/images/bomb.png' class='mini'>)",
    x: "6.4%",
    y: "41.0%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = "available";
        if (canEnterLightWorld('inverted', false, false)) {
            if (trackerData.items.moonpearl) {
                availability.inverted = "available";
            }
            else if (trackerData.items.mirror) {
                availability.inverted = "glitchpossible";
            }
        }
        else if (canEnterLightWorld('inverted', true, false) && trackerData.items.moonpearl) {
            availability.inverted = "agahnim";
        }
        else if (canEnterLightWorld('inverted', true, true) && (trackerData.items.moonpearl || trackerData.items.mirror)) {
            availability.inverted = "glitchagahnim";
        }
        return availability;
    }
};

chests[16] = {
    name: "Hype Cave! <img src='/images/bomb.png' class='mini'> (NPC + 4 <img src='/images/bomb.png' class='mini'>)",
    x: "80.0%",
    y: "77.1%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canEnterSouthDarkWorld("glitchless", false, false)) {
            availability.glitchless = "available";
        }
        else if (canEnterSouthDarkWorld("glitchless", true, false)) {
            availability.glitchless = "agahnim";
        }
        else if (canEnterSouthDarkWorld("glitchless", true, true)) {
            availability.glitchless = "glitchagahnim";
        }
        if (trackerData.items.moonpearl) {
            if (canEnterSouthDarkWorld("owGlitches", false, false)) {
                availability.owGlitches = "available";
            }
            else if (canEnterSouthDarkWorld("owGlitches", true, false)) {
                availability.owGlitches = "agahnim";
            }
            else if (canEnterSouthDarkWorld("owGlitches", true, true)) {
                availability.owGlitches = "glitchagahnim";
            }
        }
        if (glitchedLinkInDarkWorld()) {
            if (canEnterSouthDarkWorld("majorGlitches", false, false)) {
                availability.majorGlitches = "available";
            }
            else if (canEnterSouthDarkWorld("majorGlitches", true, false)) {
                availability.majorGlitches = "agahnim";
            }
            else if (canEnterSouthDarkWorld("majorGlitches", true, true)) {
                availability.majorGlitches = "glitchagahnim";
            }
        }
        availability.inverted = "available";
        return availability;
    }
};

chests[17] = {
    name: "Death Mountain East (5 + 2 <img src='/images/bomb.png' class='mini'>)",
    x: "41.4%",
    y: "17.1%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canEnterEastDeathMountain("glitchless", false)) {
            availability.glitchless = "available";
        }
        else if (canEnterEastDeathMountain("glitchless", true)) {
            availability.glitchless = "glitchavailable";
        }
        if (canEnterEastDeathMountain("owGlitches", false)) {
            availability.owGlitches = "available";
        }
        else if (canEnterEastDeathMountain("owGlitches", true)) {
            availability.owGlitches = "glitchavailable";
        }
        if (canEnterEastDeathMountain("majorGlitches", false)) {
            availability.majorGlitches = "available";
        }
        else if (canEnterEastDeathMountain("majorGlitches", true)) {
            availability.majorGlitches = "glitchavailable";
        }
        if (canEnterEastDeathMountain('inverted', false) && trackerData.items.moonpearl) {
            availability.inverted = 'available';
        }
        else if (canEnterEastDeathMountain('inverted', true) && trackerData.items.moonpearl) {
            availability.inverted = 'glitchavailable';
        }
        else if (canEnterEastDeathMountain('inverted', true) && trackerData.items.sword >= 2) {
            availability.inverted = 'glitchpossible';
        }
        return availability;
    }
};

chests[18] = {
    name: "West of Sanctuary <img src='/images/boots.png' class='mini'>",
    x: "19.5%",
    y: "29.3%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (trackerData.items.boots) {
            availability.glitchless = 'available';
            if (trackerData.items.moonpearl) {
                if (canEnterLightWorld('inverted', false, false)) {
                    availability.inverted = "available";
                }
                else if (canEnterLightWorld('inverted', true, false)) {
                    availability.inverted = "agahnim";
                }
                else if (canEnterLightWorld('inverted', true, true)) {
                    availability.inverted = "glitchagahnim";
                }
            }
        }
        return availability;
    }
};

chests[19] = {
    name: "Minimoldorm Cave (NPC + 4) <img src='/images/bomb.png' class='mini'>",
    x: "32.6%",
    y: "93.4%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = "available";
        if (trackerData.items.moonpearl) {
            if (canEnterLightWorld('inverted', false, false)) {
                availability.inverted = "available";
            }
            else if (canEnterLightWorld('inverted', true, false)) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
        return availability;
    }
};

chests[20] = {
    name: "Ice Rod Cave <img src='/images/bomb.png' class='mini'>",
    x: "44.7%",
    y: "76.9%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = "available";
        if (trackerData.items.moonpearl) {
            if (canEnterLightWorld('inverted', false, false)) {
                availability.inverted = "available";
            }
            else if (canEnterLightWorld('inverted', true, false)) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
        return availability;
    }
};

chests[21] = {
    name: "Cave Under Rock (bottom chest) <img src='/images/hookshot.png' class='mini'>/<img src='/images/boots.png' class='mini'>",
    x: "91.6%",
    y: "8.6%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (trackerData.items.moonpearl
                && (trackerData.items.hookshot || trackerData.items.boots)) {
            if (canEnterDarkWorldDeathMountain('glitchless', false)) {
                availability.glitchless = 'available';
            }
            else if (canEnterDarkWorldDeathMountain('glitchless', true)) {
                availability.glitchless = 'glitchavailable';
            }
            if (canLiftRocks()) {
                if (canEnterDarkWorldDeathMountain('owGlitches', false)) {
                    availability.owGlitches = 'available';
                }
                else if (canEnterDarkWorldDeathMountain('owGlitches', true)) {
                    availability.owGlitches = 'glitchavailable';
                }
                if (canEnterDarkWorldDeathMountain('majorGlitches', false)) {
                    availability.majorGlitches = 'available';
                }
                else if (canEnterDarkWorldDeathMountain('majorGlitches', true)) {
                    availability.majorGlitches = 'glitchavailable';
                }
            }
        }
        if (canLiftRocks() && (trackerData.items.hookshot || trackerData.items.boots)) {
            if (canEnterDarkWorldDeathMountain('inverted', false)) {
                availability.inverted = 'available';
            }
            else if (canEnterDarkWorldDeathMountain('inverted', true)) {
                availability.inverted = 'glitchavailable';
            }
        }
        return availability;
    }
};

chests[22] = {
    name: "Cave Under Rock (3 top chests) <img src='/images/hookshot.png' class='mini'>",
    x: "91.6%",
    y: "3.4%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (trackerData.items.moonpearl && trackerData.items.hookshot) {
            if (canEnterDarkWorldDeathMountain('glitchless', false)) {
                availability.glitchless = 'available';
            }
            else if (canEnterDarkWorldDeathMountain('glitchless', true)) {
                availability.glitchless = 'glitchavailable';
            }
            if (canLiftRocks()) {
                if (canEnterDarkWorldDeathMountain('owGlitches', false)) {
                    availability.owGlitches = 'available';
                }
                else if (canEnterDarkWorldDeathMountain('owGlitches', true)) {
                    availability.owGlitches = 'glitchavailable';
                }
                if (canEnterDarkWorldDeathMountain('majorGlitches', false)) {
                    availability.majorGlitches = 'available';
                }
                else if (canEnterDarkWorldDeathMountain('majorGlitches', true)) {
                    availability.majorGlitches = 'glitchavailable';
                }
            }
        }
        if (canLiftRocks() && trackerData.items.hookshot) {
            if (canEnterDarkWorldDeathMountain('inverted', false)) {
                availability.inverted = 'available';
            }
            else if (canEnterDarkWorldDeathMountain('inverted', true)) {
                availability.inverted = 'glitchavailable';
            }
        }
        return availability;
    }
};

chests[23] = {
    name: "Treasure Chest Minigame: Pay 30 rupees",
    x: "52.1%",
    y: "46.4%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canEnterNorthWestDarkWorld("glitchless", false, false)) {
            availability.glitchless = "available";
        }
        else if (canEnterNorthWestDarkWorld("glitchless", true, false)) {
            availability.glitchless = "agahnim";
        }
        else if (canEnterNorthWestDarkWorld("glitchless", true, true)) {
            availability.glitchless = "glitchagahnim";
        }
        if (canEnterNorthWestDarkWorld("owGlitches", false, false)) {
            availability.owGlitches = "available";
        }
        else if (canEnterNorthWestDarkWorld("owGlitches", true, false)) {
            availability.owGlitches = "agahnim";
        }
        else if (canEnterNorthWestDarkWorld("owGlitches", true, true)) {
            availability.owGlitches = "glitchagahnim";
        }
        if (canEnterNorthWestDarkWorld("majorGlitches", false, false)) {
            availability.majorGlitches = "available";
        }
        else if (canEnterNorthWestDarkWorld("majorGlitches", true, false)) {
            availability.majorGlitches = "agahnim";
        }
        else if (canEnterNorthWestDarkWorld("majorGlitches", true, true)) {
            availability.majorGlitches = "glitchagahnim";
        }
        availability.inverted = 'available';
        return availability;
    }
};

chests[24] = {
    name: "Bottle Vendor: Pay 100 rupees",
    x: "4.5%",
    y: "46.8%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = "available";
        if (canEnterLightWorld('inverted', false, false)) {
            availability.inverted = "available";
        }
        else if (canEnterLightWorld('inverted', true, false)) {
            availability.inverted = "agahnim";
        }
        else if (canEnterLightWorld('inverted', true, true)) {
            availability.inverted = "glitchagahnim";
        }
        return availability;
    }
};

chests[25] = {
    name: "Sahasrahla <img src='/images/pendant0.png' class='mini'>",
    x: "40.7%",
    y: "46.7%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        for (let k = 0; k < 10; k++) {
            if (trackerData.prizes[k] === 1 && trackerData.items["boss" + k] === 2) {
                availability.glitchless = "available";
                if (canEnterLightWorld('inverted', false, false)) {
                    availability.inverted = "available";
                }
                else if (canEnterLightWorld('inverted', true, false)) {
                    availability.inverted = "agahnim";
                }
                else if (canEnterLightWorld('inverted', true, true)) {
                    availability.inverted = "glitchagahnim";
                }
                break;
            }
        }
        return availability;
    }
};

chests[26] = {
    name: "Stump Kid",
    x: "65.5%",
    y: "68.6%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canEnterSouthDarkWorld("glitchless", false, false)) {
            availability.glitchless = "available";
        }
        else if (canEnterSouthDarkWorld("glitchless", true, false)) {
            availability.glitchless = "agahnim";
        }
        else if (canEnterSouthDarkWorld("glitchless", true, true)) {
            availability.glitchless = "glitchagahnim";
        }
        if (trackerData.items.moonpearl) {
            if (canEnterSouthDarkWorld("owGlitches", false, false)) {
                availability.owGlitches = "available";
            }
            else if (canEnterSouthDarkWorld("owGlitches", true, false)) {
                availability.owGlitches = "agahnim";
            }
            else if (canEnterSouthDarkWorld("owGlitches", true, true)) {
                availability.owGlitches = "glitchagahnim";
            }
        }
        if (glitchedLinkInDarkWorld()) {
            if (canEnterSouthDarkWorld("majorGlitches", false, false)) {
                availability.majorGlitches = "available";
            }
            else if (canEnterSouthDarkWorld("majorGlitches", true, false)) {
                availability.majorGlitches = "agahnim";
            }
            else if (canEnterSouthDarkWorld("majorGlitches", true, true)) {
                availability.majorGlitches = "glitchagahnim";
            }
        }
        availability.inverted = "available";
        return availability;
    }
};

chests[27] = {
    name: "Bug Kid <img src='/images/bottle0.png' class='mini'>",
    x: "7.8%",
    y: "52.1%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (trackerData.items.bottle >= 1) {
            availability.glitchless = "available";
            if (canEnterLightWorld('inverted', false, false)) {
                availability.inverted = "available";
            }
            else if (canEnterLightWorld('inverted', true, false)) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
        return availability;
    }
};

chests[28] = {
    name: "Show the Purple Chest to Gary",
    x: "65.2%",
    y: "52.2%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canLiftDarkRocks()) {
            if (canEnterNorthWestDarkWorld('glitchless', false, false)) {
                availability.glitchless = 'available';
            }
            else if (canEnterNorthWestDarkWorld('glitchless', true, false)) {
                availability.glitchless = 'agahnim';
            }
            else if (canEnterNorthWestDarkWorld('glitchless', true, true)) {
                availability.glitchless = 'glitchagahnim';
            }
        }
        if (trackerData.items.moonpearl) {
            if (canEnterNorthWestDarkWorld('owGlitches', false, false)
                    && chests[60].isAvailable().owGlitches === 'available'
                    && (canLiftDarkRocks() || (trackerData.items.boots && canEnterNorthEastDarkWorld('owGlitches', false, false)))) {
                availability.owGlitches = 'available';
            }
            else if (canEnterNorthWestDarkWorld('owGlitches', true, false)
                    && (chests[60].isAvailable().owGlitches === 'available' || chests[60].isAvailable().owGlitches === 'agahnim')
                    && (canLiftDarkRocks() || (trackerData.items.boots && canEnterNorthEastDarkWorld('owGlitches', true, false)))) {
                availability.owGlitches = 'agahnim';
            }
            else if (canEnterNorthWestDarkWorld('owGlitches', true, true)
                    && (chests[60].isAvailable().owGlitches === 'available' || chests[60].isAvailable().owGlitches === 'agahnim' || chests[60].isAvailable().owGlitches === 'glitchagahnim')
                    && (canLiftDarkRocks() || (trackerData.items.boots && canEnterNorthEastDarkWorld('owGlitches', true, true)))) {
                availability.owGlitches = 'glitchagahnim';
            }
        }
        if (canEnterNorthWestDarkWorld('majorGlitches', false, false)
                && chests[60].isAvailable().majorGlitches === 'available'
                && (trackerData.items.mirror
                        || (glitchedLinkInDarkWorld() && canLiftDarkRocks())
                        || (trackerData.items.boots && glitchedLinkInDarkWorld() && canEnterNorthEastDarkWorld('majorGlitches', false, false)))) {
            availability.majorGlitches = 'available';
        }
        else if (canEnterNorthWestDarkWorld('majorGlitches', true, false)
                && (chests[60].isAvailable().majorGlitches === 'available' || chests[60].isAvailable().majorGlitches === 'agahnim')
                && (trackerData.items.mirror
                        || (glitchedLinkInDarkWorld() && canLiftDarkRocks())
                        || (trackerData.items.boots && glitchedLinkInDarkWorld() && canEnterNorthEastDarkWorld('majorGlitches', true, false)))) {
            availability.majorGlitches = 'agahnim';
        }
        else if (canEnterNorthWestDarkWorld('majorGlitches', true, true)
                && (chests[60].isAvailable().majorGlitches === 'available' || chests[60].isAvailable().majorGlitches === 'agahnim' || chests[60].isAvailable().majorGlitches === 'glitchagahnim')
                && (trackerData.items.mirror
                        || (glitchedLinkInDarkWorld() && canLiftDarkRocks())
                        || (trackerData.items.boots && glitchedLinkInDarkWorld() && canEnterNorthEastDarkWorld('majorGlitches', true, true)))) {
            availability.majorGlitches = 'glitchagahnim';
        }
        if (canLiftDarkRocks() || trackerData.items.mirror) {
            if (canEnterLightWorld('inverted', false, false)) {
                availability.inverted = "available";
            }
            else if (canEnterLightWorld('inverted', true, false)) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
        return availability;
    }
};

chests[29] = {
    name: "Fugitive under the bridge <img src='/images/flippers.png' class='mini'>",
    x: "35.4%",
    y: "69.7%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (trackerData.items.flippers) {
            availability.glitchless = 'available';
        }
        else {
            availability.glitchless = 'glitchavailable';
            availability.owGlitches = 'available';
        }
        if (trackerData.items.moonpearl) {
            if (canEnterLightWorld('inverted', false, false)) {
                if (trackerData.items.flippers) {
                    availability.inverted = "available";
                }
                else {
                    availability.inverted = "glitchavailable";
                }
            }
            else if (canEnterLightWorld('inverted', true, false) && trackerData.items.flippers) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
        return availability;
    }
};

chests[30] = {
    name: "Ether Tablet <img src='/images/sword2.png' class='mini'><img src='/images/book.png' class='mini'>",
    x: "21.0%",
    y: "3.0%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (trackerData.items.book && (trackerData.items.mirror || (trackerData.items.hammer && trackerData.items.hookshot))) {
            if (canEnterWestDeathMountain('glitchless', false)) {
                if (trackerData.items.sword >= 2) {
                    availability.glitchless = 'available';
                }
                else {
                    availability.glitchless = 'possible';
                }
            }
            else if (canEnterWestDeathMountain('glitchless', true)) {
                if (trackerData.items.sword >= 2) {
                    availability.glitchless = 'glitchavailable';
                }
                else {
                    availability.glitchless = 'glitchpossible';
                }
            }
        }
        if (trackerData.items.book) {
            if (canEnterWestDeathMountain('owGlitches', false) && dungeons[2].canEnter('owGlitches', false, false)) {
                if (trackerData.items.sword >= 2) {
                    availability.owGlitches = 'available';
                }
                else {
                    availability.owGlitches = 'possible';
                }
            }
            else if (canEnterWestDeathMountain('owGlitches', true) && dungeons[2].canEnter('owGlitches', false, true)) {
                if (trackerData.items.sword >= 2) {
                    availability.owGlitches = 'glitchavailable';
                }
                else {
                    availability.owGlitches = 'glitchpossible';
                }
            }
        }
        if (trackerData.items.book) {
            if (canEnterWestDeathMountain('majorGlitches', false) && dungeons[2].canEnter('majorGlitches', false, false)) {
                if (trackerData.items.sword >= 2) {
                    availability.majorGlitches = 'available';
                }
                else {
                    availability.majorGlitches = 'possible';
                }
            }
            else if (canEnterWestDeathMountain('majorGlitches', false) && dungeons[2].mayEnter('majorGlitches', false, false)) {
                availability.majorGlitches = 'possible';
            }
            else if (canEnterWestDeathMountain('majorGlitches', true) && dungeons[2].canEnter('majorGlitches', false, true)) {
                if (trackerData.items.sword >= 2) {
                    availability.majorGlitches = 'glitchavailable';
                }
                else {
                    availability.majorGlitches = 'glitchpossible';
                }
            }
            else if (canEnterWestDeathMountain('majorGlitches', true) && dungeons[2].mayEnter('majorGlitches', false, true)) {
                availability.majorGlitches = 'glitchpossible';
            }
            else if (canEnterWestDeathMountain('majorGlitches', false) && dungeons[2].mayEnter('majorGlitches', true, false) && trackerData.items.sword >= 2) {
                availability.majorGlitches = 'agahnim';
            }
            else if (canEnterWestDeathMountain('majorGlitches', true) && dungeons[2].mayEnter('majorGlitches', true, true) && trackerData.items.sword >= 2) {
                availability.majorGlitches = 'glitchagahnim';
            }
        }
        if (trackerData.items.book && trackerData.items.hammer && trackerData.items.moonpearl) {
            if (canEnterEastDeathMountain('inverted', false)) {
                if (trackerData.items.sword >= 2) {
                    availability.inverted = 'available';
                }
                else {
                    availability.inverted = 'possible';
                }
            }
            else if (canEnterEastDeathMountain('inverted', true)) {
                if (trackerData.items.sword >= 2) {
                    availability.inverted = 'glitchavailable';
                }
                else {
                    availability.inverted = 'glitchpossible';
                }
            }
        }
        return availability;
    }
};

chests[31] = {
    name: "Bombos Tablet <img src='/images/mirror.png' class='mini'><img src='/images/sword2.png' class='mini'><img src='/images/book.png' class='mini'>",
    x: "11.0%",
    y: "92.2%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (trackerData.items.book && trackerData.items.mirror && canEnterSouthDarkWorld('glitchless', false, false)) {
            if (trackerData.items.sword >= 2) {
                availability.glitchless = 'available';
            }
            else {
                availability.glitchless = 'possible';
            }
        }
        else if (trackerData.items.book && trackerData.items.mirror && trackerData.items.sword >= 2) {
            if (canEnterSouthDarkWorld('glitchless', true, false)) {
                availability.glitchless = 'agahnim';
            }
            else if (canEnterSouthDarkWorld('glitchless', true, true)) {
                availability.glitchless = 'glitchagahnim';
            }
        }
        if (trackerData.items.book && (trackerData.items.boots || (trackerData.items.mirror && canEnterSouthDarkWorld('owGlitches', false, false)))) {
            if (trackerData.items.sword >= 2) {
                availability.owGlitches = 'available';
            }
            else {
                availability.owGlitches = 'possible';
            }
        }
        else if (trackerData.items.book && trackerData.items.mirror && trackerData.items.sword >= 2) {
            if (canEnterSouthDarkWorld('owGlitches', true, false)) {
                availability.owGlitches = 'agahnim';
            }
            else if (canEnterSouthDarkWorld('owGlitches', true, true)) {
                availability.owGlitches = 'glitchagahnim';
            }
        }
        if (trackerData.items.book
                && (trackerData.items.boots || (trackerData.items.mirror && canEnterSouthDarkWorld('majorGlitches', false, false)))) {
            if (trackerData.items.sword >= 2) {
                availability.majorGlitches = 'available';
            }
            else {
                availability.majorGlitches = 'possible';
            }
        }
        else if (trackerData.items.book && trackerData.items.mirror && trackerData.items.sword >= 2) {
            if (canEnterSouthDarkWorld('majorGlitches', true, false)) {
                availability.majorGlitches = 'agahnim';
            }
            else if (canEnterSouthDarkWorld('majorGlitches', true, true)) {
                availability.majorGlitches = 'glitchagahnim';
            }
        }
        if (trackerData.items.book) {
            if (canEnterLightWorld('inverted', false, false)) {
                if (trackerData.items.sword >= 2) {
                    availability.inverted = "available";
                }
                else {
                    availability.inverted = "possible";
                }
            }
            else if (canEnterLightWorld('inverted', true, false)) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
        return availability;
    }
};

chests[32] = {
    name: "Catfish",
    x: "96.0%",
    y: "17.2%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (trackerData.items.moonpearl && canLiftRocks()) {
            if (canEnterNorthEastDarkWorld('glitchless', false, false)) {
                availability.glitchless = 'available';
            }
            else if (canEnterNorthEastDarkWorld('glitchless', true, false)) {
                availability.glitchless = 'agahnim';
            }
            else if (canEnterNorthEastDarkWorld('glitchless', true, true)) {
                availability.glitchless = 'glitchagahnim';
            }
        }
        if (trackerData.items.moonpearl
                && (canLiftRocks() || trackerData.items.boots)) {
            if (canEnterNorthEastDarkWorld('owGlitches', false, false)) {
                availability.owGlitches = 'available';
            }
            else if (canEnterNorthEastDarkWorld('owGlitches', true, false)) {
                availability.owGlitches = 'agahnim';
            }
            else if (canEnterNorthEastDarkWorld('owGlitches', true, true)) {
                availability.owGlitches = 'glitchagahnim';
            }
        }
        if (glitchedLinkInDarkWorld()
                && (canLiftRocks() || trackerData.items.boots)) {
            if (canEnterNorthEastDarkWorld('majorGlitches', false, false)) {
                availability.majorGlitches = 'available';
            }
            else if (canEnterNorthEastDarkWorld('majorGlitches', true, false)) {
                availability.majorGlitches = 'agahnim';
            }
            else if (canEnterNorthEastDarkWorld('majorGlitches', true, true)) {
                availability.majorGlitches = 'glitchagahnim';
            }
        }
        if (canLiftRocks()) {
            if (canEnterNorthEastDarkWorld('inverted', false, false)) {
                availability.inverted = 'available';
            }
            else if (canEnterNorthEastDarkWorld('inverted', false, true)) {
                availability.inverted = 'glitchavailable';
            }
            else if (canEnterNorthEastDarkWorld('inverted', true, false)) {
                availability.inverted = 'agahnim';
            }
            else if (canEnterNorthEastDarkWorld('inverted', true, true)) {
                availability.inverted = 'glitchagahnim';
            }
        }
        return availability;
    }
};

chests[33] = {
    name: "King Zora: Pay 500 rupees",
    x: "47.5%",
    y: "12.1%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (trackerData.items.flippers || canLiftRocks()) {
            availability.glitchless = 'available';
        }
        else {
            availability.glitchless = 'glitchavailable';
            availability.owGlitches = 'available';
        }
        if (trackerData.items.moonpearl) {
            if (canEnterLightWorld('inverted', false, false)) {
                if (trackerData.items.flippers || canLiftRocks()) {
                    availability.inverted = "available";
                }
                else {
                    availability.inverted = "glitchavailable";
                }
            }
            else if (canEnterLightWorld('inverted', true, false) && (trackerData.items.flippers || canLiftRocks())) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
        return availability;
    }
};

chests[34] = {
    name: "Lost Old Man",
    x: "20.8%",
    y: "20.4%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canEnterWestDeathMountain('glitchless', true)) {
            if (trackerData.items.lantern) {
                availability.glitchless = 'available';
            }
            else {
                availability.glitchless = 'glitchavailable';
            }
        }
        else if (canEnterWestDeathMountain('owGlitches', true)) {
            if (trackerData.items.lantern) {
                availability.owGlitches = 'available';
            }
            else {
                availability.owGlitches = 'glitchavailable';
            }
        }
        else if (canEnterWestDeathMountain('majorGlitches', true)) {
            if (trackerData.items.lantern) {
                availability.majorGlitches = 'available';
            }
            else {
                availability.majorGlitches = 'glitchavailable';
            }
        }
        if (canEnterDarkWorldDeathMountain('inverted', false)) {
            availability.inverted = 'available';
        }
        else if (canEnterDarkWorldDeathMountain('inverted', true)) {
            availability.inverted = 'glitchavailable';
        }
        return availability;
    }
};

chests[35] = {
    name: "Witch: Give her <img src='/images/mushroom.png' class='mini'>",
    x: "40.8%",
    y: "32.5%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (trackerData.items.mushroom) {
            availability.glitchless = 'available';
            if (trackerData.items.moonpearl) {
                if (canEnterLightWorld('inverted', false, false)) {
                    availability.inverted = "available";
                }
                else if (canEnterLightWorld('inverted', true, false)) {
                    availability.inverted = "agahnim";
                }
                else if (canEnterLightWorld('inverted', true, true)) {
                    availability.inverted = "glitchagahnim";
                }
            }
        }
        return availability;
    }
};

chests[36] = {
    name: "Forest Hideout",
    x: "9.4%",
    y: "13.0%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = 'available';
        if (canEnterLightWorld('inverted', false, false)) {
            if (trackerData.items.moonpearl) {
                availability.inverted = "available";
            }
            else {
                availability.inverted = "possible";
            }
        }
        else if (canEnterLightWorld('inverted', true, false)) {
            availability.inverted = "agahnim";
        }
        else if (canEnterLightWorld('inverted', true, true)) {
            availability.inverted = "glitchagahnim";
        }
        return availability;
    }
};

chests[37] = {
    name: "Lumberjack Tree <img src='/images/agahnim1.png' class='mini'><img src='/images/boots.png' class='mini'>",
    x: "15.1%",
    y: "7.6%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = 'possible';
        if (trackerData.items.boots) {
            if (trackerData.items.agahnim) {
                availability.glitchless = 'available';
            }
            else if (canGoBeatAgahnim1(false)) {
                availability.glitchless = 'agahnim';
            }
            else if (canGoBeatAgahnim1(true)) {
                availability.glitchless = 'glitchagahnim';
            }
        }
        if (canEnterLightWorld('inverted', false, false)) {
            availability.inverted = 'possible';
            if (trackerData.items.boots && trackerData.items.moonpearl) {
                if (trackerData.items.agahnim) {
                    availability.inverted = 'available';
                }
                else if (canGoBeatAgahnim1(false, 'inverted')) {
                    availability.inverted = 'agahnim';
                }
                else if (canGoBeatAgahnim1(true, 'inverted')) {
                    availability.inverted = 'glitchagahnim';
                }
            }
        }
        else if (canEnterLightWorld('inverted', true, false)) {
            availability.inverted = 'agahnim';
        }
        else if (canEnterLightWorld('inverted', true, true)) {
            availability.inverted = 'glitchagahnim';
        }
        return availability;
    }
};

chests[38] = {
    name: "Spectacle Rock Cave",
    x: "24.3%",
    y: "14.8%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canEnterWestDeathMountain("glitchless", false)) {
            availability.glitchless = "available";
        }
        else if (canEnterWestDeathMountain("glitchless", true)) {
            availability.glitchless = "glitchavailable";
        }
        if (canEnterWestDeathMountain("owGlitches", false)) {
            availability.owGlitches = "available";
        }
        else if (canEnterWestDeathMountain("owGlitches", true)) {
            availability.owGlitches = "glitchavailable";
        }
        if (canEnterWestDeathMountain("majorGlitches", false)) {
            availability.majorGlitches = "available";
        }
        else if (canEnterWestDeathMountain("majorGlitches", true)) {
            availability.majorGlitches = "glitchavailable";
        }
        if (canEnterWestDeathMountain("inverted", false)) {
            availability.inverted = "available";
        }
        else if (canEnterWestDeathMountain("inverted", true)) {
            availability.inverted = "glitchavailable";
        }
        return availability;
    }
};

chests[39] = {
    name: "South of Grove <img src='/images/mirror.png' class='mini'>",
    x: "14.1%",
    y: "84.1%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (trackerData.items.mirror) {
            if (canEnterSouthDarkWorld('glitchless', false, false)) {
                availability.glitchless = 'available';
            }
            else if (canEnterSouthDarkWorld('glitchless', true, false)) {
                availability.glitchless = 'agahnim';
            }
            else if (canEnterSouthDarkWorld('glitchless', true, true)) {
                availability.glitchless = 'glitchagahnim';
            }
        }
        if (trackerData.items.boots) {
            availability.owGlitches = 'available';
        }
        else {
            if (trackerData.items.mirror) {
                if (canEnterSouthDarkWorld('owGlitches', false, false)) {
                    availability.owGlitches = 'available';
                }
                else if (canEnterSouthDarkWorld('owGlitches', true, false)) {
                    availability.owGlitches = 'agahnim';
                }
                else if (canEnterSouthDarkWorld('owGlitches', true, true)) {
                    availability.owGlitches = 'glitchagahnim';
                }
                if (canEnterSouthDarkWorld('majorGlitches', false, false)) {
                    availability.majorGlitches = 'available';
                }
                else if (canEnterSouthDarkWorld('majorGlitches', true, false)) {
                    availability.majorGlitches = 'agahnim';
                }
                else if (canEnterSouthDarkWorld('majorGlitches', true, true)) {
                    availability.majorGlitches = 'glitchagahnim';
                }
            }
        }
        if (canEnterLightWorld('inverted', false, false)) {
            if (trackerData.items.moonpearl) {
                availability.inverted = "available";
            }
            else {
                availability.inverted = "glitchavailable";
            }
        }
        else if (canEnterLightWorld('inverted', true, false) && trackerData.items.moonpearl) {
            availability.inverted = "agahnim";
        }
        else if (canEnterLightWorld('inverted', true, true)) {
            availability.inverted = "glitchagahnim";
        }
        return availability;
    }
};

chests[40] = {
    name: "Graveyard Cliff Cave <img src='/images/mirror.png' class='mini'>",
    x: "28.1%",
    y: "27.0%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (trackerData.items.mirror && trackerData.items.moonpearl) {
            if (canEnterNorthWestDarkWorld('glitchless', false, false)) {
                availability.glitchless = 'available';
            }
            else if (canEnterNorthWestDarkWorld('glitchless', true, false)) {
                availability.glitchless = 'agahnim';
            }
            else if (canEnterNorthWestDarkWorld('glitchless', true, true)) {
                availability.glitchless = 'glitchagahnim';
            }
        }
        if (trackerData.items.boots) {
            availability.owGlitches = 'available';
        }
        else {
            if (trackerData.items.mirror && trackerData.items.moonpearl) {
                if (canEnterNorthWestDarkWorld('owGlitches', false, false)) {
                    availability.owGlitches = 'available';
                }
                else if (canEnterNorthWestDarkWorld('owGlitches', true, false)) {
                    availability.owGlitches = 'agahnim';
                }
                else if (canEnterNorthWestDarkWorld('owGlitches', true, true)) {
                    availability.owGlitches = 'glitchagahnim';
                }
            }
            if (trackerData.items.mirror && glitchedLinkInDarkWorld()) {
                availability.majorGlitches = 'available';
            }
        }
        if (trackerData.items.moonpearl) {
            if (canEnterLightWorld('inverted', false, false)) {
                availability.inverted = "available";
            }
            else if (canEnterLightWorld('inverted', true, false)) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
        return availability;
    }
};

chests[41] = {
    name: "Checkerboard Cave <img src='/images/mirror.png' class='mini'>",
    x: "8.8%",
    y: "77.3%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canFly() && canLiftDarkRocks() && trackerData.items.mirror) {
            availability.glitchless = 'available';
        }
        if (canLiftRocks()) {
            if (trackerData.items.boots) {
                availability.owGlitches = 'available';
            }
            else if (trackerData.items.mirror) {
                if (canEnterMireArea('owGlitches', false, false)) {
                    availability.owGlitches = 'available';
                }
                else if (canEnterMireArea('owGlitches', true, false)) {
                    availability.owGlitches = 'agahnim';
                }
                else if (canEnterMireArea('owGlitches', true, true)) {
                    availability.owGlitches = 'glitchagahnim';
                }
                if (canEnterMireArea('majorGlitches', false, false)) {
                    availability.majorGlitches = 'available';
                }
                else if (canEnterMireArea('majorGlitches', true, false)) {
                    availability.majorGlitches = 'agahnim';
                }
                else if (canEnterMireArea('majorGlitches', true, true)) {
                    availability.majorGlitches = 'glitchagahnim';
                }
            }
            if (trackerData.items.moonpearl) {
                if (canEnterLightWorld('inverted', false, false)) {
                    availability.inverted = "available";
                }
                else if (canEnterLightWorld('inverted', true, false)) {
                    availability.inverted = "agahnim";
                }
                else if (canEnterLightWorld('inverted', true, true)) {
                    availability.inverted = "glitchagahnim";
                }
            }
        }
        return availability;
    }
};

chests[42] = {
    name: "<img src='/images/hammer.png' class='mini'><img src='/images/hammer.png' class='mini'><img src='/images/hammer.png' class='mini'><img src='/images/hammer.png' class='mini'><img src='/images/hammer.png' class='mini'><img src='/images/hammer.png' class='mini'><img src='/images/hammer.png' class='mini'><img src='/images/hammer.png' class='mini'>!!!!!!!!",
    x: "65.8%",
    y: "60.1%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canLiftDarkRocks() && trackerData.items.hammer) {
            if (canEnterNorthWestDarkWorld('glitchless', false, false)) {
                availability.glitchless = 'available';
            }
            else if (canEnterNorthWestDarkWorld('glitchless', true, false)) {
                availability.glitchless = 'agahnim';
            }
            else if (canEnterNorthWestDarkWorld('glitchless', true, true)) {
                availability.glitchless = 'glitchagahnim';
            }
        }
        if (trackerData.items.hammer && trackerData.items.moonpearl) {
            if (canEnterNorthWestDarkWorld('owGlitches', false, false)
                    && (canLiftDarkRocks() || (trackerData.items.boots && canEnterNorthEastDarkWorld('owGlitches', false, false)))) {
                availability.owGlitches = 'available';
            }
            else if (canEnterNorthWestDarkWorld('owGlitches', true, false)
                    && (canLiftDarkRocks() || (trackerData.items.boots && canEnterNorthEastDarkWorld('owGlitches', true, false)))) {
                availability.owGlitches = 'agahnim';
            }
            else if (canEnterNorthWestDarkWorld('owGlitches', true, true)
                    && (canLiftDarkRocks() || (trackerData.items.boots && canEnterNorthEastDarkWorld('owGlitches', true, true)))) {
                availability.owGlitches = 'glitchagahnim';
            }
        }
        if (trackerData.items.hammer && glitchedLinkInDarkWorld()) {
            if (canEnterNorthWestDarkWorld('majorGlitches', false, false)) {
                availability.majorGlitches = 'available';
            }
            else if (canEnterNorthWestDarkWorld('majorGlitches', true, false)) {
                availability.majorGlitches = 'agahnim';
            }
            else if (canEnterNorthWestDarkWorld('majorGlitches', true, true)) {
                availability.majorGlitches = 'glitchagahnim';
            }
        }
        if (trackerData.items.hammer) {
            if (canLiftDarkRocks()) {
                availability.inverted = "available";
            }
            else if (trackerData.items.mirror) {
                if (canEnterLightWorld('inverted', false, false)) {
                    availability.inverted = "available";
                }
                else if (canEnterLightWorld('inverted', true, false)) {
                    availability.inverted = "agahnim";
                }
                else if (canEnterLightWorld('inverted', true, true)) {
                    availability.inverted = "glitchagahnim";
                }
            }
        }
        return availability;
    }
};

chests[43] = {
    name: "Library <img src='/images/boots.png' class='mini'>",
    x: "7.7%",
    y: "65.9%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (trackerData.items.boots) {
            availability.glitchless = 'available';
        }
        else {
            availability.glitchless = 'possible';
        }
        if (trackerData.items.moonpearl) {
            if (canEnterLightWorld('inverted', false, false)) {
                if (trackerData.items.boots) {
                    availability.inverted = "available";
                }
                else {
                    availability.inverted = 'possible';
                }
            }
            else if (canEnterLightWorld('inverted', true, false)) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
        return availability;
    }
};

chests[44] = {
    name: "Mushroom",
    x: "6.2%",
    y: "8.6%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = 'available';
        if (canEnterLightWorld('inverted', false, false)) {
            if (trackerData.items.moonpearl) {
                availability.inverted = "available";
            }
            else {
                availability.inverted = "possible";
            }
        }
        else if (canEnterLightWorld('inverted', true, false)) {
            availability.inverted = "agahnim";
        }
        else if (canEnterLightWorld('inverted', true, true)) {
            availability.inverted = "glitchagahnim";
        }
        return availability;
    }
};

chests[45] = {
    name: "Spectacle Rock <img src='/images/mirror.png' class='mini'>",
    x: "25.4%",
    y: "8.5%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canEnterWestDeathMountain('glitchless', false)) {
            if (trackerData.items.mirror) {
                availability.glitchless = 'available';
            }
            else {
                availability.glitchless = 'possible';
            }
        }
        else if (canEnterWestDeathMountain('glitchless', true)) {
            if (trackerData.items.mirror) {
                availability.glitchless = 'glitchavailable';
            }
            else {
                availability.glitchless = 'glitchpossible';
            }
        }
        if (canEnterWestDeathMountain('owGlitches', false)) {
            if (trackerData.items.boots || trackerData.items.mirror) {
                availability.owGlitches = 'available';
            }
            else {
                availability.owGlitches = 'possible';
            }
        }
        else if (canEnterWestDeathMountain('owGlitches', true)) {
            if (trackerData.items.boots || trackerData.items.mirror) {
                availability.owGlitches = 'glitchavailable';
            }
            else {
                availability.owGlitches = 'glitchpossible';
            }
        }
        if (canEnterWestDeathMountain('majorGlitches', false)) {
            if (trackerData.items.boots || trackerData.items.mirror) {
                availability.majorGlitches = 'available';
            }
            else {
                availability.majorGlitches = 'possible';
            }
        }
        else if (canEnterWestDeathMountain('majorGlitches', true)) {
            if (trackerData.items.boots || trackerData.items.mirror) {
                availability.majorGlitches = 'glitchavailable';
            }
            else {
                availability.majorGlitches = 'glitchpossible';
            }
        }
        if (canEnterEastDeathMountain('inverted', false) && trackerData.items.hammer && trackerData.items.moonpearl) {
                availability.inverted = 'available';
        }
        else if (canEnterEastDeathMountain('inverted', true) && trackerData.items.hammer && trackerData.items.moonpearl) {
                availability.inverted = 'glitchavailable';
        }
        else if (canEnterWestDeathMountain('inverted', false)) {
                availability.inverted = 'possible';
        }
        else if (canEnterWestDeathMountain('inverted', true)) {
                availability.inverted = 'glitchpossible';
        }
        return availability;
    }
};

chests[46] = {
    name: "Floating Island <img src='/images/mirror.png' class='mini'>",
    x: "40.2%",
    y: "3.0%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canEnterEastDeathMountain('glitchless', false)) {
            if (trackerData.items.mirror
                    && trackerData.items.moonpearl
                    && canLiftDarkRocks()) {
                availability.glitchless = 'available';
            }
            else {
                availability.glitchless = 'possible';
            }
        }
        else if (canEnterEastDeathMountain('glitchless', true)) {
            if (trackerData.items.mirror
                    && trackerData.items.moonpearl
                    && canLiftDarkRocks()) {
                availability.glitchless = 'glitchavailable';
            }
            else {
                availability.glitchless = 'glitchpossible';
            }
        }
        if (canEnterEastDeathMountain('owGlitches', false)) {
            if ((trackerData.items.boots
                            || (trackerData.items.mirror
                                    && trackerData.items.moonpearl
                                    && canLiftRocks()
                                    && canEnterDarkWorldDeathMountain('owGlitches', false)))) {
                availability.owGlitches = 'available';
            }
            else {
                availability.owGlitches = 'possible';
            }
        }
        else if (canEnterEastDeathMountain('owGlitches', true)) {
            if ((trackerData.items.boots
                            || (trackerData.items.mirror
                                    && trackerData.items.moonpearl
                                    && canLiftRocks()
                                    && canEnterDarkWorldDeathMountain('owGlitches', true)))) {
                availability.owGlitches = 'glitchavailable';
            }
            else {
                availability.owGlitches = 'glitchpossible';
            }
        }
        if (canEnterEastDeathMountain('majorGlitches', false)) {
            if ((trackerData.items.boots
                            || (trackerData.items.mirror
                                    && glitchedLinkInDarkWorld()
                                    && canLiftRocks()
                                    && canEnterDarkWorldDeathMountain('majorGlitches', false)))) {
                availability.majorGlitches = 'available';
            }
            else {
                availability.majorGlitches = 'possible';
            }
        }
        else if (canEnterEastDeathMountain('majorGlitches', true)) {
            if ((trackerData.items.boots
                            || (trackerData.items.mirror
                                    && glitchedLinkInDarkWorld()
                                    && canLiftRocks()
                                    && canEnterDarkWorldDeathMountain('majorGlitches', true)))) {
                availability.majorGlitches = 'glitchavailable';
            }
            else {
                availability.majorGlitches = 'glitchpossible';
            }
        }
        if (canEnterEastDeathMountain('inverted', false)) {
                availability.inverted = 'available';
        }
        else if (canEnterEastDeathMountain('inverted', true)) {
                availability.inverted = 'glitchavailable';
        }
        return availability;
    }
};

chests[47] = {
    name: "Race Minigame <img src='/images/bomb.png' class='mini'>/<img src='/images/boots.png' class='mini'>",
    x: "1.8%",
    y: "69.8%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = 'available';
        if (trackerData.items.moonpearl) {
            if (canEnterLightWorld('inverted', false, false)) {
                availability.inverted = "available";
            }
            else if (canEnterLightWorld('inverted', true, false)) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
        return availability;
    }
};

chests[48] = {
    name: "Desert West Ledge <img src='/images/book.png' class='mini'>/<img src='/images/mirror.png' class='mini'>",
    x: "1.5%",
    y: "91.0%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = 'possible';
        if (dungeons[1].canEnter('glitchless', false, false)) {
            availability.glitchless = 'available';
        }
        else {
            if (dungeons[1].canEnter('owGlitches', false, false)) {
                availability.owGlitches = 'available';
            }
            else if (dungeons[1].canEnter('owGlitches', true, false)) {
                availability.owGlitches = 'agahnim';
            }
            else if (dungeons[1].canEnter('owGlitches', true, true)) {
                availability.owGlitches = 'glitchagahnim';
            }
            if (dungeons[1].canEnter('majorGlitches', false, false)) {
                availability.majorGlitches = 'available';
            }
            else if (dungeons[1].canEnter('majorGlitches', true, false)) {
                availability.majorGlitches = 'agahnim';
            }
            else if (dungeons[1].canEnter('majorGlitches', true, true)) {
                availability.majorGlitches = 'glitchagahnim';
            }
        }
        if (canEnterLightWorld('inverted', false, false)) {
            availability.inverted = 'possible';
            if (dungeons[1].canEnter('inverted', false, false)) {
                if (trackerData.items.moonpearl) {
                    availability.inverted = "available";
                }
                else {
                    availability.inverted = "glitchavailable";
                }
            }
        }
        else if (canEnterLightWorld('inverted', true, false) && trackerData.items.moonpearl) {
            availability.inverted = "agahnim";
        }
        else if (canEnterLightWorld('inverted', true, true)) {
            availability.inverted = "glitchagahnim";
        }
        return availability;
    }
};

chests[49] = {
    name: "Lake Hylia Island <img src='/images/mirror.png' class='mini'>",
    x: "36.1%",
    y: "82.9%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = 'possible';
        if (trackerData.items.flippers && trackerData.items.moonpearl && trackerData.items.mirror) {
            if (canEnterSouthDarkWorld('glitchless', false, false) || canEnterNorthEastDarkWorld('glitchless', false, false)) {
                availability.glitchless = 'available';
            }
            else if (canEnterSouthDarkWorld('glitchless', true, false) || canEnterNorthEastDarkWorld('glitchless', true, false)) {
                availability.glitchless = 'agahnim';
            }
            else if (canEnterSouthDarkWorld('glitchless', true, true) || canEnterNorthEastDarkWorld('glitchless', true, true)) {
                availability.glitchless = 'glitchagahnim';
            }
        }
        if (trackerData.items.boots) {
            availability.owGlitches = 'available';
        }
        else {
            if (trackerData.items.flippers && trackerData.items.mirror) {
                if ((trackerData.items.moonpearl && canEnterSouthDarkWorld('owGlitches', false, false))
                        || canEnterNorthEastDarkWorld('owGlitches', false, false)) {
                    availability.owGlitches = 'available';
                }
                else if ((trackerData.items.moonpearl && canEnterSouthDarkWorld('owGlitches', true, false))
                        || canEnterNorthEastDarkWorld('owGlitches', true, false)) {
                    availability.owGlitches = 'agahnim';
                }
                else if ((trackerData.items.moonpearl && canEnterSouthDarkWorld('owGlitches', true, true))
                        || canEnterNorthEastDarkWorld('owGlitches', true, true)) {
                    availability.owGlitches = 'glitchagahnim';
                }
                if (glitchedLinkInDarkWorld() || canEnterNorthEastDarkWorld('majorGlitches', false, false)) {
                    availability.majorGlitches = 'available';
                }
                else if (glitchedLinkInDarkWorld() || canEnterNorthEastDarkWorld('majorGlitches', true, false)) {
                    availability.majorGlitches = 'agahnim';
                }
                else if (glitchedLinkInDarkWorld() || canEnterNorthEastDarkWorld('majorGlitches', true, true)) {
                    availability.majorGlitches = 'glitchagahnim';
                }
            }
        }
        if (trackerData.items.moonpearl) {
            if (canEnterLightWorld('inverted', false, false)) {
                if (trackerData.items.flippers) {
                    availability.inverted = "available";
                }
                else {
                    availability.inverted = "glitchavailable";
                }
            }
            else if (canEnterLightWorld('inverted', true, false) && trackerData.items.flippers) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
        return availability;
    }
};

chests[50] = {
    name: "Bumper Cave <img src='/images/cape.png' class='mini'>",
    x: "67.1%",
    y: "15.2%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canEnterNorthWestDarkWorld('glitchless', false, false)) {
            if (canLiftRocks() && trackerData.items.cape) {
                availability.glitchless = 'available';
            }
            else {
                availability.glitchless = 'possible';
            }
        }
        else if (canEnterNorthWestDarkWorld('glitchless', true, false) && canLiftRocks() && trackerData.items.cape) {
            availability.glitchless = 'agahnim';
        }
        else if (canEnterNorthWestDarkWorld('glitchless', true, true) && canLiftRocks() && trackerData.items.cape) {
            availability.glitchless = 'glitchagahnim';
        }
        if (canEnterNorthWestDarkWorld('owGlitches', false, false)) {
            if (trackerData.items.moonpearl && (trackerData.items.boots || (canLiftRocks() && trackerData.items.cape))) {
                availability.owGlitches = 'available';
            }
            else {
                availability.owGlitches = 'possible';
            }
        }
        else if (trackerData.items.moonpearl && (trackerData.items.boots || (canLiftRocks() && trackerData.items.cape))) {
            if (canEnterNorthWestDarkWorld('owGlitches', true, false)) {
                availability.owGlitches = 'agahnim';
            }
            else if (canEnterNorthWestDarkWorld('owGlitches', true, true)) {
                availability.owGlitches = 'glitchagahnim';
            }
        }
        if (canEnterNorthWestDarkWorld('majorGlitches', false, false)) {
            if (glitchedLinkInDarkWorld() && (trackerData.items.boots || (canLiftRocks() && trackerData.items.cape))) {
                availability.majorGlitches = 'available';
            }
            else {
                availability.majorGlitches = 'possible';
            }
        }
        else if (glitchedLinkInDarkWorld() && (trackerData.items.boots || (canLiftRocks() && trackerData.items.cape))) {
            if (canEnterNorthWestDarkWorld('majorGlitches', true, false)) {
                availability.majorGlitches = 'agahnim';
            }
            else if (canEnterNorthWestDarkWorld('majorGlitches', true, true)) {
                availability.majorGlitches = 'glitchagahnim';
            }
        }
        availability.inverted = 'possible';
        if (trackerData.items.moonpearl && canLiftRocks() && trackerData.items.cape && trackerData.items.mirror) {
            if (canEnterLightWorld('inverted', false, false)) {
                availability.inverted = "available";
            }
            else if (canEnterLightWorld('inverted', true, false)) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
        return availability;
    }
};

chests[51] = {
    name: "Pyramid",
    x: "79.0%",
    y: "43.5%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canEnterNorthEastDarkWorld('glitchless', false, false)) {
            availability.glitchless = 'available';
        }
        else if (canEnterNorthEastDarkWorld('glitchless', true, false)) {
            availability.glitchless = 'agahnim';
        }
        else if (canEnterNorthEastDarkWorld('glitchless', true, true)) {
            availability.glitchless = 'glitchagahnim';
        }
        if (canEnterNorthEastDarkWorld('owGlitches', false, false)) {
            availability.owGlitches = 'available';
        }
        else if (canEnterNorthEastDarkWorld('owGlitches', true, false)) {
            availability.owGlitches = 'agahnim';
        }
        else if (canEnterNorthEastDarkWorld('owGlitches', true, true)) {
            availability.owGlitches = 'glitchagahnim';
        }
        if (canEnterNorthEastDarkWorld('majorGlitches', false, false)) {
            availability.majorGlitches = 'available';
        }
        else if (canEnterNorthEastDarkWorld('majorGlitches', true, false)) {
            availability.majorGlitches = 'agahnim';
        }
        else if (canEnterNorthEastDarkWorld('majorGlitches', true, true)) {
            availability.majorGlitches = 'glitchagahnim';
        }
        if (canEnterNorthEastDarkWorld('inverted', false, false)) {
            availability.inverted = 'available';
        }
        else if (canEnterNorthEastDarkWorld('inverted', false, true)) {
            availability.inverted = 'glitchavailable';
        }
        else if (canEnterNorthEastDarkWorld('inverted', true, false)) {
            availability.inverted = 'agahnim';
        }
        else if (canEnterNorthEastDarkWorld('inverted', true, true)) {
            availability.inverted = 'glitchagahnim';
        }
        return availability;
    }
};

chests[52] = {
    name: "Dig Game: Pay 80 rupees",
    x: "52.9%",
    y: "69.2%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canEnterSouthDarkWorld("glitchless", false, false)) {
            availability.glitchless = "available";
        }
        else if (canEnterSouthDarkWorld("glitchless", true, false)) {
            availability.glitchless = "agahnim";
        }
        else if (canEnterSouthDarkWorld("glitchless", true, true)) {
            availability.glitchless = "glitchagahnim";
        }
        if (trackerData.items.moonpearl) {
            if (canEnterSouthDarkWorld("owGlitches", false, false)) {
                availability.owGlitches = "available";
            }
            else if (canEnterSouthDarkWorld("owGlitches", true, false)) {
                availability.owGlitches = "agahnim";
            }
            else if (canEnterSouthDarkWorld("owGlitches", true, true)) {
                availability.owGlitches = "glitchagahnim";
            }
        }
        if (glitchedLinkInDarkWorld()) {
            if (canEnterSouthDarkWorld("majorGlitches", false, false)) {
                availability.majorGlitches = "available";
            }
            else if (canEnterSouthDarkWorld("majorGlitches", true, false)) {
                availability.majorGlitches = "agahnim";
            }
            else if (canEnterSouthDarkWorld("majorGlitches", true, true)) {
                availability.majorGlitches = "glitchagahnim";
            }
        }
        availability.inverted = 'available';
        return availability;
    }
};

chests[53] = {
    name: "Zora River Ledge <img src='/images/flippers.png' class='mini'>",
    x: "47.5%",
    y: "17.3%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (trackerData.items.flippers) {
            availability.glitchless = 'available';
        }
        else if (canLiftRocks()) {
            availability.glitchless = 'possible';
        }
        else {
            availability.glitchless = 'glitchpossible';
        }
        if (trackerData.items.boots && trackerData.items.moonpearl) {
            availability.owGlitches = 'available';
        }
        else {
            availability.owGlitches = 'possible';
        }
        if (trackerData.items.moonpearl) {
            if (canEnterLightWorld('inverted', false, false)) {
                if (trackerData.items.flippers) {
                    availability.inverted = "available";
                }
                else if (canLiftRocks()) {
                    availability.inverted = "possible";
                }
                else {
                    availability.inverted = "glitchpossible";
                }
            }
            else if (canEnterLightWorld('inverted', true, false) && (trackerData.items.flippers || canLiftRocks())) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
        return availability;
    }
};

chests[54] = {
    name: "Buried Item <img src='/images/shovel.png' class='mini'>",
    x: "14.4%",
    y: "66.2%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (trackerData.items.shovel) {
            availability.glitchless = 'available';
            if (trackerData.items.moonpearl) {
                if (canEnterLightWorld('inverted', false, false)) {
                    availability.inverted = "available";
                }
                else if (canEnterLightWorld('inverted', true, false)) {
                    availability.inverted = "agahnim";
                }
                else if (canEnterLightWorld('inverted', true, true)) {
                    availability.inverted = "glitchagahnim";
                }
            }
        }
        return availability;
    }
};

chests[55] = {
    name: "Escape Sewer (4) <img src='/images/bomb.png' class='mini'>/<img src='/images/boots.png' class='mini'>",
    x: "26.8%",
    y: "32.4%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = 'available';
        if (canEnterLightWorld('inverted', false, false)) {
            if (trackerData.items.moonpearl) {
                availability.inverted = "available";
            }
            else {
                availability.inverted = "glitchpossible";
            }
        }
        else if (canEnterLightWorld('inverted', true, false) && trackerData.items.moonpearl) {
            availability.inverted = "agahnim";
        }
        else if (canEnterLightWorld('inverted', true, true)) {
            availability.inverted = "glitchagahnim";
        }
        return availability;
    }
};

chests[56] = {
    name: "Castle Secret Entrance (2)",
    x: "29.8%",
    y: "41.8%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = 'available';
        if (trackerData.items.moonpearl) {
            if (canEnterLightWorld('inverted', false, false)) {
                availability.inverted = "available";
            }
            else if (canEnterLightWorld('inverted', true, false)) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
        return availability;
    }
};

chests[57] = {
    name: "Hyrule Castle (3)",
    x: "24.9%",
    y: "44.1%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = 'available';
        if (canEnterLightWorld('inverted', false, false)) {
            if (trackerData.items.moonpearl) {
                availability.inverted = "available";
            }
            else {
                availability.inverted = "glitchavailable";
            }
        }
        else if (canEnterLightWorld('inverted', true, false) && trackerData.items.moonpearl) {
            availability.inverted = "agahnim";
        }
        else if (canEnterLightWorld('inverted', true, true)) {
            availability.inverted = "glitchagahnim";
        }
        return availability;
    }
};

chests[58] = {
    name: "Sanctuary",
    x: "23.0%",
    y: "28.0%",
    isOpened: true,
    isAvailable: function () {
        const availability = new Availability();
        availability.glitchless = 'available';
        if (canEnterLightWorld('inverted', false, false)) {
            if (trackerData.items.moonpearl) {
                availability.inverted = "available";
            }
            else if (trackerData.items.mirror) {
                availability.inverted = "glitchavailable";
            }
            else {
                availability.inverted = "glitchpossible";
            }
        }
        else if (canEnterLightWorld('inverted', true, false) && trackerData.items.moonpearl) {
            availability.inverted = "agahnim";
        }
        else if (canEnterLightWorld('inverted', true, true)) {
            availability.inverted = "glitchagahnim";
        }
        return availability;
    }
};

chests[59] = {
    name: "Mad Batter <img src='/images/hammer.png' class='mini'>/<img src='/images/mirror.png' class='mini'> + <img src='/images/powder.png' class='mini'>",
    x: "16.0%",
    y: "58.0%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (trackerData.items.hammer
                || (trackerData.items.moonpearl && trackerData.items.mirror && canLiftDarkRocks())) {
            if (trackerData.items.powder) {
                availability.glitchless = 'available';
            }
            else if (trackerData.items.somaria && trackerData.items.mushroom) {
                availability.glitchless = 'glitchavailable';
            }
        }
        if (trackerData.items.powder && trackerData.items.boots) {
            availability.owGlitches = 'available';
        }
        else if (trackerData.items.powder && trackerData.items.mirror) {
            availability.majorGlitches = 'available';
        }
        if (trackerData.items.moonpearl && trackerData.items.hammer) {
            if (trackerData.items.powder) {
                if (canEnterLightWorld('inverted', false, false)) {
                    availability.inverted = "available";
                }
                else if (canEnterLightWorld('inverted', true, false)) {
                    availability.inverted = "agahnim";
                }
                else if (canEnterLightWorld('inverted', true, true)) {
                    availability.inverted = "glitchagahnim";
                }
            }
            else if (trackerData.items.somaria && trackerData.items.mushroom) {
                if (canEnterLightWorld('inverted', false, false)) {
                    availability.inverted = "glitchavailable";
                }
                else if (canEnterLightWorld('inverted', true, true)) {
                    availability.inverted = "glitchagahnim";
                }
            }
        }
        return availability;
    }
};

chests[60] = {
    name: "Take the frog home (<img src='/images/mirror.png' class='mini'> or save and quit)",
    x: "15.2%",
    y: "51.8%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (canLiftDarkRocks()) {
            if (canEnterNorthWestDarkWorld('glitchless', false, false)) {
                availability.glitchless = 'available';
            }
            else if (canEnterNorthWestDarkWorld('glitchless', true, false)) {
                availability.glitchless = 'agahnim';
            }
            else if (canEnterNorthWestDarkWorld('glitchless', true, true)) {
                availability.glitchless = 'glitchagahnim';
            }
        }
        if (trackerData.items.moonpearl && (canLiftDarkRocks() || (trackerData.items.boots && trackerData.items.mirror))) {
            if (canEnterNorthWestDarkWorld('owGlitches', false, false)) {
                availability.owGlitches = 'available';
            }
            else if (canEnterNorthWestDarkWorld('owGlitches', true, false)) {
                availability.owGlitches = 'agahnim';
            }
            else if (canEnterNorthWestDarkWorld('owGlitches', true, true)) {
                availability.owGlitches = 'glitchagahnim';
            }
        }
        if (glitchedLinkInDarkWorld() && (canLiftDarkRocks() || (trackerData.items.boots && trackerData.items.mirror))) {
            if (canEnterNorthWestDarkWorld('majorGlitches', false, false)) {
                availability.majorGlitches = 'available';
            }
            else if (canEnterNorthWestDarkWorld('majorGlitches', true, false)) {
                availability.majorGlitches = 'agahnim';
            }
            else if (canEnterNorthWestDarkWorld('majorGlitches', true, true)) {
                availability.majorGlitches = 'glitchagahnim';
            }
        }
        if (canLiftDarkRocks() || trackerData.items.mirror) {
            if (canEnterLightWorld('inverted', false, false)) {
                availability.inverted = "available";
            }
            else if (canEnterLightWorld('inverted', true, false)) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
        return availability;
    }
};

chests[61] = {
    name: "Fat Fairy: Buy OJ bomb from Dark Link's House after <img src='/images/crystal0.png' class='mini'>5 <img src='/images/crystal0.png' class='mini'>6 (2 items)",
    x: "73.5%",
    y: "48.5%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        // Crystal check
        let crystalCount = 0;
        for (let k = 0; k < 10; k++) {
            if (trackerData.prizes[k] === 4 && trackerData.items["boss" + k] === 2) {
                crystalCount++;
                if (crystalCount === 2) {
                    break;
                }
            }
        }
        if (crystalCount === 2 && trackerData.items.moonpearl) {
            if (canEnterSouthDarkWorld('glitchless', false, false)
                    && (trackerData.items.hammer || (trackerData.items.mirror && trackerData.items.agahnim))) {
                availability.glitchless = 'available';
            }
            else if (canEnterSouthDarkWorld('glitchless', true, false)
                    && (trackerData.items.hammer || (trackerData.items.mirror && canGoBeatAgahnim1(false)))) {
                availability.glitchless = 'agahnim';
            }
            else if (canEnterSouthDarkWorld('glitchless', true, true)
                    && (trackerData.items.hammer || (trackerData.items.mirror && canGoBeatAgahnim1(true)))) {
                availability.glitchless = 'glitchagahnim';
            }
        }
        if (trackerData.items.mirror && canSpinSpeed()) {
            availability.owGlitches = 'available';
        }
        else if (crystalCount === 2) {
            if (canEnterSouthDarkWorld('owGlitches', false, false)
                    && ((trackerData.items.hammer && trackerData.items.moonpearl) || (trackerData.items.mirror && trackerData.items.agahnim))) {
                availability.owGlitches = 'available'
            }
            else if (canEnterSouthDarkWorld('owGlitches', true, false)
                    && ((trackerData.items.hammer && trackerData.items.moonpearl) || (trackerData.items.mirror && canGoBeatAgahnim1(false)))) {
                availability.owGlitches = 'agahnim';
            }
            else if (canEnterSouthDarkWorld('owGlitches', true, true)
                    && ((trackerData.items.hammer && trackerData.items.moonpearl) || (trackerData.items.mirror && canGoBeatAgahnim1(true)))) {
                availability.owGlitches = 'glitchagahnim';
            }
            if (canEnterSouthDarkWorld('majorGlitches', false, false)
                    && ((trackerData.items.hammer && glitchedLinkInDarkWorld()) || (trackerData.items.mirror && trackerData.items.agahnim))) {
                availability.majorGlitches = 'available'
            }
            else if (canEnterSouthDarkWorld('majorGlitches', true, false)
                    && ((trackerData.items.hammer && glitchedLinkInDarkWorld()) || (trackerData.items.mirror && canGoBeatAgahnim1(false)))) {
                availability.majorGlitches = 'agahnim';
            }
            else if (canEnterSouthDarkWorld('majorGlitches', true, true)
                    && ((trackerData.items.hammer && glitchedLinkInDarkWorld()) || (trackerData.items.mirror && canGoBeatAgahnim1(true)))) {
                availability.majorGlitches = 'glitchagahnim';
            }
        }
        if (crystalCount === 2 && trackerData.items.mirror) {
            if (canEnterLightWorld('inverted', false, false)) {
                availability.inverted = "available";
            }
            else if (canEnterLightWorld('inverted', true, false)) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
        return availability;
    }
};

chests[62] = {
    name: "Master Sword Pedestal <img src='/images/pendant0.png' class='mini'><img src='/images/pendant1.png' class='mini'><img src='/images/pendant2.png' class='mini'> (can check with <img src='/images/book.png' class='mini'>)",
    x: "2.5%",
    y: "3.2%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        let pendantCount = 0;
        for (let k = 0; k < 10; k++) {
            if ((trackerData.prizes[k] === 1 || trackerData.prizes[k] === 2) && trackerData.items["boss" + k] === 2) {
                pendantCount++;
                if (pendantCount === 3) {
                    break;
                }
            }
        }
        if (pendantCount === 3) {
            availability.glitchless = 'available';
        }
        else if (trackerData.items.book) {
            availability.glitchless = 'possible';
        }
        if (canEnterLightWorld('inverted', false, false)) {
            if (pendantCount === 3) {
                availability.inverted = "available";
            }
            else if (trackerData.items.book) {
                availability.inverted = "possible";
            }
        }
        else if (pendantCount === 3 || trackerData.items.book) {
            if (canEnterLightWorld('inverted', true, false)) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
            
        return availability;
    }
};

chests[63] = {
    name: "Waterfall of the Wishing (2)  <img src='/images/flippers.png' class='mini'>",
    x: "44.9%",
    y: "14.7%",
    isOpened: false,
    isAvailable: function () {
        const availability = new Availability();
        if (trackerData.items.flippers) {
            availability.glitchless = 'available';
        }
        else if (trackerData.items.moonpearl) {
            availability.glitchless = 'glitchavailable';
            availability.owGlitches = 'available';
        }
        else if (trackerData.items.boots) {
            availability.glitchless = 'glitchavailable';
        }
        if (trackerData.items.moonpearl) {
            if (canEnterLightWorld('inverted', false, false)) {
                if (trackerData.items.flippers) {
                    availability.inverted = "available";
                }
                else {
                    availability.inverted = "glitchavailable";
                }
            }
            else if (canEnterLightWorld('inverted', true, false) && trackerData.items.flippers) {
                availability.inverted = "agahnim";
            }
            else if (canEnterLightWorld('inverted', true, true)) {
                availability.inverted = "glitchagahnim";
            }
        }
        return availability;
    }
};

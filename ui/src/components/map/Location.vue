<template>
  <span
    class="mapspan chest"
    :class="locationAvailability()"
    :style="locationStyle()"
    @click="toggleLocation"></span>
</template>

<script>
export default {
  name: 'Location',
  props: {
    location: Object
  },
  computed: {
    locationLeft() {
      return this.location.position.horizontal.left
    },
    locationTop() {
      return this.location.position.horizontal.top
    }
  },
  methods: {
    locationStyle() {
      return (
        'background-image: url("/static/image/poi.png"); left: ' +
        this.locationLeft +
        '%; top: ' +
        this.locationTop +
        '%;'
      )
    },
    locationAvailability() {
      if (this.location.cleared) {
        return 'opened'
      }

      return 'available'
    },
    toggleLocation(event) {
      event.stopPropagation()

      let cleared = !this.location.cleared
      let data = { cleared: cleared }
      this.updateLocationState(data)
    },
    updateLocationState(data) {
      let host = window.location.hostname + ':' + this.$store.state.serverConfig.apiPort

      var xhr = new XMLHttpRequest()
      xhr.open('POST', 'http://' + host + '/location_state/' + this.name, true)
      xhr.setRequestHeader('Content-Type', 'application/json')
      xhr.send(JSON.stringify(data))
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.chest {
  width: 24px;
  height: 24px;
  background-size: 100% 100%;
  position: absolute;
  margin-left: -12px;
  margin-top: -12px;
}

.opened {
  background-color: rgb(127, 127, 127);
}

.available {
  background-color: rgb(0, 255, 0);
  color: black;
}

.possible {
  background-color: rgb(255, 255, 0);
  color: black;
}

.unavailable {
  background-color: rgb(255, 0, 0);
}

.glitchavailable {
  background-color: rgb(0,127,0);
}

.glitchpossible {
  background-color: rgb(192,192,0);
}

.agahnim {
  background-color: rgb(0,255,255);
  color: black;
}

.glitchagahnim {
  background-color: rgb(0,160,160);
}
</style>

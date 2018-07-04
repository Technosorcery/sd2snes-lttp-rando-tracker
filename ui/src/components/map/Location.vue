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
    name: String,
    left: Number,
    top: Number,
    hoverText: String
  },
  computed: {
    cleared() {
      if (this.$store.state.locations[this.name]) {
        return this.$store.state.locations[this.name].cleared
      }

      return false
    }
  },
  methods: {
    locationStyle() {
      return (
        'background-image: url("/static/image/poi.png"); left: ' +
        this.left +
        '%; top: ' +
        this.top +
        '%;'
      )
    },
    locationAvailability() {
      if (this.cleared) {
        return 'opened'
      }

      return 'available'
    },
    toggleLocation(event) {
      event.stopPropagation()

      let cleared = !this.cleared
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
}

.possible {
  background-color: rgb(255, 255, 0);
}

.unavailable {
  background-color: rgb(255, 0, 0);
}
</style>

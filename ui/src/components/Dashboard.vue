<template>
  <div class="dashboard">
    <h1>{{ msg }}</h1>
    <item-tracker></item-tracker>
    <map-tracker></map-tracker>
  </div>
</template>

<script>
import ItemTracker from '@/components/ItemTracker'
import MapTracker from '@/components/MapTracker'

export default {
  name: 'Dashboard',
  components: {
    ItemTracker,
    MapTracker
  },
  data () {
    return {
      msg: 'The Dasboard'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h1, h2 {
  font-weight: normal;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>

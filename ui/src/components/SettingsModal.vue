<script lang="ts">
export default {
  name: "SettingsModal",
}
</script>
<script setup lang="ts">
import { computed, ref } from 'vue'
import { ErrorMessage, Field, Form } from 'vee-validate'
import { useStore } from "../store"
import type { ServerConfigUpdate } from '../server_types/ServerConfigUpdate'

const store = useStore()

const deviceList = computed(() => store.getters.knownDevices)
const dataPollRate = ref(store.state.serverConfig.dataPollRate)
const sourceType = ref(store.state.serverConfig.sourceType)
const selectedDevice = ref(
  store.state.serverConfig.sourceType === "QUsb2snes" ? store.state.serverConfig.dataSource : ''
)
const filePath = ref(
  store.state.serverConfig.sourceType === "LocalFile" ? store.state.serverConfig.dataSource : ''
)
const logicSet = ref(store.state.serverConfig.logic)

function onSubmit(values: any) {
  window.console.log("Form submitted:", values)

  var config_update: ServerConfigUpdate = {
    dataPollRate: values.dataPollRate,
    sourceType: values.sourceType,
    dataSource: (values.sourceType === "LocalFile" ? values.source : values.device),
    logic: values.logic,
  }
  window.console.log("Sending update:", config_update)

  let host = window.location.hostname + ':' + store?.state?.serverConfig.apiPort
  var xhr = new XMLHttpRequest()
  xhr.open('POST', 'http://' + host + '/api/config', true)
  xhr.setRequestHeader('Content-Type', 'application/json')
  xhr.send(JSON.stringify(config_update))
}

function validateDataPollRate(value: any) {
  if (!value) {
    return 'Polling interval is required'
  }

  if (value <= 0) {
    return 'Polling interval must be greater than zero'
  }

  return true
}

function validateFilePath(value: any) {
  if (!value) {
    return 'File path is required'
  }

  return true
}
</script>

<template>
  <transition name="modal">
    <div class="fixed z-[9998] top-0 left-0 w-full h-full bg-black/90 table transition-opacity duration-300 ease-in-out"> <!-- Modal mask -->
      <div class="table-cell align-middle"> <!-- Modal wrapper -->
        <div class="m-auto p-20 min-w-min w-4/5 bg-slate-800 rounded-lg shadow-inner shadow-black transition-all duration-300 ease-in-out"> <!-- Modal container -->
          <Form @submit="onSubmit">
            <div class="modal-header"> <!-- Modal header -->
              <h3 class="text-2xl mt-0 text-green-600">Settings</h3>
            </div>
            <div class="mx-20 my-0"> <!-- Modal body -->
              <div class="block m-2">
                <div>
                  <div class="inline-block">
                    <label for="logic" class="mx-2">Game logic rules:</label>
                    <Field name="logic" id="logic" class="text-black" as="select" v-bind:value="logicSet">
                      <option value="glitchless">Glitchless</option>
                      <option value="overWorldGlitches">Overworld Glitches</option>
                      <option value="majorGlitches">Major Glitches</option>
                    </Field>
                  </div>
                </div>
                <ErrorMessage name="sourceType" class="block" />
              </div>
              <div class="block m-2">
                <div class="inline-block">
                  <label for="data-poll-rate" class="mx-2">Game state polling interval (ms):</label>
                  <Field type="number" class="text-black" name="dataPollRate" id="data-poll-rate" :rules="validateDataPollRate" v-model="dataPollRate" />
                </div>
                <ErrorMessage name="dataPollRate" class="block" />
              </div>
              <div class="block m-2">
                <div>
                  <label for="source-type" class="mx-2">Game data source:</label>
                  <div class="inline-block">
                    <div class="block">
                      <Field name="sourceType" type="radio" id="local-file" v-model="sourceType" value="LocalFile" />
                      <label for="local-file">Local File</label>
                    </div>
                    <div class="block">
                      <Field name="sourceType" type="radio" id="qusb2snes" v-model="sourceType" value="QUsb2snes" />
                      <label for="qusb2snes">QUsb2snes</label>
                    </div>
                  </div>
                </div>
                <ErrorMessage name="sourceType" class="block" />
              </div>
              <div class="block m-2" v-if="sourceType === 'QUsb2snes'">
                <div class="inline-block">
                  <label for="device" class="mx-2">QUsb2snes device:</label>
                  <Field name="device" id="devices" class="text-black" as="select" v-bind:value="selectedDevice">
                    <option value="">None</option>
                    <option v-for="device in deviceList" :value="device">{{device}}</option>
                  </Field>
                </div>
                <ErrorMessage name="device" class="block" />
              </div>
              <div class="block m-2" v-if="sourceType === 'LocalFile'">
                <div class="inline-block">
                  <label for="source" class="mx-2">File path:</label>
                  <Field name="source" v-model="filePath" class="text-black" id="file-path" :rules="validateFilePath" />
                </div>
                <ErrorMessage name="source" class="block" />
              </div>
            </div>
            <div class="modal-footer"> <!-- Modal footer -->
              <button class="p-1 mx-2 border-4 rounded-lg">Save</button>
              <button class="p-1 mx-2 border-4 rounded-lg float-right" @click="$emit('close')">Close</button>
            </div>
          </Form>
        </div>
      </div>
    </div>
  </transition>
</template>

<style scoped>
/*
 * The following styles are auto-applied to elements with
 * transition="modal" when their visibility is toggled
 * by Vue.js.
 *
 * You can easily play with the modal transition by editing
 * these styles.
 */

.modal-enter {
  opacity: 0;
}

.modal-leave-active {
  opacity: 0;
}

.modal-enter .modal-container,
.modal-leave-active .modal-container {
  -webkit-transform: scale(1.1);
  transform: scale(1.1);
}
</style>

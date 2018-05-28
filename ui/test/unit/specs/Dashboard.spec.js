import Vue from 'vue'
import Dashboard from '@/components/Dashboard'

describe('Dashboard.vue', () => {
  it('should render correct contents', () => {
    const Constructor = Vue.extend(Dashboard)
    const vm = new Constructor().$mount()
    expect(vm.$el.querySelector('.dashboard h1').textContent)
      .toEqual('The Dasboard')
  })
})

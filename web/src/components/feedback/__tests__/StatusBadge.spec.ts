import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import StatusBadge from '../StatusBadge.vue'

describe('StatusBadge', () => {
  it('renders the status label', () => {
    const wrapper = mount(StatusBadge, { props: { status: 'open' } })
    expect(wrapper.text()).toBe('Open')
  })

  it('applies green classes for done status', () => {
    const wrapper = mount(StatusBadge, { props: { status: 'done' } })
    expect(wrapper.find('span').classes()).toContain('bg-green-100')
  })

  it('applies blue classes for planned status', () => {
    const wrapper = mount(StatusBadge, { props: { status: 'planned' } })
    expect(wrapper.find('span').classes()).toContain('bg-blue-100')
  })

  it('applies yellow classes for in_progress status', () => {
    const wrapper = mount(StatusBadge, { props: { status: 'in_progress' } })
    expect(wrapper.find('span').classes()).toContain('bg-yellow-100')
  })

  it('applies red classes for closed status', () => {
    const wrapper = mount(StatusBadge, { props: { status: 'closed' } })
    expect(wrapper.find('span').classes()).toContain('bg-red-100')
  })

  it('applies gray classes for open status', () => {
    const wrapper = mount(StatusBadge, { props: { status: 'open' } })
    expect(wrapper.find('span').classes()).toContain('bg-gray-100')
  })
})

import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import TagBadge from '../TagBadge.vue'

describe('TagBadge', () => {
  it('renders tag name', () => {
    const wrapper = mount(TagBadge, {
      props: { tag: { id: '1', name: 'UX', color: '#8b5cf6' } },
    })
    expect(wrapper.text()).toBe('UX')
  })

  it('applies tag color as background', () => {
    const wrapper = mount(TagBadge, {
      props: { tag: { id: '1', name: 'Backend', color: '#3b82f6' } },
    })
    const style = wrapper.find('span').attributes('style')
    expect(style).toContain('background-color')
  })
})

import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import VoteButton from '../VoteButton.vue'

describe('VoteButton', () => {
  it('displays vote count', () => {
    const wrapper = mount(VoteButton, {
      props: { voteCount: 42, hasVoted: false },
    })
    expect(wrapper.text()).toContain('42')
  })

  it('emits vote event on click', async () => {
    const wrapper = mount(VoteButton, {
      props: { voteCount: 0, hasVoted: false },
    })
    await wrapper.find('button').trigger('click')
    expect(wrapper.emitted('vote')).toHaveLength(1)
  })

  it('applies primary color classes when voted', () => {
    const wrapper = mount(VoteButton, {
      props: { voteCount: 1, hasVoted: true },
    })
    const button = wrapper.find('button')
    expect(button.classes().some((c) => c.includes('primary'))).toBe(true)
  })

  it('applies gray classes when not voted', () => {
    const wrapper = mount(VoteButton, {
      props: { voteCount: 0, hasVoted: false },
    })
    const button = wrapper.find('button')
    expect(button.classes().some((c) => c.includes('gray'))).toBe(true)
  })
})

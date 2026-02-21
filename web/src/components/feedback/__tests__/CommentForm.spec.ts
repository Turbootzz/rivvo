import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import CommentForm from '../CommentForm.vue'

describe('CommentForm', () => {
  it('emits submit with body text', async () => {
    const wrapper = mount(CommentForm)

    await wrapper.find('textarea').setValue('Great idea!')
    await wrapper.find('form').trigger('submit')

    expect(wrapper.emitted('submit')).toHaveLength(1)
    expect(wrapper.emitted('submit')![0]).toEqual(['Great idea!'])
  })

  it('does not emit with empty body', async () => {
    const wrapper = mount(CommentForm)
    await wrapper.find('form').trigger('submit')
    expect(wrapper.emitted('submit')).toBeUndefined()
  })

  it('clears textarea after submit', async () => {
    const wrapper = mount(CommentForm)

    await wrapper.find('textarea').setValue('Reply')
    await wrapper.find('form').trigger('submit')

    expect((wrapper.find('textarea').element as HTMLTextAreaElement).value).toBe('')
  })
})

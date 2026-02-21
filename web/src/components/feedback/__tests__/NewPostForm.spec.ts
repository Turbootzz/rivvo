import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import NewPostForm from '../NewPostForm.vue'

describe('NewPostForm', () => {
  it('emits submit with title and description', async () => {
    const wrapper = mount(NewPostForm)

    await wrapper.find('input').setValue('Dark mode')
    await wrapper.find('textarea').setValue('Please add dark mode')
    await wrapper.find('form').trigger('submit')

    expect(wrapper.emitted('submit')).toHaveLength(1)
    expect(wrapper.emitted('submit')![0]).toEqual(['Dark mode', 'Please add dark mode'])
  })

  it('does not emit submit with empty title', async () => {
    const wrapper = mount(NewPostForm)

    await wrapper.find('form').trigger('submit')

    expect(wrapper.emitted('submit')).toBeUndefined()
  })

  it('emits cancel on cancel button click', async () => {
    const wrapper = mount(NewPostForm)

    const cancelBtn = wrapper.findAll('button').find((b) => b.text() === 'Cancel')
    await cancelBtn!.trigger('click')

    expect(wrapper.emitted('cancel')).toHaveLength(1)
  })

  it('clears form after submit', async () => {
    const wrapper = mount(NewPostForm)

    await wrapper.find('input').setValue('Title')
    await wrapper.find('textarea').setValue('Desc')
    await wrapper.find('form').trigger('submit')

    expect((wrapper.find('input').element as HTMLInputElement).value).toBe('')
    expect((wrapper.find('textarea').element as HTMLTextAreaElement).value).toBe('')
  })
})

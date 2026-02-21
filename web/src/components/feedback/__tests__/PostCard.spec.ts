import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import PostCard from '../PostCard.vue'
import type { PostListItem } from '@/types'

const post: PostListItem = {
  id: 'p1',
  title: 'Dark mode support',
  description_preview: 'We need dark mode for night owls',
  status: 'planned',
  vote_count: 12,
  comment_count: 3,
  pinned: false,
  author_name: 'Alice',
  has_voted: false,
  tags: [{ id: 't1', name: 'UX', color: '#8b5cf6' }],
  created_at: '2025-01-01',
}

describe('PostCard', () => {
  it('renders post title', () => {
    const wrapper = mount(PostCard, { props: { post } })
    expect(wrapper.text()).toContain('Dark mode support')
  })

  it('renders vote count', () => {
    const wrapper = mount(PostCard, { props: { post } })
    expect(wrapper.text()).toContain('12')
  })

  it('renders comment count', () => {
    const wrapper = mount(PostCard, { props: { post } })
    expect(wrapper.text()).toContain('3')
  })

  it('renders description preview', () => {
    const wrapper = mount(PostCard, { props: { post } })
    expect(wrapper.text()).toContain('We need dark mode for night owls')
  })

  it('renders tags', () => {
    const wrapper = mount(PostCard, { props: { post } })
    expect(wrapper.text()).toContain('UX')
  })

  it('emits click with post id on card click', async () => {
    const wrapper = mount(PostCard, { props: { post } })
    await wrapper.find('.cursor-pointer, [class*="cursor"]').trigger('click')
    const emitted = wrapper.emitted('click')
    expect(emitted).toBeTruthy()
    expect(emitted![0]).toEqual(['p1'])
  })

  it('emits vote with post id', async () => {
    const wrapper = mount(PostCard, { props: { post } })
    const voteButton = wrapper.find('button')
    expect(voteButton.exists()).toBe(true)
    await voteButton.trigger('click')
    const emitted = wrapper.emitted('vote')
    expect(emitted).toBeTruthy()
    expect(emitted![0]).toEqual(['p1'])
  })
})

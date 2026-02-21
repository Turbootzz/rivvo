import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { setActivePinia, createPinia } from 'pinia'
import CommentItem from '../CommentItem.vue'
import type { Comment } from '@/types'

// Mock auth store
vi.mock('@/stores/auth', () => ({
  useAuthStore: () => ({
    user: { id: 'user-1', email: 'a@b.com', name: 'Alice', avatar_url: null, created_at: '' },
    token: 'tok',
    isAuthenticated: true,
  }),
}))

const comment: Comment = {
  id: 'c1',
  body: 'Nice feature!',
  is_admin_reply: false,
  author: { id: 'user-1', name: 'Alice', avatar_url: null },
  created_at: '2025-01-15T12:00:00Z',
}

describe('CommentItem', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('renders comment body', () => {
    const wrapper = mount(CommentItem, { props: { comment } })
    expect(wrapper.text()).toContain('Nice feature!')
  })

  it('renders author name', () => {
    const wrapper = mount(CommentItem, { props: { comment } })
    expect(wrapper.text()).toContain('Alice')
  })

  it('shows delete button for comment owner', () => {
    const wrapper = mount(CommentItem, { props: { comment } })
    const deleteBtn = wrapper.find('button')
    expect(deleteBtn.exists()).toBe(true)
  })

  it('hides delete button for non-owner', () => {
    const otherComment: Comment = {
      ...comment,
      author: { id: 'user-other', name: 'Bob', avatar_url: null },
    }
    const wrapper = mount(CommentItem, { props: { comment: otherComment } })
    expect(wrapper.find('button').exists()).toBe(false)
  })

  it('emits delete with comment id', async () => {
    const wrapper = mount(CommentItem, { props: { comment } })
    await wrapper.find('button').trigger('click')
    expect(wrapper.emitted('delete')).toEqual([['c1']])
  })

  it('shows Admin badge for admin replies', () => {
    const adminComment: Comment = { ...comment, is_admin_reply: true }
    const wrapper = mount(CommentItem, { props: { comment: adminComment } })
    expect(wrapper.text()).toContain('Admin')
  })

  it('shows fallback name for deleted user', () => {
    const noAuthor: Comment = { ...comment, author: null }
    const wrapper = mount(CommentItem, { props: { comment: noAuthor } })
    expect(wrapper.text()).toContain('Deleted user')
  })
})
